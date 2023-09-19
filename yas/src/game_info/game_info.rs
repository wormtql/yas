use crate::common::positioning::{Rect, Size};
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum UI {
    Desktop,
    Mobile,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Resolution {
    // PC
    Windows43x18,
    WIndows7x3,
    Windows16x9,
    Windows8x5,
    Windows4x3,
    // Mobile
    MacOS8x5,
}

impl Resolution {
    pub fn new(size: Size) -> Self {
        let height = size.height as u32;
        let width = size.width as u32;

        if height * 43 == width * 18 {
            Resolution::Windows43x18
        } else if height * 16 == width * 9 {
            Resolution::Windows16x9
        } else if height * 8 == width * 5 {
            Resolution::Windows8x5
        } else if height * 4 == width * 3 {
            Resolution::Windows4x3
        } else if height * 7 == width * 3 {
            Resolution::WIndows7x3
        } else if (height as i32 * 8 - width as i32 * 5).abs() < 20 {
            Resolution::MacOS8x5
        } else {
            crate::error_and_quit!("不支持的分辨率")
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameInfo {
    pub window: Rect,
    pub resolution: Resolution,
    pub is_cloud: bool,
    pub ui: UI,
}
