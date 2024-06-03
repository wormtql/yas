mod capturer;
mod screenshots_capturer;
mod xcap_capturer;
#[cfg(target_os = "windows")]
mod winapi_capturer;
mod generic_capturer;
// #[cfg(target_os = "windows")]
// mod window_capture_capturer;

pub use capturer::Capturer;
pub use screenshots_capturer::ScreenshotsCapturer;
#[cfg(target_os = "windows")]
pub use winapi_capturer::WinapiCapturer;
pub use generic_capturer::GenericCapturer;
