use image::{Rgb, RgbImage};

use crate::common::color::Color;
use crate::common::PixelRect;

/// retures Ok(buf) on success
/// buf contains pixels in [b:u8, g:u8, r:u8, a:u8] format, as an `[[i32;width];height]`.
pub fn capture_absolute(
    PixelRect {
        left,
        top,
        width,
        height,
    }: &PixelRect,
) -> Result<Vec<u8>, String> {
    let screen = screenshots::Screen::all().ok_or("cannot get DisplayInfo")?[0];
    let (mut buffer, is_bgra) = screen
        .capture_area(*left, *top, *width as u32, *height as u32)
        .ok_or("capture failed")?;

    if !is_bgra {
        for chunk in buffer.chunks_mut(4) {
            let temp = chunk[0];
            chunk[0] = chunk[2];
            chunk[2] = temp;
        }
    }

    Ok(buffer)
}

pub fn capture_absolute_image(
    PixelRect {
        left,
        top,
        width,
        height,
    }: &PixelRect,
) -> Result<image::RgbImage, String> {
    // simply use the first screen.
    // todo: multi-screen support
    let screen = screenshots::Screen::all().ok_or("cannot get DisplayInfo")?[0];
    let (buffer, is_bgra) = screen
        .capture_area(*left, *top, *width as u32, *height as u32)
        .ok_or("capture failed")?;
    Ok(RgbImage::from_fn(*width as u32, *height as u32, |x, y| {
        let offset = (y * (*width as u32) + x) as usize;
        if is_bgra {
            Rgb([buffer[offset + 2], buffer[offset + 1], buffer[offset]])
        } else {
            Rgb([buffer[offset], buffer[offset + 1], buffer[offset + 2]])
        }
    }))
}

pub fn get_color(x: u32, y: u32) -> Color {
    let im = capture_absolute(&PixelRect {
        left: x as i32,
        top: y as i32,
        width: 1,
        height: 1,
    })
    .unwrap();
    Color::from(im[2], im[1], im[0])
}
