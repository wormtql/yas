#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub use windows::windows_control::WindowsSystemControl as SystemControl;
#[cfg(target_os = "macos")]
pub use macos::macos_control::MacOSControl as SystemControl;
#[cfg(target_os = "linux")]
pub use linux::linux_control::LinuxControl as SystemControl;
