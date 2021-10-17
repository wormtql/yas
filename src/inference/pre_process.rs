use image::imageops::colorops::grayscale;
use image::{RgbImage, GrayImage, ImageBuffer};
use image::imageops::resize;

use crate::common::RawImage;

#[inline]
fn get_index(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

pub fn to_gray(raw: Vec<u8>, width: u32, height: u32) -> RawImage {
    let mut ans: Vec<f32> = vec![0.0; (width * height) as usize];
    for i in 0..width {
        for j in 0..height {
            let x = i;
            let y = height - j - 1;
            let b = raw[((y * width + x) * 4 + 0) as usize];
            let g = raw[((y * width + x) * 4 + 1) as usize];
            let r = raw[((y * width + x) * 4 + 2) as usize];

            let r = r as f32 / 255.0;
            let g = g as f32 / 255.0;
            let b = b as f32 / 255.0;

            let gray = r as f32 * 0.2989 + g as f32 * 0.5870 + b as f32 * 0.1140;
            let index = get_index(width, i, j);
            ans[index] = gray;
        }
    }

    RawImage {
        data: ans,
        h: height,
        w: width,
    }
}

pub fn normalize(im: &mut RawImage, auto_inverse: bool) {
    let width = im.w;
    let height = im.h;
    let data = &mut im.data;

    let mut max: f32 = 0.0;
    let mut min: f32 = 256.0;

    for i in 0..width {
        for j in 0..height {
            let index = get_index(width, i, j);
            let p = data[index];
            if p > max {
                max = p;
            }
            if p < min {
                min = p;
            }
        }
    }

    let flag_pixel = data[get_index(width, width - 1, height - 1)];
    let flag_pixel = (flag_pixel - min) / (max - min);

    for i in 0..width {
        for j in 0..height {
            let index = get_index(width, i, j);
            let p = data[index];
            data[index] = (p - min) / (max - min);
            if auto_inverse && flag_pixel > 0.5 {
                // println!("123");
                data[index] = 1.0 - data[index];
            }
            // if data[index] < 0.6 {
            //     data[index] = 0.0;
            // }
        }
    }
}

pub fn crop(im: &RawImage) -> RawImage {
    let width = im.w;
    let height = im.h;

    let mut min_col = width - 1;
    let mut max_col = 0;
    let mut min_row = height - 1;
    let mut max_row = 0_u32;

    for i in 0..width {
        for j in 0..height {
            let index = get_index(width, i, j);
            let p = im.data[index];
            if p > 0.7 {
                if i < min_col {
                    min_col = i;
                }
                if i > max_col {
                    max_col = i;
                }
                break;
            }
        }
    }

    for j in 0..height {
        for i in 0..width {
            let index = get_index(width, i, j);
            let p = im.data[index];
            if p > 0.7 {
                if j < min_row {
                    min_row = j;
                }
                if j > max_row {
                    max_row = j;
                }
                break;
            }
        }
    }

    let new_height = max_row - min_row + 1;
    let new_width = max_col - min_col + 1;

    let mut ans: Vec<f32> = vec![0.0; (new_width * new_height) as usize];

    for i in min_col..=max_col {
        for j in min_row..=max_row {
            let index = get_index(width, i, j);
            let new_index = get_index(new_width, i - min_col, j - min_row);
            ans[new_index] = im.data[index];
        }
    }

    RawImage {
        data: ans,
        w: new_width,
        h: new_height,
    }
}

pub fn raw_to_img(im: &RawImage) -> GrayImage {
    let width = im.w;
    let height = im.h;
    let data = &im.data;

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let index = get_index(width, x, y);
        let p = data[index];
        let pixel = (p * 255.0) as u32;
        let pixel: u8 = if pixel > 255 {
            255
        } else if pixel < 0 {
            0
        } else {
            pixel as u8
        };
        image::Luma([pixel])
    });

    img
}

pub fn resize_and_pad(im: &RawImage) -> RawImage {
    let w = im.w;
    let h = im.h;

    let new_width = (32.0 / h as f64 * w as f64) as u32;

    let img = raw_to_img(&im);
    let img = resize(&img, new_width, 32, image::imageops::FilterType::Triangle);

    let mut data: Vec<f32> = vec![0.0; 32 * 384];
    for i in 0..new_width.min(384) {
        for j in 0..32_u32 {
            let pixel = (img.get_pixel(i, j).0[0] as f32) / 255.0;
            data[(j * 384 + i) as usize] = pixel;
        }
    }

    RawImage {
        data,
        w: 384,
        h: 32,
    }
}

pub fn pre_process(im: RawImage) -> RawImage {
    let mut im = im;
    normalize(&mut im, true);
    let mut im = crop(&im);
    normalize(&mut im, false);

    let mut im = resize_and_pad(&im);
    for i in 0..im.w {
        for j in 0..im.h {
            let index = get_index(im.w, i, j);
            let p = im.data[index];
            if p < 0.5 {
                im.data[index] = 0.0;
            } else {
                im.data[index] = 1.0;
            }
        }
    }

    im
}

pub fn image_to_raw(im: GrayImage) -> RawImage {
    let w = im.width();
    let h = im.height();

    let mut data: Vec<f32> = vec![0.0; (w * h) as usize];
    for i in 0..w {
        for j in 0..h {
            let pixel = im.get_pixel(i, j).0[0] as f32 / 255.0;
            let index = get_index(w, i, j);
            data[index] = pixel;
        }
    }

    RawImage {
        data,
        w,
        h,
    }
}