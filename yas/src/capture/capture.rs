use crate::common::color::Color;
use image::{buffer::ConvertBuffer, imageops::resize, imageops::FilterType::Triangle, RgbImage};

use anyhow::Result;
use crate::common::positioning::{Pos, Rect};

pub trait Capturable<ResultType> {
    fn capture(&self) -> Result<ResultType>;
}

pub trait RelativeCapturable<ResultType> {
    fn capture_relative(&self, offset: Pos) -> Result<ResultType>;
}

impl RelativeCapturable<RgbImage> for Rect {
    fn capture_relative(&self, offset: Pos) -> Result<RgbImage> {
        self.translate(offset).capture()
    }
}

impl Capturable<RgbImage> for Rect {
    fn capture(&self) -> Result<RgbImage> {
        // todo optimize screen logic
        let screen = screenshots::Screen::all()?[0];

        let left = self.left as i32;
        let top = self.top as i32;
        let width = self.width as u32;
        let height = self.height as u32;

        let mut rgb_img: RgbImage = screen
            .capture_area(
                left,
                top,
                width,
                height,
            )?
            .convert();

        // why is this step
        if rgb_img.width() > width && rgb_img.height() > height {
            rgb_img = resize(&rgb_img, self.width as u32, self.height as u32, Triangle);
        }

        Ok(rgb_img)
    }
}

// impl<T> Capturable<GrayImageFloat> for Rect<T>
// where
//     T: Copy + TryInto<i32>
// {
//     fn capture(&self) -> Result<GrayImageFloat> {
//         let rgb_img: RgbImage = self.capture()?;

//         // todo refactor pre process logic
//         match pre_process(to_gray(&rgb_img)) {
//             Some(im) => Ok(im),
//             None => Err(anyhow::anyhow!("Capture error")),
//         }
//     }
// }

pub fn get_color(pos: Pos) -> Result<Color> {
    let rect: Rect = Rect {
        left: pos.x,
        top: pos.y,
        width: 1.0,
        height: 1.0,
    };
    let image: RgbImage = rect.capture()?;

    let pixel = image.get_pixel(0, 0);
    Ok(Color::new(pixel[0], pixel[1], pixel[2]))
}