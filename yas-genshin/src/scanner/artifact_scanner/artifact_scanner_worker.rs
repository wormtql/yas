use std::collections::HashSet;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use image::{GenericImageView, ImageBuffer, Luma, RgbImage};
use yas::common::positioning::{Pos, Rect};
use yas::ocr::ImageToText;
use crate::scanner::artifact_scanner::artifact_scanner_window_info::ArtifactScannerWindowInfo;
use crate::scanner::artifact_scanner::GenshinArtifactScannerConfig;
use anyhow::{anyhow, Result};
use log::{error, info, warn};
use crate::scanner::artifact_scanner::message_items::SendItem;
use crate::scanner::artifact_scanner::scan_result::GenshinArtifactScanResult;

fn parse_level(s: &str) -> Result<i32> {
    let pos = s.find('+');

    if pos.is_none() {
        let level = s.parse::<i32>()?;
        return anyhow::Ok(level);
    }

    let level = s[pos.unwrap()..].parse::<i32>()?;
    return anyhow::Ok(level);
}

/// run in a separate thread, accept captured image and get an artifact
pub struct ArtifactScannerWorker {
    model: Rc<dyn ImageToText<RgbImage>>,
    window_info: ArtifactScannerWindowInfo,
    config: GenshinArtifactScannerConfig,
}

impl ArtifactScannerWorker {
    pub fn new(
        model: Rc<dyn ImageToText<ImageBuffer<Luma<f32>, Vec<f32>>>>,
        window_info: ArtifactScannerWindowInfo,
        config: GenshinArtifactScannerConfig,
    ) -> Self {
        ArtifactScannerWorker {
            model,
            window_info,
            config,
        }
    }

    /// the captured_img is a panel of the artifact, the rect is a region of the panel
    fn model_inference(&self, rect: Rect, captured_img: &RgbImage) -> Result<String> {
        // todo move dump mode into a scanner
        // if dump_mode {
        //     captured_img.save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        // }

        let relative_rect = rect.translate(Pos {
            x: -self.window_info.panel_rect.left,
            y: -self.window_info.panel_rect.top,
        });

        let raw_img = captured_img.view(
            relative_rect.left as u32, relative_rect.top as u32, relative_rect.width as u32, relative_rect.height as u32,
        ).to_image();
        // let raw_img_grayed = to_gray(&raw_img);

        // let raw_img = to_gray(captured_img)
        //     .view(
        //         relative_rect.left,
        //         relative_rect.top,
        //         rect.size.width,
        //         rect.size.height,
        //     )
        //     .to_image();

        // if dump_mode {
        //     raw_img
        //         .to_common_grayscale()
        //         .save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        // }

        // let (processed_img, process_flag) = pre_process(raw_img_grayed);
        // if !process_flag {
        //     return Ok(String::new());
        // }

        // if dump_mode {
        //     processed_img
        //         .to_common_grayscale()
        //         .save(Path::new("dumps").join(format!("{}_{}.pp.png", name, cnt)))?;
        // }

        // let inference_result = self.model.inference_string(&processed_img)?;

        // if dump_mode {
        //     dump_text(
        //         &inference_result,
        //         Path::new("dumps").join(format!("{}_{}.txt", name, cnt)),
        //     );
        // }

        let inference_result = self.model.image_to_text(&raw_img, false);

        inference_result
    }

    /// Parse the captured result (of type SendItem) to a scanned artifact
    fn scan_item_image(&self, item: SendItem) -> Result<GenshinArtifactScanResult> {
        let image = &item.panel_image;

        let str_title = self.model_inference(self.window_info.title_rect, &image)?;
        let str_main_stat_name = self.model_inference(self.window_info.main_stat_name_rect, &image)?;
        let str_main_stat_value = self.model_inference(self.window_info.main_stat_value_rect, &image)?;

        let str_sub_stat0 = self.model_inference(self.window_info.sub_stat_rect[0], &image)?;
        let str_sub_stat1 = self.model_inference(self.window_info.sub_stat_rect[1], &image)?;
        let str_sub_stat2 = self.model_inference(self.window_info.sub_stat_rect[2], &image)?;
        let str_sub_stat3 = self.model_inference(self.window_info.sub_stat_rect[3], &image)?;

        let str_level = self.model_inference(self.window_info.level_rect, &image)?;
        let str_equip = self.model_inference(self.window_info.item_equip_rect, &image)?;

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
        })
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

            for (_cnt, item) in rx.into_iter().enumerate() {
                let item = match item {
                    Some(v) => v,
                    None => break,
                };

                let result = match self.scan_item_image(item) {
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