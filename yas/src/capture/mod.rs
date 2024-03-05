mod capturer;
mod screenshots_capturer;
mod xcap_capturer;
#[cfg(target_os = "windows")]
mod winapi_capturer;
mod generic_capturer;

pub use capturer::Capturer;
pub use screenshots_capturer::ScreenshotsCapturer;
pub use winapi_capturer::WinapiCapturer;
pub use generic_capturer::GenericCapturer;
