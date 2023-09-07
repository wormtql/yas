use super::*;
use anyhow::Result;
use image::RgbImage;
use std::{fmt::*, ops::DerefMut};

pub mod genshin;
pub mod starrail;

mod scanner;
pub use scanner::*;

type R = RectBound<WindowInfoType>;
type P = Pos<WindowInfoType>;
type S = Size<WindowInfoType>;

#[derive(Clone, Debug)]
pub enum WindowInfo {
    Genshin(&'static genshin::GenshinWindowInfo),
    StarRail(&'static starrail::StarRailWindowInfo),
}

#[derive(Clone, Debug)]
pub enum ScanInfo {
    Genshin(genshin::GenshinScanInfo),
    StarRail(starrail::StarRailScanInfo),
}

#[derive(Clone, Debug)]
pub enum Item {
    GenshinArtifact(genshin::GenshinArtifact),
    StarrailRelic(starrail::StarrailRelic),
}

#[derive(Debug)]
pub enum Game {
    Genshin,
    StarRail,
    Unknown,
}

pub fn get_window_info(resolution: Resolution) -> WindowInfo {
    match crate::TARGET_GAME.get().unwrap() {
        Game::Genshin => WindowInfo::Genshin(genshin::get_window_info(resolution)),
        Game::StarRail => WindowInfo::StarRail(starrail::get_window_info(resolution)),
        _ => crate::error_and_quit!("不支持的游戏类型"),
    }
}

pub fn parse_level(str_level: &str) -> u8 {
    let pos = str_level.find('+');

    if pos.is_none() {
        return str_level.parse::<u8>().unwrap();
    }

    str_level[pos.unwrap()..].parse::<u8>().unwrap()
}

impl WindowInfo {
    pub fn get_scan_info(&self, size: Size) -> ScanInfo {
        let float_size = Size::<f64> {
            width: size.width as f64,
            height: size.height as f64,
        };

        match self {
            WindowInfo::Genshin(info) => ScanInfo::Genshin(info.to_scan_info(float_size)),
            WindowInfo::StarRail(info) => ScanInfo::StarRail(info.to_scan_info(float_size)),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::Genshin => write!(f, "原神"),
            Game::StarRail => write!(f, "崩坏：星穹铁道"),
            Game::Unknown => write!(f, "未知"),
        }
    }
}

impl ScanInfo {
    pub fn inner_genshin(&self) -> &genshin::GenshinScanInfo {
        match self {
            ScanInfo::Genshin(info) => info,
            _ => crate::error_and_quit!("ScanInfo is not genshin"),
        }
    }

    pub fn inner_starrail(&self) -> &starrail::StarRailScanInfo {
        match self {
            ScanInfo::StarRail(info) => info,
            _ => crate::error_and_quit!("ScanInfo is not starrail"),
        }
    }

    pub fn capture_window(&self) -> Result<RgbImage> {
        Rect {
            origin: self.origin,
            size: self.size,
        }
        .capture()
    }

    pub fn move_to(&mut self, pos: &Pos) {
        self.origin = pos.clone();
    }
}

impl DrawConfig for ScanInfo {
    fn draw_config(&self, image: &mut image::RgbImage) {
        match self {
            ScanInfo::Genshin(info) => info.draw_config(image),
            ScanInfo::StarRail(info) => info.draw_config(image),
        }
    }
}

impl Deref for ScanInfo {
    type Target = SharedScanInfo;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            ScanInfo::StarRail(info) => &info.shared,
            ScanInfo::Genshin(info) => &info.shared,
        }
    }
}

impl DerefMut for ScanInfo {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ScanInfo::StarRail(info) => &mut info.shared,
            ScanInfo::Genshin(info) => &mut info.shared,
        }
    }
}
