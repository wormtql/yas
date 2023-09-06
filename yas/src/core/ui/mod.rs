use crate::common::pos::Size;

pub mod os;

pub enum WindowSize {
    // PC
    Windows43x18,
    WIndows7x3,
    Windows16x9,
    Windows8x5,
    Windows4x3,
    // Mobile
    MacOS8x5,
}

impl WindowSize {
    pub fn new(size: Size) -> Self {
        if size.height * 43 == size.width * 18 {
            WindowSize::Windows43x18
        } else if size.height * 16 == size.width * 9 {
            WindowSize::Windows16x9
        } else if size.height * 8 == size.width * 5 {
            WindowSize::Windows8x5
        } else if size.height * 4 == size.width * 3 {
            WindowSize::Windows4x3
        } else if size.height * 7 == size.width * 3 {
            WindowSize::WIndows7x3
        } else if (size.height as i32 * 8 - size.width as i32 * 5).abs() < 20 {
            WindowSize::MacOS8x5
        } else {
            crate::error_and_quit!("不支持的分辨率");
        }
    }
}
