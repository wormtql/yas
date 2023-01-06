use crate::common::color::Color;
use crate::common::PixelRect;

/// retures Ok(buf) on success
/// buf contains pixels in [b:u8, g:u8, r:u8, a:u8] format, as an `[[i32;width];height]`.
pub fn capture_absolute(rect: &PixelRect) -> Result<Vec<u8>, String> {
    let mut re = Vec::with_capacity((rect.height * rect.width) as usize * 4);
    let rgb_image = capture_absolute_image(rect)?;

    let width = rect.width as usize;
    for (i, pixel) in rgb_image.pixels().enumerate() {
        let x = i % width;
        let y = i / width;
        let rgb_arr = pixel.0;
        let offset = (y * width + x) * 4;
        re[offset] = rgb_arr[2];
        re[offset + 1] = rgb_arr[1];
        re[offset + 2] = rgb_arr[0];
        re[offset + 3] = 255;
    }
    Ok(re)
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
    let png = screen
        .capture_area(*left, *top, *width as u32, *height as u32)
        .ok_or("capture failed")?;
    let rgba_image = image::load_from_memory_with_format(png.buffer(), image::ImageFormat::Png);
    match rgba_image {
        Ok(dyc_img) => Ok(dyc_img.to_rgb8()),
        Err(e) => Err("Error occured on image loading".into()),
    }
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
