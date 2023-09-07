#![allow(clippy::missing_safety_doc)]
#![allow(clippy::single_match)]

use anyhow::Result;
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
use common::{draw_config::DrawConfig, utils::ensure_dir, *};
use core::ScanResult;
pub static TARGET_GAME: OnceCell<Game> = OnceCell::new();

pub fn init_env(game: Game) {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    TARGET_GAME.set(game).ok();

    if CONFIG.dump_mode {
        ensure_dir("dumps");
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

pub fn get_scanner(model: &[u8], content: &str) -> Result<Scanner> {
    let game_info = core::ui::get_game_info();
    let window_info = core::get_window_info(game_info.resolution);
    let mut scan_info = window_info.get_scan_info(game_info.window.size);
    scan_info.move_to(&game_info.window.origin);

    if CONFIG.draw_config_only {
        ensure_dir("dumps");

        let mut image = scan_info.capture_window()?;
        scan_info.draw_config(&mut image);

        image.save("dumps/draw_config.png")?;

        info!("绘制配置完成，保存在 dumps/draw_config.png");

        std::process::exit(0)
    }

    Ok(Scanner::new(scan_info, game_info, model, content))
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
