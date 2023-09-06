#![allow(clippy::missing_safety_doc)]

use clap::Parser;
use env_logger::Builder;
use once_cell::sync::OnceCell;

#[macro_use]
extern crate log;

use log::LevelFilter;

pub mod common;
pub mod core;
pub mod export;

use common::*;
pub use core::{Game, Scanner, YasScannerConfig};

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

pub fn get_scanner(model: &[u8], content: &str) -> Scanner {
    let config = YasScannerConfig::parse();

    let game_info = core::ui::get_game_info();
    let window_info = core::get_window_info(game_info.resolution);
    let scan_info = window_info.get_scan_info(game_info.window.size);

    Scanner::new(scan_info, config, game_info, model, content)
}
