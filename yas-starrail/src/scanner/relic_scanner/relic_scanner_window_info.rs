use yas::positioning::{Pos, Rect};

#[derive(Clone, yas_derive::YasWindowInfo, Debug)]
pub struct RelicScannerWindowInfo {
    #[window_info(rename = "starrail_relic_title_rect")]
    pub title_rect: Rect<f64>,

    #[window_info(rename = "starrail_relic_main_stat_name_rect")]
    pub main_stat_name_rect: Rect<f64>,

    #[window_info(rename = "starrail_relic_main_stat_value_rect")]
    pub main_stat_value_rect: Rect<f64>,

    /// the sub stat name positions relative to window
    #[window_info(rename = "starrail_relic_sub_stat0_name_rect")]
    pub sub_stat_name_1: Rect<f64>,
    #[window_info(rename = "starrail_relic_sub_stat1_name_rect")]
    pub sub_stat_name_2: Rect<f64>,
    #[window_info(rename = "starrail_relic_sub_stat2_name_rect")]
    pub sub_stat_name_3: Rect<f64>,
    #[window_info(rename = "starrail_relic_sub_stat3_name_rect")]
    pub sub_stat_name_4: Rect<f64>,

    /// the sub stat value positions relative to window
    #[window_info(rename = "starrail_relic_sub_stat0_value_rect")]
    pub sub_stat_value_1: Rect<f64>,
    #[window_info(rename = "starrail_relic_sub_stat1_value_rect")]
    pub sub_stat_value_2: Rect<f64>,
    #[window_info(rename = "starrail_relic_sub_stat2_value_rect")]
    pub sub_stat_value_3: Rect<f64>,
    #[window_info(rename = "starrail_relic_sub_stat3_value_rect")]
    pub sub_stat_value_4: Rect<f64>,

    #[window_info(rename = "starrail_relic_level_rect")]
    pub level_rect: Rect<f64>,

    #[window_info(rename = "starrail_relic_equip_rect")]
    pub equip_rect: Rect<f64>,

    #[window_info(rename = "starrail_relic_equipper_pos")]
    pub equipper_pos: Pos<f64>,

    #[window_info(rename = "starrail_relic_item_count_rect")]
    pub item_count_rect: Rect<f64>,

    #[window_info(rename = "starrail_relic_star_pos")]
    pub star_pos: Pos<f64>,

    #[window_info(rename = "starrail_relic_lock_pos")]
    pub lock_pos: Pos<f64>,

    #[window_info(rename = "starrail_relic_discard_pos")]
    pub discard_pos: Pos<f64>,

    #[window_info(rename = "starrail_repository_panel_rect")]
    pub panel_rect: Rect<f64>,

    #[window_info(rename = "starrail_repository_item_col")]
    pub col: i32,
}
