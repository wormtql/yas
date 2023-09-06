use super::*;

pub mod genshin;
pub mod starrail;

type R = RectBound<WindowInfoType>;
type P = Pos<WindowInfoType>;
type S = Size<WindowInfoType>;

#[derive(Clone, Debug)]
pub enum WindowInfo {
    StarRail(starrail::StarRailWindowInfo),
    Genshin(genshin::GenshinWindowInfo),
}

#[derive(Clone, Debug)]
pub enum ScanInfo {
    StarRail(starrail::StarRailScanInfo),
    Genshin(genshin::GenshinScanInfo),
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
    name: String,
    main_stat_name: String,
    main_stat_value: String,
    sub_stat: [String; 4],
    level: String,
    equip: String,
    star: u32,
}
