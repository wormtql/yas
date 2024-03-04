use yas::common::positioning::{Pos, Rect};
use yas::window_info::WindowInfoRepository;

#[derive(Clone)]
pub struct ArtifactScannerWindowInfo {
    /// the left-top corner of the game window
    pub origin_pos: Pos,

    /// the position of artifact title relative to window
    pub title_rect: Rect,
    /// the main stat name position of artifact relative to window
    pub main_stat_name_rect: Rect,
    /// the main stat value position of artifact relative to window
    pub main_stat_value_rect: Rect,
    /// the sub stats positions relative to window
    pub sub_stat_rect: [Rect; 4],

    /// the level of the artifact relative to window
    pub level_rect: Rect,

    /// equip status of the artifact relative to window
    pub item_equip_rect: Rect,
    /// the count of artifacts relative to window
    pub item_count_rect: Rect,

    /// the sample position of star, relative to window
    pub star_pos: Pos,

    /// the whole panel of the artifact, relative to window
    pub panel_rect: Rect,

    /// how many columns in this layout
    pub col: i32,
}

impl From<&WindowInfoRepository> for ArtifactScannerWindowInfo {
    fn from(value: &WindowInfoRepository) -> Self {
        ArtifactScannerWindowInfo {
            origin_pos: value.get("window_origin_pos").unwrap(),
            title_rect: value.get("genshin_artifact_title_rect").unwrap(),
            main_stat_name_rect: value.get("genshin_artifact_main_stat_name_rect").unwrap(),
            main_stat_value_rect: value.get("genshin_artifact_main_stat_value_rect").unwrap(),
            level_rect: value.get("genshin_artifact_level_rect").unwrap(),
            item_equip_rect: value.get("genshin_artifact_item_equip_rect").unwrap(),
            item_count_rect: value.get("genshin_artifact_item_count_rect").unwrap(),
            star_pos: value.get("genshin_artifact_star_pos").unwrap(),

            panel_rect: value.get("genshin_repository_panel_rect").unwrap(),
            col: value.get("genshin_repository_item_col").unwrap(),

            sub_stat_rect: [
                value.get("genshin_artifact_sub_stat0_rect").unwrap(),
                value.get("genshin_artifact_sub_stat1_rect").unwrap(),
                value.get("genshin_artifact_sub_stat2_rect").unwrap(),
                value.get("genshin_artifact_sub_stat3_rect").unwrap(),
            ],
        }
    }
}