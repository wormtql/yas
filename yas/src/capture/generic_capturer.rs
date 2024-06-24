#[cfg(target_os = "windows")]
use crate::capture::WindowsCapturer;
#[cfg(target_os = "windows")]
pub type GenericCapturer = WindowsCapturer;

#[cfg(target_os = "linux")]
use crate::capture::LibwayshotCapturer;
#[cfg(target_os = "linux")]
pub type GenericCapturer = LibwayshotCapturer;

// #[cfg(target_os = "macos")]
// pub type GenericCapturer = 