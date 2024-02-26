use crate::common::color::Color;
use image::{buffer::ConvertBuffer, imageops::resize, imageops::FilterType::Triangle, RgbImage};

use anyhow::Result;
use crate::common::positioning::{Pos, Rect};
use xcap::{image::DynamicImage, Monitor};

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
        let monitors = Monitor::all().unwrap();
        let monitor = monitors.first().unwrap();

        let left = self.left as u32;
        let top = self.top as u32;
        let width = self.width as u32;
        let height = self.height as u32;

        let image = monitor.capture_image().unwrap();
        let rgb_img = DynamicImage::from(image)
            .crop(left, top, width, height)
            .to_rgb8();
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