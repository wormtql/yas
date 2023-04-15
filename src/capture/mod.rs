use std::{os::macos::raw, fs::File};

use image::{Rgb, RgbImage, ImageBuffer, RgbaImage, buffer::ConvertBuffer};

use crate::common::color::Color;
use crate::common::PixelRect;

use png::{Decoder, Encoder};

/// retures Ok(buf) on success
/// buf contains pixels in [b:u8, g:u8, r:u8, a:u8] format, as an `[[i32;width];height]`.
pub fn capture_absolute(
    PixelRect {
        left,
        top,
        width,
        height,
    }: &PixelRect,
) -> Result<RgbImage, String> {
    let screen = screenshots::Screen::all().expect("cannot get DisplayInfo")[0];
    let png_img = screen
        .capture_area(*left, *top, *width as u32, *height as u32)
        .expect("capture failed");
    png_decode(png_img)
}

fn png_decode(png_img:screenshots::Image) -> Result<RgbImage, String>  {
    let png_decoder = Decoder::new(png_img.buffer().as_slice());
    let mut png_reader = png_decoder.read_info().unwrap();

    let mut png_data_buf = vec![0; png_reader.output_buffer_size()];

    let info = png_reader.next_frame(&mut png_data_buf).unwrap();
    
    assert!(info.color_type == png::ColorType::Rgba, "Not rgba format image");

    let mut buffer = png_data_buf[..info.buffer_size()].to_vec();

    let rgba_img = RgbaImage::from_raw(png_img.width(), png_img.height(), png_data_buf).unwrap();
    let rgb_img: RgbImage = rgba_img.convert();
    rgba_img.save("dumps/rgba1.png");
    Ok(rgb_img)
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
    let screen = screenshots::Screen::all().expect("cannot get DisplayInfo")[0];
    let image = screen
        .capture_area(*left, *top, *width as u32, *height as u32)
        .expect("capture failed");

    let buffer = png_decode(image).unwrap();
    Ok(buffer)
}

pub fn get_color(x: u32, y: u32) -> Color {
    let im = capture_absolute(&PixelRect {
        left: x as i32,
        top: y as i32,
        width: 1,
        height: 1,
    })
    .unwrap();
    let pixel = im.get_pixel(0, 0);
    Color::from(pixel[0], pixel[1], pixel[2])
}
