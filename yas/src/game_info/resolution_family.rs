use serde::{Deserialize, Serialize};
use crate::positioning::Size;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResolutionFamily {
    // PC
    Windows43x18,
    Windows7x3,
    Windows16x9,
    Windows8x5,
    Windows4x3,
    // Mobile
    MacOS8x5,
}

impl ResolutionFamily {
    pub fn new(size: Size<usize>) -> Option<Self> {
        // todo get OS at run time

        let height = size.height as u32;
        let width = size.width as u32;

        if height * 43 == width * 18 {
            Some(ResolutionFamily::Windows43x18)
        } else if height * 16 == width * 9 {
            Some(ResolutionFamily::Windows16x9)
        } else if height * 8 == width * 5 {
            Some(ResolutionFamily::Windows8x5)
        } else if height * 4 == width * 3 {
            Some(ResolutionFamily::Windows4x3)
        } else if height * 7 == width * 3 {
            Some(ResolutionFamily::Windows7x3)
        } else if (height as i32 * 8 - width as i32 * 5).abs() < 20 {
            Some(ResolutionFamily::MacOS8x5)
        } else {
            None
        }
    }
}
