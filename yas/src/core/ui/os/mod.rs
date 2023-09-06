use crate::common::*;

pub struct GameInfo {
    pub window_pos: Rect,
    pub is_cloud: bool,
    pub ui: UI,
}

#[cfg(windows)]
mod winodws;
#[cfg(windows)]
pub use winodws::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
