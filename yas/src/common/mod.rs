pub mod buffer;
pub mod capture;
pub mod character_name;
pub mod color;
pub mod pos;
pub mod utils;

use crate::inference::pre_process::{raw_to_img, uint8_raw_to_img};
use image::{GrayImage, ImageBuffer};
pub use pos::*;

pub enum UI {
    Desktop,
    Mobile,
}

pub struct RawImage {
    pub data: Vec<f32>,
    pub size: Size,
}

pub struct RawCaptureImage {
    pub data: Vec<u8>,
    pub size: Size,
}

impl RawImage {
    pub fn to_gray_image(&self) -> GrayImage {
        raw_to_img(&self)
    }

    pub fn grayscale_to_gray_image(&self) -> GrayImage {
        uint8_raw_to_img(&self)
    }
}

impl RawCaptureImage {
    pub fn save(&self, path: &str) {
        let width = self.size.width;
        let height = self.size.height;
        let data = &self.data;

        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let index = (y * width + x) as usize;

            let b = data[index * 4];
            let g = data[index * 4 + 1];
            let r = data[index * 4 + 2];

            image::Rgb([r, g, b])
        });

        img.save(path).unwrap();
    }

    pub fn crop_to_raw_img(&self, rect: &Rect) -> RawImage {
        let mut data = vec![0.0; rect.size.area() as usize];
        let ori = &rect.origin;
        let size = &rect.size;
        let (x, y) = (ori.x as usize, ori.y as usize);
        let (w, h) = (size.width as usize, size.height as usize);

        for i in x..x + w {
            for j in y..y + h {
                let b: u8 = self.data[(j * w + x) * 4];
                let g: u8 = self.data[(j * w + x) * 4 + 1];
                let r: u8 = self.data[(j * w + x) * 4 + 2];

                let gray = r as f32 * 0.2989 + g as f32 * 0.5870 + b as f32 * 0.1140;
                let new_index = ((j - y) * w + i - x) as usize;
                data[new_index] = gray;
            }
        }

        RawImage {
            data,
            size: rect.size
        }
    }
}
