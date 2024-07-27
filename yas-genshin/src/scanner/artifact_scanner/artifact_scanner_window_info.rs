use yas::positioning::{Pos, Rect, Size};

#[derive(Clone, yas_derive::YasWindowInfo, Debug)]
pub struct ArtifactScannerWindowInfo {
    /// the position of artifact title relative to window
    #[window_info(rename = "genshin_artifact_title_rect")]
    pub title_rect: Rect<f64>,

    /// the main stat name position of artifact relative to window
    #[window_info(rename = "genshin_artifact_main_stat_name_rect")]
    pub main_stat_name_rect: Rect<f64>,

    /// the main stat value position of artifact relative to window
    #[window_info(rename = "genshin_artifact_main_stat_value_rect")]
    pub main_stat_value_rect: Rect<f64>,

    /// the sub stats positions relative to window
    #[window_info(rename = "genshin_artifact_sub_stat1_rect")]
    pub sub_stat_1: Rect<f64>,
    #[window_info(rename = "genshin_artifact_sub_stat2_rect")]
    pub sub_stat_2: Rect<f64>,
    #[window_info(rename = "genshin_artifact_sub_stat3_rect")]
    pub sub_stat_3: Rect<f64>,
    #[window_info(rename = "genshin_artifact_sub_stat4_rect")]
    pub sub_stat_4: Rect<f64>,

    /// the level of the artifact relative to window
    #[window_info(rename = "genshin_artifact_level_rect")]
    pub level_rect: Rect<f64>,

    /// equip status of the artifact relative to window
    #[window_info(rename = "genshin_artifact_item_equip_rect")]
    pub item_equip_rect: Rect<f64>,

    /// the count of artifacts relative to window
    #[window_info(rename = "genshin_artifact_item_count_rect")]
    pub item_count_rect: Rect<f64>,

    /// the sample position of star, relative to window
    #[window_info(rename = "genshin_artifact_star_pos")]
    pub star_pos: Pos<f64>,

    /// the whole panel of the artifact, relative to window
    #[window_info(rename = "genshin_repository_panel_rect")]
    pub panel_rect: Rect<f64>,

    /// how many columns in this layout
    #[window_info(rename = "genshin_repository_item_col")]
    pub col: i32,

    /// how many rows in this layout
    #[window_info(rename = "genshin_repository_item_row")]
    pub row: i32,

    #[window_info(rename = "genshin_repository_item_gap_size")]
    pub item_gap_size: Size<f64>,

    #[window_info(rename = "genshin_repository_item_size")]
    pub item_size: Size<f64>,

    #[window_info(rename = "genshin_repository_scan_margin_pos")]
    pub scan_margin_pos: Pos<f64>,

    #[window_info(rename = "genshin_repository_lock_pos")]
    pub lock_pos: Pos<f64>,
}
