use crate::scanner_controller::repository_layout::scan_info::GenshinRepositoryScanInfo;

#[derive(Clone, Debug)]
pub struct GenshinArtifactScannerScanInfo {
    pub genshin_repo_scan_info: GenshinRepositoryScanInfo,

    pub title_pos: RectBound<i32>,
    pub main_stat_name_pos: RectBound<i32>,
    pub main_stat_value_pos: RectBound<i32>,

    pub level_pos: RectBound<i32>,

    pub item_equip_pos: RectBound<i32>,
    pub item_count_pos: RectBound<i32>,

    pub star: Pos<T>,
}