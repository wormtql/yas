use std::fmt::Arguments;
use std::fs;
use std::io::stdin;
use std::process;
use std::thread;
use std::time::Duration;

use super::*;
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, USER_AGENT};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

pub fn sleep(ms: u32) {
    let time = Duration::from_millis(ms as u64);
    thread::sleep(time);
}

pub fn read_file_to_string(path: String) -> String {
    fs::read_to_string(path).unwrap()
}

#[doc(hidden)]
pub fn error_and_quit_internal(args: Arguments) -> ! {
    eprintln!("{}，按任意键退出。", args);
    let mut s: String = String::new();
    stdin().read_line(&mut s).unwrap();
    process::exit(0);
}

#[macro_export]
macro_rules! error_and_quit {
    ($($arg:tt)*) => (
        $crate::common::utils::error_and_quit_internal(format_args!($($arg)*))
    );
}

#[cfg(not(windows))]
pub fn is_rmb_down() -> bool {
    false
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn check_update() -> Option<String> {
    let client = Client::new();

    let resp = client
        .get("https://api.github.com/repos/wormtql/yas/tags")
        .timeout(Duration::from_secs(5))
        .header(USER_AGENT, HeaderValue::from_static("reqwest"))
        .send()
        .ok()?
        .json::<Vec<GithubTag>>()
        .ok()?;

    let latest = if resp.is_empty() {
        return None;
    } else {
        resp[0].name.clone()
    };
    let latest = &latest[1..];

    let latest_sem: semver::Version = semver::Version::parse(latest).unwrap();
    let current_sem: semver::Version = semver::Version::parse(VERSION).unwrap();

    if latest_sem > current_sem {
        Some(String::from(latest))
    } else {
        None
    }
}
