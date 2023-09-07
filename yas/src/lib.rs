#![allow(clippy::missing_safety_doc)]
#![allow(clippy::single_match)]

use clap::Parser;
use env_logger::{Builder, Env};
use once_cell::sync::OnceCell;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod core;
pub mod export;

pub use crate::core::{
    genshin::GenshinArtifact, starrail::StarrailRelic, Game, Scanner, YasScannerConfig, CONFIG,
};
use common::*;
use core::ScanResult;
use std::{path::Path, fs};

pub static TARGET_GAME: OnceCell<Game> = OnceCell::new();

pub fn init_env(game: Game) {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    TARGET_GAME.set(game).ok();

    let dump_path = Path::new("dumps");
    if CONFIG.dump_mode && !dump_path.exists() {
        fs::create_dir(dump_path).unwrap();
    }

    #[cfg(target_os = "macos")]
    if !utils::request_capture_access() {
        crate::error_and_quit!("无法获取屏幕截图权限");
    }

    #[cfg(windows)]
    if !utils::is_admin() {
        crate::error_and_quit!("请以管理员身份运行该程序")
    }

    if let Some(v) = utils::check_update() {
        warn!("检测到新版本，请手动更新：{}", v);
    }
}

pub fn get_config() -> YasScannerConfig {
    YasScannerConfig::parse()
}

pub fn get_scanner(model: &[u8], content: &str) -> Scanner {
    let game_info = core::ui::get_game_info();
    let window_info = core::get_window_info(game_info.resolution);
    let scan_info = window_info.get_scan_info(game_info.window.size);

    Scanner::new(scan_info, game_info, model, content)
}

pub fn map_results_to<'a, T>(results: &'a [ScanResult]) -> Vec<T>
where
    T: TryFrom<&'a ScanResult, Error = ()>,
{
    results
        .iter()
        .map(|r| T::try_from(r))
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>()
}
