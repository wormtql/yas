#![allow(clippy::missing_safety_doc)]
#![allow(clippy::single_match)]

use anyhow::Result;
use env_logger::{Builder, Env};
use indicatif::{MultiProgress, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use once_cell::sync::OnceCell;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

pub mod common;
pub mod core;
pub mod export;
pub mod draw_capture_region;
pub mod capture;
pub mod utils;
pub mod scanner;
pub mod inference;
pub mod game_info;
pub mod arguments_builder;
pub mod window_info;
pub mod system_control;

pub use crate::core::{
    genshin::GenshinArtifact, starrail::StarrailRelic, Game, Scanner, YasScannerConfig, CONFIG,
};
use common::{draw_config::DrawConfig, utils::ensure_dir, *};
use core::ScanResult;

pub static TARGET_GAME: OnceCell<Game> = OnceCell::new();

lazy_static! {
    pub static ref MULTI_PROGRESS: MultiProgress = MultiProgress::new();
    pub static ref PROGRESS_STYLE: ProgressStyle = ProgressStyle::with_template(
        "{spinner:.green} {prefix:.green.bold} [{wide_bar:.cyan/blue}] {pos:>4}/{len:4.dim} {msg:50}"
    )
    .unwrap()
    .progress_chars("#>-")
    .tick_chars("😐😐😮😮😦😦😧😧🤯💥✨\u{3000}\u{3000}\u{3000}");
}

pub fn init_env(game: Game) -> Result<()> {
    LogWrapper::new(
        MULTI_PROGRESS.clone(),
        Builder::from_env(Env::default().default_filter_or("info")).build(),
    )
    .try_init()?;

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

    Ok(())
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
