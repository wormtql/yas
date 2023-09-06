use core::Game;

use env_logger::Builder;
use once_cell::sync::OnceCell;

#[macro_use]
extern crate log;

use log::LevelFilter;

pub mod common;
pub mod core;
pub mod export;

use common::utils;

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
