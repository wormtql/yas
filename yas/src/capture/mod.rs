mod capturer;
#[cfg(feature="capturer_screenshots")]
mod screenshots_capturer;
#[cfg(feature="capturer_xcap")]
mod xcap_capturer;
#[cfg(feature="capturer_libwayshot")]
mod libwayshot_capturer;
#[cfg(target_os = "windows")]
mod winapi_capturer;
mod generic_capturer;
// #[cfg(target_os = "windows")]
// mod window_capture_capturer;

pub use capturer::Capturer;
#[cfg(feature="capturer_screenshots")]
pub use screenshots_capturer::ScreenshotsCapturer;
#[cfg(target_os = "windows")]
pub use winapi_capturer::WinapiCapturer;
pub use generic_capturer::GenericCapturer;
