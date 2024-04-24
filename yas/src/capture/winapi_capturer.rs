use std::mem::size_of;
use std::ptr::null_mut;

use anyhow::{anyhow, Result};
use image::{ImageBuffer, RgbImage};
use winapi::shared::windef::{HBITMAP, HDC};
use winapi::um::wingdi::{
    BI_RGB,
    BitBlt,
    BITMAP,
    BITMAPINFO,
    BITMAPINFOHEADER,
    CreateCompatibleBitmap,
    CreateCompatibleDC,
    DeleteObject,
    DIB_RGB_COLORS,
    GetDIBits,
    GetObjectW,
    SelectObject,
    SRCCOPY,
};
use winapi::um::winuser::{GetDC, ReleaseDC};

use crate::capture::Capturer;
use crate::positioning::{Pos, Rect};

// BGRA
unsafe fn unsafe_capture(rect: Rect<i32>) -> Result<Vec<u8>> {
    let dc_window: HDC = GetDC(null_mut());

    let dc_mem: HDC = CreateCompatibleDC(dc_window);
    if dc_mem.is_null() {
        return Err(anyhow!("CreateCompatibleDC failed"));
    }

    let hbm: HBITMAP = CreateCompatibleBitmap(dc_window, rect.width, rect.height);
    if hbm.is_null() {
        return Err(anyhow!("CreateCompatibleBitmap failed"));
    }

    SelectObject(dc_mem, hbm as *mut winapi::ctypes::c_void);

    let result = BitBlt(
        dc_mem,
        0,
        0,
        rect.width,
        rect.height,
        dc_window,
        rect.left,
        rect.top,
        SRCCOPY
    );
    if result == 0 {
        return Err(anyhow!("BitBlt failed"));
    }

    let mut bitmap: BITMAP = BITMAP {
        bmBits: 0 as *mut winapi::ctypes::c_void,
        bmBitsPixel: 0,
        bmPlanes: 0,
        bmWidthBytes: 0,
        bmHeight: 0,
        bmWidth: 0,
        bmType: 0,
    };
    GetObjectW(
        hbm as *mut winapi::ctypes::c_void,
        size_of::<BITMAP>() as i32,
        (&mut bitmap) as *mut BITMAP as *mut winapi::ctypes::c_void
    );

    let mut bi: BITMAPINFOHEADER = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: bitmap.bmWidth,
        biHeight: bitmap.bmHeight,
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    let bitmap_size: usize = (((bitmap.bmWidth * 32 + 31) / 32) * 4 * bitmap.bmHeight) as usize;
    let mut buffer: Vec<u8> = vec![0; bitmap_size];

    GetDIBits(
        dc_window,
        hbm,
        0,
        bitmap.bmHeight as u32,
        // lpbitmap,
        buffer.as_mut_ptr() as *mut winapi::ctypes::c_void,
        (&mut bi) as *mut BITMAPINFOHEADER as *mut BITMAPINFO,
        DIB_RGB_COLORS
    );

    DeleteObject(hbm as *mut winapi::ctypes::c_void);
    DeleteObject(dc_mem as *mut winapi::ctypes::c_void);
    ReleaseDC(null_mut(), dc_window);

    Ok(buffer)
}

pub struct WinapiCapturer;

impl WinapiCapturer {
    pub fn new() -> Self {
        // todo maybe we can explicitly account for windows scale, and remove the call
        // crate::utils::set_dpi_awareness();
        Self
    }
}

impl Capturer<RgbImage> for WinapiCapturer {
    fn capture_rect(&self, rect: Rect<i32>) -> Result<RgbImage> {
        let raw: Vec<u8> = unsafe {
            unsafe_capture(rect)?
        };

        let height = rect.height as u32;
        let width = rect.width as u32;

        let img = ImageBuffer::from_fn(
            rect.width as u32,
            rect.height as u32,
            move |x, y| {
                let y = height - y - 1;
                let b = raw[((y * width + x) * 4 + 0) as usize];
                let g = raw[((y * width + x) * 4 + 1) as usize];
                let r = raw[((y * width + x) * 4 + 2) as usize];
                image::Rgb([r, g, b])
            }
        );

        Ok(img)
    }

    fn capture_color(&self, pos: Pos<i32>) -> Result<image::Rgb<u8>> {
        let raw: Vec<u8> = unsafe {
            unsafe_capture(Rect {
                left: pos.x,
                top: pos.y,
                width: 1,
                height: 1
            })?
        };
        let r = raw[2];
        let g = raw[1];
        let b = raw[0];
        Ok(image::Rgb([r, g, b]))
    }
}
