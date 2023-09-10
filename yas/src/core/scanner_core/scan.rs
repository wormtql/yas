use super::inference::{pre_process, to_gray};
use super::*;
use crate::common::color::Color;
use crate::TARGET_GAME;
use anyhow::Result;
use image::*;
use crate::inference::pre_process::ImageConvExt;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use crate::capture::capture;

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
