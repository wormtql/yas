#[cfg(windows)] extern crate winapi;
use std::io::Error;
use std::ffi::{OsStr};
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::mem::{size_of, transmute};

use winapi::um::winuser::{
    FindWindowW,
    GetDC,
    ReleaseDC,
    SetThreadDpiAwarenessContext,
    GetClientRect,
    SetForegroundWindow
};
use winapi::shared::windef::{HWND, HDC, RECT, HBITMAP, DPI_AWARENESS_CONTEXT};
use winapi::shared::ntdef::NULL;
use winapi::um::wingdi::{
    CreateCompatibleDC,
    DeleteObject,
    BitBlt,
    SRCCOPY,
    CreateCompatibleBitmap,
    SelectObject,
    GetObjectW,
    BITMAP,
    BITMAPINFOHEADER,
    BI_RGB,
    GetDIBits,
    BITMAPINFO,
    DIB_RGB_COLORS,
};
use winapi::ctypes::{c_void};
use winapi::um::winbase::{GlobalAlloc, GHND, GlobalLock};

use image::ImageBuffer;

use crate::common::{PixelRect, PixelRectBound};
use crate::common::color::Color;
use winapi::shared::windef::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE;
use self::winapi::um::wingdi::{GetDeviceCaps, HORZRES};
use self::winapi::shared::windef::DPI_AWARENESS_CONTEXT_SYSTEM_AWARE;


#[cfg(windows)]
unsafe fn unsafe_capture(rect: &PixelRect) -> Result<Vec<u8>, String> {
    // SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_SYSTEM_AWARE);

    let dc_window: HDC = GetDC(null_mut());

    let dc_mem: HDC = CreateCompatibleDC(dc_window);
    if dc_mem.is_null() {
        return Err(String::from("CreateCompatibleDC Failed"));
    }

    let hbm: HBITMAP = CreateCompatibleBitmap(dc_window, rect.width, rect.height);
    if hbm.is_null() {
        return Err(String::from("CreateCompatibleBitmap failed"));
    }

    SelectObject(dc_mem, hbm as *mut c_void);

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
        return Err(String::from("BitBlt failed"));
    }

    let mut bitmap: BITMAP = BITMAP {
        bmBits: 0 as *mut c_void,
        bmBitsPixel: 0,
        bmPlanes: 0,
        bmWidthBytes: 0,
        bmHeight: 0,
        bmWidth: 0,
        bmType: 0,
    };
    GetObjectW(
        hbm as *mut c_void,
        size_of::<BITMAP>() as i32,
        (&mut bitmap) as *mut BITMAP as *mut c_void
    );
    // println!("bitmap width: {}", bitmap.bmWidth);
    // println!("bitmap height: {}", bitmap.bmHeight);
    // println!("bitmap bits pixel: {}", bitmap.bmBitsPixel);

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
    // println!("bitmap size: {}", bitmap_size);
    // let mut buffer: Vec<u8> = vec![0; bitmap_size];

    // let h_dib = GlobalAlloc(GHND, bitmap_size);
    // let lpbitmap = GlobalLock(h_dib);
    // println!("bitmap {:p}", lpbitmap);
    let mut buffer: Vec<u8> = vec![0; bitmap_size];

    GetDIBits(
        dc_window,
        hbm,
        0,
        bitmap.bmHeight as u32,
        // lpbitmap,
        buffer.as_mut_ptr() as *mut c_void,
        (&mut bi) as *mut BITMAPINFOHEADER as *mut BITMAPINFO,
        DIB_RGB_COLORS
    );

    // let buffer: Vec<u8> = Vec::from_raw_parts(lpbitmap as *mut u8, bitmap_size, bitmap_size);
    // for i in 0..10 {
    //     println!("{}", buffer[i]);
    // }

    // println!("{}", buffer[0]);

    DeleteObject(hbm as *mut c_void);
    DeleteObject(dc_mem as *mut c_void);
    ReleaseDC(null_mut(), dc_window);

    Ok(buffer)
}

#[cfg(windows)]
pub fn capture_absolute(rect: &PixelRect) -> Result<Vec<u8>, String> {
    unsafe {
        unsafe_capture(&rect)
    }
}

#[cfg(windows)]
pub fn capture_absolute_image(rect: &PixelRect) -> Result<image::RgbImage, String> {
    let raw: Vec<u8> = match capture_absolute(rect) {
        Err(s) => {
            return Err(s);
        },
        Ok(v) => v,
    };

    let height = rect.height as u32;
    let width = rect.width as u32;

    let mut img = ImageBuffer::from_fn(
        width,
        height,
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

#[cfg(windows)]
pub fn get_color(x: u32, y: u32) -> Color {
    let im = capture_absolute(&PixelRect {
        left: x as i32,
        top: y as i32,
        width: 1,
        height: 1,
    }).unwrap();

    let b = im[0];
    let g = im[1];
    let r = im[2];
    Color(r, g, b)
}