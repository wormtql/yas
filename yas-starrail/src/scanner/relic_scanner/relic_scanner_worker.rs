use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use anyhow::Result;
use image::{GenericImageView, RgbImage};
use log::{error, info, warn};

use yas::ocr::{ImageToText, yas_ocr_model};
use yas::positioning::{Pos, Rect};

use crate::scanner::relic_scanner::message_items::SendItem;
use crate::scanner::relic_scanner::relic_scanner_window_info::RelicScannerWindowInfo;
use crate::scanner::relic_scanner::scan_result::StarRailRelicScanResult;
use crate::scanner::relic_scanner::StarRailRelicScannerConfig;

pub struct RelicScannerWorker {
    model: Box<dyn ImageToText<RgbImage> + Send>,
    window_info: RelicScannerWindowInfo,
    config: StarRailRelicScannerConfig,
}

fn parse_level(s: &str) -> Result<i32> {
    let pos = s.find('+');

    if pos.is_none() {
        let level = s.parse::<i32>()?;
        return Ok(level);
    }

    let level = s[pos.unwrap()..].parse::<i32>()?;
    return Ok(level);
}

fn get_image_to_text() -> Result<Box<dyn ImageToText<RgbImage> + Send>> {
    let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(
        yas_ocr_model!("./models/model_training.onnx", "./models/index_2_word.json")?
    );
    Ok(model)
}

impl RelicScannerWorker {
    pub fn new(
        window_info: RelicScannerWindowInfo,
        config: StarRailRelicScannerConfig,
    ) -> Result<Self> {
        Ok(RelicScannerWorker {
            model: get_image_to_text()?,
            window_info,
            config,
        })
    }

    fn model_inference(&self, rect: Rect<f64>, captured_img: &RgbImage) -> Result<String> {
        let relative_rect = rect.translate(Pos {
            x: -self.window_info.panel_rect.left,
            y: -self.window_info.panel_rect.top,
        });

        let raw_img = captured_img.view(
            relative_rect.left as u32, relative_rect.top as u32, relative_rect.width as u32, relative_rect.height as u32
        ).to_image();

        let inference_result = self.model.image_to_text(&raw_img, false);

        inference_result
    }

    fn scan_item_image(&self, item: SendItem) -> Result<StarRailRelicScanResult> {
        let image = &item.panel_image;

        let str_title = self.model_inference(self.window_info.title_rect, &image)?;
        let str_main_stat_name = self.model_inference(self.window_info.main_stat_name_rect, &image)?;
        let str_main_stat_value = self.model_inference(self.window_info.main_stat_value_rect, &image)?;

        let str_sub_stat0_name = self.model_inference(self.window_info.sub_stat_name_1, &image)?;
        let str_sub_stat1_name = self.model_inference(self.window_info.sub_stat_name_2, &image)?;
        let str_sub_stat2_name = self.model_inference(self.window_info.sub_stat_name_3, &image)?;
        let str_sub_stat3_name = self.model_inference(self.window_info.sub_stat_name_4, &image)?;
        let str_sub_stat0_value = self.model_inference(self.window_info.sub_stat_value_1, &image)?;
        let str_sub_stat1_value = self.model_inference(self.window_info.sub_stat_value_2, &image)?;
        let str_sub_stat2_value = self.model_inference(self.window_info.sub_stat_value_3, &image)?;
        let str_sub_stat3_value = self.model_inference(self.window_info.sub_stat_value_4, &image)?;

        let str_level = self.model_inference(self.window_info.level_rect, &image)?;
        let str_equip = self.model_inference(self.window_info.equip_rect, &image)?;

        Ok(StarRailRelicScanResult {
            name: str_title,
            main_stat_name: str_main_stat_name,
            main_stat_value: str_main_stat_value,
            sub_stat_name: [
                str_sub_stat0_name,
                str_sub_stat1_name,
                str_sub_stat2_name,
                str_sub_stat3_name,
            ],
            sub_stat_value: [
                str_sub_stat0_value,
                str_sub_stat1_value,
                str_sub_stat2_value,
                str_sub_stat3_value,
            ],
            level: parse_level(&str_level)?,
            equip: item.equip + &str_equip,
            star: item.star as i32,
            lock: item.lock,
            discard: item.discard,
        })
    }

    pub fn run(self, rx: Receiver<Option<SendItem>>) -> JoinHandle<Vec<StarRailRelicScanResult>> {
        std::thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;

            let is_verbose = self.config.verbose;
            let min_level = self.config.min_level;
            let info = self.window_info.clone();

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
                    },
                };

                if is_verbose {
                    info!("{:?}", result);
                }

                if result.level < min_level {
                    info!(
                        "找到满足最低等级要求 {} 的物品({})，准备退出……",
                        min_level, result.level
                    );
                    // token.cancel();
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
                    break;
                }
            }

            info!("识别结束，非重复物品数量: {}", hash.len());

            results
        })
    }
}