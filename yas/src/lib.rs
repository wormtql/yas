use env_logger::Builder;

#[macro_use]
extern crate log;

use log::LevelFilter;

pub mod common;
pub mod core;
pub mod export;
pub mod scanner;

use common::utils;

pub fn init_env() {
    Builder::new().filter_level(LevelFilter::Info).init();

    #[cfg(windows)]
    if !utils::is_admin() {
        utils::error_and_quit("请以管理员身份运行该程序")
    }

    if let Some(v) = utils::check_update() {
        warn!("检测到新版本，请手动更新：{}", v);
    }
}
