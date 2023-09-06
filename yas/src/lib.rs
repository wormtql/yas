#![allow(clippy::missing_safety_doc)]
#![allow(clippy::single_match)]

use clap::Parser;
use env_logger::Builder;
use once_cell::sync::OnceCell;

#[macro_use]
extern crate log;

use log::LevelFilter;

pub mod common;
pub mod core;
pub mod export;

pub use crate::core::{Game, Scanner, YasScannerConfig};
use common::*;
use core::ScanResult;

pub static TARGET_GAME: OnceCell<Game> = OnceCell::new();

pub fn init_env(game: Game) {
    Builder::new().filter_level(LevelFilter::Info).init();

    TARGET_GAME.set(game).ok();

    #[cfg(windows)]
    if !utils::is_admin() {
        utils::error_and_quit("请以管理员身份运行该程序")
    }

    if let Some(v) = utils::check_update() {
        warn!("检测到新版本，请手动更新：{}", v);
    }
}

pub fn get_config() -> YasScannerConfig {
    YasScannerConfig::parse()
}

pub fn get_scanner(model: &[u8], content: &str, config: &YasScannerConfig) -> Scanner {
    let game_info = core::ui::get_game_info();
    let window_info = core::get_window_info(game_info.resolution);
    let scan_info = window_info.get_scan_info(game_info.window.size);

    Scanner::new(scan_info, config.clone(), game_info, model, content)
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
