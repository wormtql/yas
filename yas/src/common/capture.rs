use super::{*, color::Color};
use crate::core::inference::pre_process::{pre_process, to_gray, GrayImageFloat};
use image::{buffer::ConvertBuffer, imageops::resize, imageops::FilterType::Triangle, RgbImage};

use std::ops::Add;
use anyhow::Result;

pub trait Capturable<T> {
    fn capture(&self) -> Result<T>;
}

pub trait RelativeCapturable<T> {
    fn capture_relative(self, pos: &Pos) -> Result<T>;
}

impl<'a, 'b, T> RelativeCapturable<T> for &'a dyn Capturable<T>
where
    &'a dyn Capturable<T>: Add<&'b Pos, Output = dyn Capturable<T>>,
{
    fn capture_relative(self, pos: &Pos) -> Result<T> {
        (self + pos).capture()
    }
}

impl Capturable<RgbImage> for Rect {
    fn capture(&self) -> Result<RgbImage> {
        let screen = screenshots::Screen::all()?[0];
        let mut rgb_img: RgbImage = screen
            .capture_area(self.origin.x, self.origin.y, self.size.width, self.size.height)?
            .convert();

        if rgb_img.width() > self.size.width && rgb_img.height() > self.size.height {
            rgb_img = resize(&rgb_img, self.size.width, self.size.height, Triangle);
        }

        Ok(rgb_img)
    }
}

impl Capturable<GrayImageFloat> for Rect {
    fn capture(&self) -> Result<GrayImageFloat> {
        let rgb_img: RgbImage = self.capture()?;

        match pre_process(to_gray(&rgb_img)) {
            Some(im) => Ok(im),
            None => Err(anyhow::anyhow!("Capture error")),
        }
    }
}

const UNIT_SIZE: Size = Size {
    width: 1,
    height: 1,
};

pub fn get_color(pos: Pos) -> Result<Color> {
    let image: RgbImage = Rect {
        origin: pos,
        size: UNIT_SIZE,
    }.capture()?;

    let pixel = image.get_pixel(0, 0);
    Ok(Color::new(pixel[0], pixel[1], pixel[2]))
}
