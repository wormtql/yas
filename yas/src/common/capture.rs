use super::{color::Color, *};
use crate::core::inference::{pre_process, to_gray, GrayImageFloat};
use image::{buffer::ConvertBuffer, imageops::resize, imageops::FilterType::Triangle, RgbImage};

use anyhow::Result;

pub trait Capturable<T> {
    fn capture(&self) -> Result<T>;
}

pub trait RelativeCapturable<'a, T> {
    fn capture_relative(&self, offset: &'a Pos) -> Result<T>;
}

impl<'a, T> RelativeCapturable<'a, T> for Rect
where
    Rect: Capturable<T>,
{
    fn capture_relative(&self, offset: &'a Pos) -> Result<T> {
        (self + offset).capture()
    }
}

impl Capturable<RgbImage> for Rect {
    fn capture(&self) -> Result<RgbImage> {
        let screen = screenshots::Screen::all()?[0];

        debug!("Capture: {}", self);

        let mut rgb_img: RgbImage = screen
            .capture_area(
                self.origin.x as i32,
                self.origin.y as i32,
                self.size.width,
                self.size.height,
            )?
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

pub fn get_color(pos: Pos<u32>) -> Result<Color> {
    let image: RgbImage = Rect {
        origin: pos,
        size: UNIT_SIZE,
    }
    .capture()?;

    let pixel = image.get_pixel(0, 0);
    Ok(Color::new(pixel[0], pixel[1], pixel[2]))
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} x {}]", self.width, self.height)
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect {} -> {}", self.origin, self.size)
    }
}
