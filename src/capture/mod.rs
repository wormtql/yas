use crate::common::color::Color;
use crate::common::PixelRect;

#[cfg(windows)]
mod windows;

pub trait CaptureImpl {
    fn capture_absolute(rect: &PixelRect) -> Result<Vec<u8>, String>;
    fn capture_absolute_image(rect: &PixelRect) -> Result<image::RgbImage, String>;
    fn get_color(x: u32, y: u32) -> Color;
}

pub struct Capture;
