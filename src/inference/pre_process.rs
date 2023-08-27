use image::imageops::{overlay, resize};
use image::{GenericImageView, GrayImage, ImageBuffer, Luma, RgbImage};

use crate::common::RawImage;
pub type GrayImageFloat = ImageBuffer<Luma<f32>, Vec<f32>>;

pub trait ImageConvExt {
    fn to_common_grayscale(&self) -> GrayImage;
}

impl ImageConvExt for GrayImageFloat {
    fn to_common_grayscale(&self) -> GrayImage {
        let img = ImageBuffer::from_fn(self.width(), self.height(), |x, y| {
            let pv = self.get_pixel(x, y)[0];
            let pixel = (pv * 255.0) as u32;
            let pixel: u8 = if pixel > 255 { 255 } else { pixel as _ };
            image::Luma([pixel])
        });
        img
    }
}

#[inline]
fn get_index(width: u32, x: u32, y: u32) -> usize {
    (y * width + x) as usize
}

pub fn to_gray(raw: &RgbImage) -> GrayImageFloat {
    let mut new_gray = GrayImageFloat::new(raw.width(), raw.height());
    for (x, y) in (0..raw.width()).zip(0..raw.height()) {
        let rgb = raw.get_pixel(x, y);
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];

        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let gray = r as f32 * 0.2989 + g as f32 * 0.5870 + b as f32 * 0.1140;
        let grayp = new_gray.get_pixel_mut(x, y);
        grayp[0] = gray;
    }
    new_gray
}

pub fn normalize(im: &mut GrayImageFloat, auto_inverse: bool) -> bool {
    let width = im.width();
    let height = im.height();

    if width == 0 || height == 0 {
        println!("wrong width or height");
        return false;
    }
    // info!("in normalize: width = {}, height = {}", width, height);

    let mut max: f32 = 0.0;
    let mut min: f32 = 256.0;

    for i in 0..width {
        for j in 0..height {
            // info!("i = {}, j = {}, width = {}, index = {}", i, j, width, index);
            let p = im.get_pixel(i, j)[0];
            if p > max {
                max = p;
            }
            if p < min {
                min = p;
            }
        }
    }

    if max == min {
        return false;
    }

    let flag_pixel = im.get_pixel(width - 2, height - 1)[0];
    let flag_pixel = (flag_pixel - min) / (max - min);

    for i in 0..width {
        for j in 0..height {
            let p = im.get_pixel_mut(i, j);
            let pv = p[0];
            let mut new_pv = (pv - min) / (max - min);
            if auto_inverse && flag_pixel > 0.5 {
                // println!("123");
                new_pv = 1.0 - new_pv;
            }
            p[0] = new_pv;
            // if data[index] < 0.6 {
            //     data[index] = 0.0;
            // }
        }
    }

    true
}

pub fn crop(im: &GrayImageFloat) -> GrayImageFloat {
    let width = im.width();
    let height = im.height();

    let mut min_col = width - 1;
    let mut max_col = 0;
    let mut min_row = height - 1;
    let mut max_row = 0_u32;

    for i in 0..width {
        for j in 0..height {
            let p = im.get_pixel(i, j)[0];
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
            let p = im.get_pixel(i, j)[0];
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

    let _ans: Vec<f32> = vec![0.0; (new_width * new_height) as usize];
    let cropped_im = im.view(min_col, min_row, new_width, new_height).to_image();

    cropped_im
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
        Luma([pixel])
    });

    img
}

pub fn uint8_raw_to_img(im: &RawImage) -> GrayImage {
    let width = im.w;
    let height = im.h;
    let data = &im.data;

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let index = get_index(width, x, y);
        let pixel = data[index] as u32;
        let pixel: u8 = if pixel > 255 {
            255
        } else if pixel < 0 {
            0
        } else {
            pixel as u8
        };
        Luma([pixel])
    });

    img
}

pub fn resize_and_pad(im: &GrayImageFloat) -> GrayImageFloat {
    let w = im.width();
    let h = im.height();

    let new_width = if w as f64 / (h as f64) > 384.0 / 32.0 {
        384
    } else {
        std::cmp::min((32.0 / h as f64 * w as f64) as u32, 384)
    };

    let new_height = std::cmp::min((384.0 / w as f64 * h as f64) as u32, 32);

    //let img = raw_to_img(&im);
    //let img = resize(&img, new_width, 32, image::imageops::FilterType::Triangle);

    let img = resize(
        im,
        new_width,
        new_height,
        image::imageops::FilterType::Triangle,
    );

    let data: Vec<f32> = vec![0.0; 32 * 384];
    let mut padded_im = ImageBuffer::from_vec(384, 32, data).unwrap();
    overlay(&mut padded_im, &img, 0, 0);
    padded_im
}

pub fn pre_process(im: GrayImageFloat) -> Option<GrayImageFloat> {
    let mut im = im;
    if !normalize(&mut im, true) {
        return None;
    }
    let mut im = crop(&im);

    normalize(&mut im, false);

    let mut im = resize_and_pad(&im);

    for i in 0..im.width() {
        for j in 0..im.height() {
            let p = im.get_pixel_mut(i, j);
            let pv = p[0];
            if pv < 0.53 {
                p[0] = 0.0;
            } else {
                p[0] = 1.0;
            }
        }
    }

    Some(im)
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

    RawImage { data, w, h }
}
