use yas::positioning::{Pos, Rect};
use yas_derive::YasWindowInfo;

#[derive(YasWindowInfo, Debug, Clone)]
pub struct EchoScannerWindowInfo {
    #[window_info(rename = "ww_echo_title_rect")]
    pub title_rect: Rect<f64>,

    #[window_info(rename = "ww_echo_main_stat1_name_rect")]
    pub main_stat1_name_rect: Rect<f64>,
    #[window_info(rename = "ww_echo_main_stat1_value_rect")]
    pub main_stat1_value_rect: Rect<f64>,
    #[window_info(rename = "ww_echo_main_stat2_name_rect")]
    pub main_stat2_name_rect: Rect<f64>,
    #[window_info(rename = "ww_echo_main_stat2_value_rect")]
    pub main_stat2_value_rect: Rect<f64>,

    // the sub stat name positions relative to window
    #[window_info(rename = "ww_echo_sub_stat0_name_rect")]
    pub sub_stat_name_1: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat1_name_rect")]
    pub sub_stat_name_2: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat2_name_rect")]
    pub sub_stat_name_3: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat3_name_rect")]
    pub sub_stat_name_4: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat4_name_rect")]
    pub sub_stat_name_5: Rect<f64>,

    // the sub stat value positions relative to window
    #[window_info(rename = "ww_echo_sub_stat0_value_rect")]
    pub sub_stat_value_1: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat1_value_rect")]
    pub sub_stat_value_2: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat2_value_rect")]
    pub sub_stat_value_3: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat3_value_rect")]
    pub sub_stat_value_4: Rect<f64>,
    #[window_info(rename = "ww_echo_sub_stat5_value_rect")]
    pub sub_stat_value_5: Rect<f64>,

    #[window_info(rename = "ww_echo_level_rect")]
    pub level_rect: Rect<f64>,

    // #[window_info(rename = "ww_echo_equip_rect")]
    // pub equip_rect: Rect<f64>,

    #[window_info(rename = "ww_echo_item_count_rect")]
    pub item_count_rect: Rect<f64>,

    #[window_info(rename = "ww_echo_star_pos")]
    pub star_pos: Pos<f64>,

    // #[window_info(rename = "ww_echo_lock_pos")]
    // pub lock_pos: Pos<f64>,

    #[window_info(rename = "starrail_repository_panel_rect")]
    pub panel_rect: Rect<f64>,

    #[window_info(rename = "starrail_repository_item_col")]
    pub col: i32,
}