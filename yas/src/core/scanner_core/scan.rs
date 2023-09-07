use super::inference::{pre_process, to_gray};
use super::*;
use crate::common::color::Color;
use crate::TARGET_GAME;
use anyhow::Result;
use image::*;
use pre_process::ImageConvExt;
use std::fmt::Display;
use std::fs;
use std::path::Path;

impl ScannerCore {
    #[inline(always)]
    pub fn get_flag_color(&self) -> Result<Color> {
        let target = &self.scan_info.flag + &self.scan_info.origin;
        capture::get_color(target)
    }

    #[inline(always)]
    pub fn capture_panel(&mut self) -> Result<RgbImage> {
        Rect::from(&self.scan_info.panel_pos).capture_relative(&self.scan_info.origin)
    }

    #[inline(always)]
    pub fn sample_initial_color(&mut self) {
        self.initial_color = self.get_flag_color().unwrap();
    }

    pub fn get_star(&self) -> u8 {
        let color = capture::get_color(&self.scan_info.origin + &self.scan_info.star).unwrap();

        let match_colors = [
            Color::new(113, 119, 139),
            Color::new(42, 143, 114),
            Color::new(81, 127, 203),
            Color::new(161, 86, 224),
            Color::new(188, 105, 50),
        ];

        let mut min_dis: u32 = 0xdeadbeef;
        let mut ret: usize = 1;
        for (i, match_color) in match_colors.iter().enumerate() {
            let dis = match_color.distance(&color);
            if dis < min_dis {
                min_dis = dis;
                ret = i + 1;
            }
        }

        ret as u8
    }

    pub fn get_item_count(&self) -> usize {
        let count = self.config.number;
        let item_name = match TARGET_GAME.get().unwrap() {
            Game::Genshin => "圣遗物",
            Game::StarRail => "遗器数量",
        };

        let max_count = match crate::TARGET_GAME.get().unwrap() {
            Game::Genshin => 1800,
            Game::StarRail => 1500,
        };

        if count > 0 {
            return max_count.min(count);
        }

        let im = match Rect::from(&self.scan_info.item_count_pos)
            .capture_relative(&self.scan_info.origin)
        {
            Ok(im) => im,
            Err(e) => {
                error!("Error when capturing item count: {}", e);
                return max_count;
            },
        };

        let s = match self.model.inference_string(&im) {
            Ok(s) => s,
            Err(e) => {
                error!("Error when inferring item count: {}", e);
                return max_count;
            },
        };

        info!("物品信息: {}", s);

        if s.starts_with(item_name) {
            let chars = s.chars().collect::<Vec<char>>();
            let count_str = chars[4..chars.len() - 5].iter().collect::<String>();
            match count_str.parse::<usize>() {
                Ok(v) => v.min(max_count),
                Err(_) => max_count,
            }
        } else {
            max_count
        }
    }
}

pub fn get_model_inference_func(
    dump_mode: bool,
    model: Arc<CRNNModel>,
    panel_origin: Pos,
) -> ModelInferenceFunc {
    let model_inference = move |pos: &RectBound<u32>,
                                name: &str,
                                captured_img: &RgbImage,
                                cnt: usize|
          -> Result<String> {
        if dump_mode {
            captured_img.save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        }

        let rect = &Rect::from(pos) - &panel_origin;

        let raw_img = to_gray(captured_img)
            .view(
                rect.origin.x,
                rect.origin.y,
                rect.size.width,
                rect.size.height,
            )
            .to_image();

        if dump_mode {
            raw_img
                .to_common_grayscale()
                .save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        }

        let processed_img = match pre_process(raw_img) {
            Some(im) => im,
            None => return Err(anyhow::anyhow!("图像预处理失败")),
        };

        if dump_mode {
            processed_img
                .to_common_grayscale()
                .save(Path::new("dumps").join(format!("{}_{}.pp.png", name, cnt)))?;
        }

        let inference_result = model.inference_string(&processed_img)?;

        if dump_mode {
            dump_text(
                &inference_result,
                Path::new("dumps").join(format!("{}_{}.txt", name, cnt)),
            );
        }

        Ok(inference_result)
    };

    Box::new(model_inference)
}

fn dump_text<Q, C>(contents: C, path: Q)
where
    Q: AsRef<Path>,
    C: AsRef<[u8]> + Display,
{
    if let Err(e) = fs::write(&path, &contents) {
        error!(
            "Error when dumping text to {}: {}\n{}",
            &path.as_ref().display(),
            e,
            &contents
        );
    }
}
