use super::{scanner::{ScannerCore, ItemScanner}, *};
use std::{fmt::*, ops::DerefMut};

pub mod genshin;
pub mod starrail;

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

pub enum Scanner {
    Genshin(genshin::YasGenshinScanner),
    StarRail(starrail::YasStarRailScanner),
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

pub fn parse_level(str_level: &str) -> u32 {
    let pos = str_level.find('+');

    if pos.is_none() {
        return str_level.parse::<u32>().unwrap();
    }

    str_level[0..pos.unwrap()].parse::<u32>().unwrap()
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

impl Scanner {
    pub fn new(
        scan_info: ScanInfo,
        config: YasScannerConfig,
        game_info: GameInfo,
        model: &[u8],
        content: String,
    ) -> Self {
        let core = ScannerCore::new(scan_info, config, game_info, model, content);

        match crate::TARGET_GAME.get().unwrap() {
            Game::Genshin => Scanner::Genshin(genshin::YasGenshinScanner(core)),
            Game::StarRail => Scanner::StarRail(starrail::YasStarRailScanner(core)),
            _ => crate::error_and_quit!("不支持的游戏类型"),
        }
    }

    pub fn data(&self) -> &ScannerCore {
        match self {
            Scanner::Genshin(scanner) => &scanner.0,
            Scanner::StarRail(scanner) => &scanner.0,
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

impl Deref for Scanner {
    type Target = dyn ItemScanner;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Scanner::Genshin(scanner) => scanner,
            Scanner::StarRail(scanner) => scanner,
        }
    }
}


impl DerefMut for Scanner {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Scanner::Genshin(scanner) => scanner,
            Scanner::StarRail(scanner) => scanner,
        }
    }
}
