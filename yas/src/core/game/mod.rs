use std::fmt::*;

use super::*;

pub mod genshin;
pub mod starrail;

type R = RectBound<WindowInfoType>;
type P = Pos<WindowInfoType>;
type S = Size<WindowInfoType>;

#[derive(Clone, Debug)]
pub enum WindowInfo {
    Genshin(genshin::GenshinWindowInfo),
    StarRail(starrail::StarRailWindowInfo),
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

#[derive(Debug)]
pub struct ScanResult {
    pub name: String,
    pub main_stat_name: String,
    pub main_stat_value: String,
    pub sub_stat: [String; 4],
    pub level: String,
    pub equip: String,
    pub star: u32,
}

#[derive(Debug)]
pub enum Game {
    Genshin,
    StarRail,
    Unknown,
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
