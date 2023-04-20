use std::fs;
use std::io::stdin;
use std::process;
use std::time::Duration;
use std::{thread, time};

use crate::dto::GithubTag;
use crate::common::PixelRect;
use log::error;
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, USER_AGENT};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

pub fn sleep(ms: u32) {
    let time = time::Duration::from_millis(ms as u64);
    thread::sleep(time);
}

pub fn read_file_to_string(path: String) -> String {
    let content = fs::read_to_string(path).unwrap();
    content
}

pub fn error_and_quit(msg: &str) -> ! {
    error!("{}, 按Enter退出", msg);
    let mut s: String = String::new();
    stdin().read_line(&mut s);
    process::exit(0);
}

pub fn error_and_quit_if<T>(data: Option<T>, msg: &str) -> T {
    match data {
        Some(happy) => happy,
        None => error_and_quit(msg),
    }
}

pub fn detect_game_window() -> Option<(PixelRect, bool)> {
    #[cfg(windows)]
    {
        use winapi::shared::windef::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE;
        use winapi::um::wingdi::{GetDeviceCaps, HORZRES};
        use winapi::um::winuser::{
            GetDpiForSystem, GetSystemMetrics, SetForegroundWindow, SetProcessDPIAware,
            SetThreadDpiAwarenessContext, ShowWindow, SW_RESTORE, SW_SHOW,
        };
        // use winapi::um::shellscalingapi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};

        set_dpi_awareness();

        let (is_cloud, hwnd) = match find_window_local() {
            Ok(h) => (false, h),
            Err(_) => (true, find_window_cloud().ok()?),
        };

        unsafe {
            ShowWindow(hwnd, SW_RESTORE);
        }
        // sleep(1000);
        unsafe {
            SetForegroundWindow(hwnd);
        }
        sleep(1000);

        let rect = get_client_rect(hwnd).ok()?;
        Some((rect, is_cloud))
    }

    #[cfg(all(target_os = "linux"))]
    {
        let window_id = unsafe {
            String::from_utf8_unchecked(
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(r#" xwininfo|grep "Window id"|cut -d " " -f 4 "#)
                    .output()
                    .unwrap()
                    .stdout,
            )
        };
        let window_id = window_id.trim_end_matches("\n");

        let position_size = unsafe {
            String::from_utf8_unchecked(
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&format!(r#" xwininfo -id {window_id}|cut -f 2 -d :|tr -cd "0-9\n"|grep -v "^$"|sed -n "1,2p;5,6p" "#))
                    .output()
                    .unwrap()
                    .stdout,
            )
        };

        let mut info = position_size.split("\n");

        let left = info.next().unwrap().parse().unwrap();
        let top = info.next().unwrap().parse().unwrap();
        let width = info.next().unwrap().parse().unwrap();
        let height = info.next().unwrap().parse().unwrap();

        let rect = PixelRect {
            left,
            top,
            width,
            height,
        };
        let is_cloud = false; // todo: detect cloud genshin by title

        Some((rect, is_cloud))
    }
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

    let latest = if resp.len() == 0 {
        return None;
    } else {
        resp[0].name.clone()
    };
    let latest = &latest[1..];

    let latest_sem: semver::Version = semver::Version::parse(&latest).unwrap();
    let current_sem: semver::Version = semver::Version::parse(VERSION).unwrap();

    if latest_sem > current_sem {
        Some(String::from(latest))
    } else {
        None
    }
}
