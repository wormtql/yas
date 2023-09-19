use std::fmt::Arguments;
use std::fs;
use std::thread;
use std::time::Duration;
use serde::Deserialize;

// use crate::core::VERSION;

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
    thread::sleep(Duration::from_millis(ms as u64));
}

pub fn read_file_to_string(path: String) -> String {
    fs::read_to_string(path).unwrap()
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

pub fn check_update() -> Option<String> {
    None
    // todo
    // let client = Client::new();

    // let resp = client
    //     .get("https://api.github.com/repos/wormtql/yas/tags")
    //     .timeout(Duration::from_secs(5))
    //     .header(USER_AGENT, HeaderValue::from_static("reqwest"))
    //     .send()
    //     .ok()?
    //     .json::<Vec<GithubTag>>()
    //     .ok()?;

    // let latest = if resp.is_empty() {
    //     return None;
    // } else {
    //     resp[0].name.clone()
    // };
    // let latest = &latest[1..];

    // let latest_sem: semver::Version = semver::Version::parse(latest).unwrap();
    // let current_sem: semver::Version = semver::Version::parse(VERSION).unwrap();

    // if latest_sem > current_sem {
    //     Some(String::from(latest))
    // } else {
    //     None
    // }
}

pub fn ensure_dir(path: &str) {
    if !std::path::Path::new(path).exists() {
        fs::create_dir_all(path).unwrap();
    }
}
