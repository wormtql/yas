mod capturer;
mod generic_capturer;

pub use capturer::Capturer;
pub use generic_capturer::GenericCapturer;

// windows

#[cfg(target_os = "windows")]
mod screenshots_capturer;
#[cfg(target_os = "windows")]
mod winapi_capturer;
#[cfg(target_os = "windows")]
mod windows_capturer;

#[cfg(target_os = "windows")]
pub use screenshots_capturer::ScreenshotsCapturer;
#[cfg(target_os = "windows")]
pub use winapi_capturer::WinapiCapturer;
#[cfg(target_os = "windows")]
pub use windows_capturer::WindowsCapturer;

// linux
#[cfg(target_os = "linux")]
mod libwayshot_capturer;

#[cfg(target_os = "linux")]
pub use libwayshot_capturer::LibwayshotCapturer;
