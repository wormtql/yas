use std::fmt::Arguments;
use std::fs;
use std::thread;
use std::time::Duration;
use serde::Deserialize;
use std::io::stdin;
use std::process;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

pub fn sleep(ms: u32) {
    thread::sleep(Duration::from_millis(ms as u64));
}

pub fn read_file_to_string(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn quit() -> ! {
    let mut s: String = String::new();
    stdin().read_line(&mut s).unwrap();
    process::exit(0);
}

#[doc(hidden)]
pub fn error_and_quit_internal(args: Arguments) -> ! {
    panic!("Error: {}", args);
}

#[macro_export]
macro_rules! error_and_quit {
    ($($arg:tt)*) => (
        $crate::utils::error_and_quit_internal(format_args!($($arg)*))
    );
}

#[cfg(not(windows))]
pub fn is_rmb_down() -> bool {
    false
}

#[derive(Deserialize)]
pub struct GithubTag {
    pub name: String,
}

pub fn ensure_dir(path: &str) {
    if !std::path::Path::new(path).exists() {
        fs::create_dir_all(path).unwrap();
    }
}