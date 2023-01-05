use image::ImageBuffer;
use tract_onnx::prelude::tract_itertools::Itertools;


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
    // simply use the first screen.
    // todo: multi-screen support
    let screen = screenshots::Screen::all().ok_or("cannot get DisplayInfo")?[0];
    let mut rgba_buf = screen
        .capture_area(*left, *top, *width as u32, *height as u32)
        .ok_or("capture failed")?
        .buffer().clone();
    
    for mut rgba in &rgba_buf.iter_mut().chunks(4) {
        let r = rgba.nth(0).ok_or("cannot access r in rgba chunk")?;
        let b = rgba.nth(2).ok_or("cannot access b in rgba chunk")?;
        std::mem::swap(r, b);
    }

    Ok(rgba_buf)
}

pub fn capture_absolute_image(rect: &PixelRect) -> Result<image::RgbImage, String> {
    let raw: Vec<u8> = match capture_absolute(rect) {
        Err(s) => {
            return Err(s);
        }
        Ok(v) => v,
    };

    let height = rect.height as u32;
    let width = rect.width as u32;

    let mut img = ImageBuffer::from_fn(width, height, move |x, y| {
        let y = height - y - 1;
        let offset = (y * width + x) * 4;
        let b = raw[(offset + 0) as usize];
        let g = raw[(offset + 1) as usize];
        let r = raw[(offset + 2) as usize];
        image::Rgb([r, g, b])
    });

    Ok(img)
}

pub fn get_color(x: u32, y: u32) -> Color {
    let im = capture_absolute(&PixelRect {
        left: x as i32,
        top: y as i32,
        width: 1,
        height: 1,
    })
    .unwrap();

    let b = im[0];
    let g = im[1];
    let r = im[2];
    Color(r, g, b)
}
