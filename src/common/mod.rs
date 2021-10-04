use crate::capture;
use crate::inference::pre_process::{pre_process, to_gray, raw_to_img};
use crate::info::info::ScanInfo;
use image::{GrayImage, RgbImage};
use crate::capture::capture_absolute;

pub mod utils;
pub mod buffer;
pub mod color;

#[derive(Debug)]
pub struct PixelRect {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
pub struct PixelRectBound {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl PixelRectBound {
    pub fn capture_absolute(&self) -> Result<RawImage, String> {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        let rect = PixelRect {
            left: self.left,
            top: self.top,
            width: w,
            height: h,
        };
        let raw_u8 = capture::capture_absolute(&rect).unwrap();
        let raw_gray = to_gray(raw_u8, w as u32, h as u32);
        let raw_after_pp = pre_process(raw_gray);
        Ok(raw_after_pp)
    }

    pub fn capture_relative(&self, info: &ScanInfo) -> Result<RawImage, String> {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        let rect = PixelRect {
            left: self.left + info.left as i32,
            top: self.top + info.top as i32,
            width: w,
            height: h,
        };
        let raw_u8 = capture::capture_absolute(&rect).unwrap();
        let raw_gray = to_gray(raw_u8, w as u32, h as u32);
        let raw_after_pp = pre_process(raw_gray);
        Ok(raw_after_pp)
    }

    pub fn capture_relative_image(&self, info: &ScanInfo) -> Result<RgbImage, String> {
        let w = self.right - self.left;
        let h = self.bottom - self.top;
        let rect = PixelRect {
            left: self.left + info.left as i32,
            top: self.top + info.top as i32,
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

impl RawImage {
    pub fn to_gray_image(&self) -> GrayImage {
        raw_to_img(&self)
    }
}

// pub struct