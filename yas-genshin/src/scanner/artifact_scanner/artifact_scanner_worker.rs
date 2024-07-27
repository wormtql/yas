use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use anyhow::Result;
use image::{GenericImageView, RgbImage};
use log::{error, info, warn};

use yas::ocr::ImageToText;
use yas::ocr::yas_ocr_model;
use yas::positioning::{Pos, Rect};

use crate::scanner::artifact_scanner::artifact_scanner_window_info::ArtifactScannerWindowInfo;
use crate::scanner::artifact_scanner::GenshinArtifactScannerConfig;
use crate::scanner::artifact_scanner::message_items::SendItem;
use crate::scanner::artifact_scanner::scan_result::GenshinArtifactScanResult;

fn parse_level(s: &str) -> Result<i32> {
    let pos = s.find('+');

    if pos.is_none() {
        let level = s.parse::<i32>()?;
        return anyhow::Ok(level);
    }

    let level = s[pos.unwrap()..].parse::<i32>()?;
    anyhow::Ok(level)
}

fn get_image_to_text() -> Result<Box<dyn ImageToText<RgbImage> + Send>> {
    let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(
        yas_ocr_model!("./models/model_training.onnx", "./models/index_2_word.json")?
    );
    Ok(model)
}

/// run in a separate thread, accept captured image and get an artifact
pub struct ArtifactScannerWorker {
    model: Box<dyn ImageToText<RgbImage> + Send>,
    window_info: ArtifactScannerWindowInfo,
    config: GenshinArtifactScannerConfig,
}

impl ArtifactScannerWorker {
    pub fn new(
        window_info: ArtifactScannerWindowInfo,
        config: GenshinArtifactScannerConfig,
    ) -> Result<Self> {
        Ok(ArtifactScannerWorker {
            model: get_image_to_text()?,
            window_info,
            config,
        })
    }

    /// the captured_img is a panel of the artifact, the rect is a region of the panel
    fn model_inference(&self, rect: Rect<f64>, captured_img: &RgbImage) -> Result<String> {
        let relative_rect = rect.translate(Pos {
            x: -self.window_info.panel_rect.left,
            y: -self.window_info.panel_rect.top,
        });

        let raw_img = captured_img.view(
            relative_rect.left as u32, relative_rect.top as u32, relative_rect.width as u32, relative_rect.height as u32,
        ).to_image();

        let inference_result = self.model.image_to_text(&raw_img, false);

        inference_result
    }

    /// Parse the captured result (of type SendItem) to a scanned artifact
    fn scan_item_image(&self, item: SendItem, lock: bool) -> Result<GenshinArtifactScanResult> {
        let image = &item.panel_image;

        let str_title = self.model_inference(self.window_info.title_rect, image)?;
        let str_main_stat_name = self.model_inference(self.window_info.main_stat_name_rect, image)?;
        let str_main_stat_value = self.model_inference(self.window_info.main_stat_value_rect, image)?;

        let str_sub_stat0 = self.model_inference(self.window_info.sub_stat_1, image)?;
        let str_sub_stat1 = self.model_inference(self.window_info.sub_stat_2, image)?;
        let str_sub_stat2 = self.model_inference(self.window_info.sub_stat_3, image)?;
        let str_sub_stat3 = self.model_inference(self.window_info.sub_stat_4, image)?;

        let str_level = self.model_inference(self.window_info.level_rect, image)?;
        let str_equip = self.model_inference(self.window_info.item_equip_rect, image)?;

        anyhow::Ok(GenshinArtifactScanResult {
            name: str_title,
            main_stat_name: str_main_stat_name,
            main_stat_value: str_main_stat_value,
            sub_stat: [
                str_sub_stat0,
                str_sub_stat1,
                str_sub_stat2,
                str_sub_stat3,
            ],
            level: parse_level(&str_level)?,
            equip: str_equip,
            star: item.star as i32,
            lock,
        })
    }

    /// Get all lock state from a list image
    fn get_page_locks(&self, list_image: &RgbImage) -> Vec<bool> {
        let mut result = Vec::new();

        let row = self.window_info.row;
        let col = self.window_info.col;
        let gap = self.window_info.item_gap_size;
        let size = self.window_info.item_size;
        let lock_pos = self.window_info.lock_pos;

        for r in 0..row {
            if ((gap.height + size.height) * (r as f64)) as u32 > list_image.height() {
                break;
            }
            for c in 0..col {
                let pos_x = (gap.width + size.width) * (c as f64) + lock_pos.x;
                let pos_y = (gap.height + size.height) * (r as f64) + lock_pos.y;

                let mut locked = false;
                'sq: for dx in -1..1 {
                    for dy in -10..10 {
                        if pos_y as i32 + dy < 0 || (pos_y as i32 + dy) as u32 >= list_image.height() {
                            continue;
                        }

                        let color = list_image
                            .get_pixel((pos_x as i32 + dx) as u32, (pos_y as i32 + dy) as u32);

                        if color.0[0] > 200 {
                            locked = true;
                            break 'sq;
                        }
                    }
                }
                result.push(locked);
            }
        }
        result
    }

    pub fn run(self, rx: Receiver<Option<SendItem>>) -> JoinHandle<Vec<GenshinArtifactScanResult>> {
        std::thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash: HashSet<GenshinArtifactScanResult> = HashSet::new();
            // if too many artifacts are same in consecutive, then an error has occurred
            let mut consecutive_dup_count = 0;

            let is_verbose = self.config.verbose;
            let min_level = self.config.min_level;
            let info = self.window_info.clone();
            // todo remove dump mode to another scanner
            // let dump_mode = false;
            // let model = self.model.clone();
            // let panel_origin = Pos { x: self.window_info.panel_rect.left, y: self.window_info.panel_rect.top };

            let mut locks = Vec::new();
            let mut artifact_index: i32 = 0;

            for item in rx.into_iter() {
                // receiving None, which means the worker should end
                let item = match item {
                    Some(v) => v,
                    None => break,
                };

                // if there is a list image, then parse the lock state
                match item.list_image.as_ref() {
                    Some(v) => {
                        locks = vec![locks, self.get_page_locks(v)].concat()
                    }
                    None => {}
                };

                artifact_index += 1;
                let result = match self.scan_item_image(item, locks[artifact_index as usize - 1]) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("识别错误: {}", e);
                        continue;
                    }
                };

                if is_verbose {
                    info!("{:?}", result);
                }

                if result.level < min_level {
                    info!(
                        "找到满足最低等级要求 {} 的物品({})，准备退出……",
                        min_level, result.level
                    );
                    break;
                }

                if hash.contains(&result) {
                    consecutive_dup_count += 1;
                    warn!("识别到重复物品: {:#?}", result);
                } else {
                    consecutive_dup_count = 0;
                    hash.insert(result.clone());
                    results.push(result);
                }

                if consecutive_dup_count >= info.col && !self.config.ignore_dup {
                    error!("识别到连续多个重复物品，可能为翻页错误，或者为非背包顶部开始扫描");
                    // token.cancel();
                    break;
                }

                // if token.cancelled() {
                // error!("扫描任务被取消");
                // break;
                // }
            }

            info!("识别结束，非重复物品数量: {}", hash.len());

            // progress_bar.finish();
            // MULTI_PROGRESS.remove(&progress_bar);

            results
        })
    }
}
