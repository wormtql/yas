use crate::capture;
use crate::inference::pre_process::{
    pre_process, raw_to_img, to_gray, uint8_raw_to_img, GrayImageFloat,
};
use image::{GrayImage, ImageBuffer, RgbImage};
use log::info;
use std::time::SystemTime;

pub mod arguments;
pub mod buffer;
pub mod character_name;
pub mod color;
pub mod utils;

#[derive(Debug)]
pub struct PixelRect {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

impl PixelRect {
    pub fn scale(&mut self, ratio: f64) {
        self.left = (self.left as f64 * ratio).round() as i32;
        self.top = (self.top as f64 * ratio).round() as i32;
        self.width = (self.width as f64 * ratio).round() as i32;
        self.height = (self.height as f64 * ratio).round() as i32;
    }
}

#[derive(Clone, Debug)]
pub struct PixelRectBound {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl PixelRectBound {
    pub fn capture_absolute(&self) -> Result<GrayImageFloat, String> {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        let rect = PixelRect {
            left: self.left,
            top: self.top,
            width: w,
            height: h,
        };
        let raw_u8 = capture::capture_absolute(&rect).unwrap();
        let raw_gray = to_gray(&raw_u8);
        let raw_after_pp = pre_process(raw_gray);

        match raw_after_pp {
            Some(im) => Ok(im),
            None => Err(String::from("capture error")),
        }
    }

    pub fn capture_relative(
        &self,
        left: i32,
        top: i32,
        use_pre_process: bool,
    ) -> Result<GrayImageFloat, String> {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        let rect = PixelRect {
            left: self.left + left,
            top: self.top + top,
            width: w,
            height: h,
        };
        let now = SystemTime::now();
        let raw_u8 = capture::capture_absolute(&rect).unwrap();
        info!("capture raw time: {}ms", now.elapsed().unwrap().as_millis());
        let raw_gray = to_gray(&raw_u8);
        let raw_after_pp = if use_pre_process {
            pre_process(raw_gray)
        } else {
            Some(raw_gray)
        };

        info!("preprocess time: {}ms", now.elapsed().unwrap().as_millis());

        match raw_after_pp {
            Some(im) => Ok(im),
            None => Err(String::from("capture error")),
        }
    }

    pub fn capture_relative_image(&self, left: i32, top: i32) -> Result<RgbImage, String> {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        let rect = PixelRect {
            left: self.left + left,
            top: self.top + top,
            width: w,
            height: h,
        };

        capture::capture_absolute_image(&rect)
    }
}

pub struct RawImage {
    pub data: Vec<f32>,
    pub w: u32,
    pub h: u32,
}

pub struct RawCaptureImage {
    pub data: Vec<u8>,
    pub w: u32,
    pub h: u32,
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
        let width = self.w;
        let height = self.h;
        let data = &self.data;

        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let index = (y * self.w + x) as usize;

            let b = data[index * 4];
            let g = data[index * 4 + 1];
            let r = data[index * 4 + 2];

            image::Rgb([r, g, b])
            // image::Luma([pixel])
        });

        img.save(path).unwrap();
    }

    pub fn crop_to_raw_img(&self, rect: &PixelRect) -> RawImage {
        // let now = SystemTime::now();
        let vol = rect.width * rect.height;
        let mut data = vec![0.0; vol as usize];
        for i in rect.left..rect.left + rect.width {
            for j in rect.top..rect.top + rect.height {
                let x = i;
                let y = j;
                let b: u8 = self.data[((y * self.w as i32 + x) * 4) as usize];
                let g: u8 = self.data[((y * self.w as i32 + x) * 4 + 1) as usize];
                let r: u8 = self.data[((y * self.w as i32 + x) * 4 + 2) as usize];

                let gray = r as f32 * 0.2989 + g as f32 * 0.5870 + b as f32 * 0.1140;
                let new_index = ((j - rect.top) * rect.width + i - rect.left) as usize;
                data[new_index] = gray;
            }
        }

        let im = RawImage {
            data,
            w: rect.width as u32,
            h: rect.height as u32,
        };
        // let im = pre_process(im);
        // No preprocess!

        // info!("preprocess time: {}ms", now.elapsed().unwrap().as_millis());
        // im.to_gray_image().save("test.png");
        im
    }
}

// pub struct

pub enum UI {
    Desktop,
    Mobile,
}
