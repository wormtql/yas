use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use image::{GenericImageView, RgbImage};
use yas::ocr::{ImageToText, yas_ocr_model};
use crate::scanner::echo_scanner::echo_scanner_config::WWEchoScannerConfig;
use crate::scanner::echo_scanner::echo_scanner_window_info::EchoScannerWindowInfo;
use anyhow::Result;
use log::{error, info, warn};
use rayon::iter::ParallelBridge;
use yas::positioning::{Pos, Rect};
use crate::scanner::echo_scanner::message_item::SendItem;
use crate::scanner::echo_scanner::scan_result::WWEchoScanResult;

pub struct WWEchoScannerWorker {
    model: Box<dyn ImageToText<RgbImage> + Send>,
    window_info: EchoScannerWindowInfo,
    config: WWEchoScannerConfig,
}

fn parse_level(s: &str) -> Result<usize> {
    let pos = s.find('+');

    if pos.is_none() {
        let level = s.parse::<usize>()?;
        return Ok(level);
    }

    let level = s[pos.unwrap()..].parse::<usize>()?;
    return Ok(level);
}

fn get_image_to_text() -> Result<Box<dyn ImageToText<RgbImage> + Send>> {
    let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(
        yas_ocr_model!("./models/model_training.onnx", "./models/index_2_word.json")?
    );
    // let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(PPOCRChV4RecInfer::new()?);
    Ok(model)
}

impl WWEchoScannerWorker {
    pub fn new(
        window_info: EchoScannerWindowInfo,
        config: WWEchoScannerConfig,
    ) -> Result<Self> {
        Ok(Self {
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

    fn determine_star(&self, im: &RgbImage) -> Result<usize> {
        let pos_relative_to_panel = self.window_info.star_pos - self.window_info.panel_rect.origin();

        let x = pos_relative_to_panel.x as u32;
        let y = im.height() - pos_relative_to_panel.y as u32;

        let color = im.get_pixel(x, y);
        // todo

        Ok(5)
    }

    fn parse_item(&self, item: SendItem) -> Result<WWEchoScanResult> {
        let image = &item.panel_image;

        let str_title = self.model_inference(self.window_info.title_rect, &image)?;
        let str_main_stat1_name = self.model_inference(self.window_info.main_stat1_name_rect, &image)?;
        let str_main_stat1_value = self.model_inference(self.window_info.main_stat1_value_rect, &image)?;
        let str_main_stat2_name = self.model_inference(self.window_info.main_stat2_name_rect, &image)?;
        let str_main_stat2_value = self.model_inference(self.window_info.main_stat2_value_rect, &image)?;

        let str_sub_stat0_name = self.model_inference(self.window_info.sub_stat_name_1, &image)?;
        let str_sub_stat1_name = self.model_inference(self.window_info.sub_stat_name_2, &image)?;
        let str_sub_stat2_name = self.model_inference(self.window_info.sub_stat_name_3, &image)?;
        let str_sub_stat3_name = self.model_inference(self.window_info.sub_stat_name_4, &image)?;
        let str_sub_stat4_name = self.model_inference(self.window_info.sub_stat_name_5, &image)?;
        let str_sub_stat0_value = self.model_inference(self.window_info.sub_stat_value_1, &image)?;
        let str_sub_stat1_value = self.model_inference(self.window_info.sub_stat_value_2, &image)?;
        let str_sub_stat2_value = self.model_inference(self.window_info.sub_stat_value_3, &image)?;
        let str_sub_stat3_value = self.model_inference(self.window_info.sub_stat_value_4, &image)?;
        let str_sub_stat4_value = self.model_inference(self.window_info.sub_stat_value_5, &image)?;

        let str_level = self.model_inference(self.window_info.level_rect, &image)?;
        // let str_equip = self.model_inference(self.window_info., &image)?;

        let star = self.determine_star(&image)?;

        Ok(WWEchoScanResult {
            name: str_title,
            main_stat1_name: str_main_stat1_name,
            main_stat1_value: str_main_stat1_value,
            main_stat2_name: str_main_stat2_name,
            main_stat2_value: str_main_stat2_value,
            sub_stat_names: [
                str_sub_stat0_name,
                str_sub_stat1_name,
                str_sub_stat2_name,
                str_sub_stat3_name,
                str_sub_stat4_name,
            ],
            sub_stat_values: [
                str_sub_stat0_value,
                str_sub_stat1_value,
                str_sub_stat2_value,
                str_sub_stat3_value,
                str_sub_stat4_value,
            ],
            level: parse_level(&str_level)?,
            // equip: item.equip + &str_equip,
            star,
            // lock: item.lock,
        })
    }

    pub fn run(self, rx: Receiver<SendItem>) -> JoinHandle<Vec<WWEchoScanResult>> {
        std::thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;

            let is_verbose = self.config.verbose;
            let min_level = self.config.min_level;
            let info = self.window_info.clone();

            for (_cnt, item) in rx.into_iter().enumerate() {
                let result = match self.parse_item(item) {
                    Ok(v) => v,
                    Err(e) => {
                        // error!("识别错误: {}", e);
                        continue;
                    },
                };

                if is_verbose {
                    info!("{:?}", result);
                }

                if hash.contains(&result) {
                    consecutive_dup_count += 1;
                    // warn!("识别到重复物品: {:#?}", result);
                } else {
                    consecutive_dup_count = 0;
                    hash.insert(result.clone());
                    results.push(result);
                }

                // if consecutive_dup_count >= info.col && !self.config.ignore_dup {
                //     error!("识别到连续多个重复物品，可能为翻页错误，或者为非背包顶部开始扫描");
                //     break;
                // }
            }

            info!("识别结束，非重复物品数量: {}", hash.len());

            results
        })
    }
}
