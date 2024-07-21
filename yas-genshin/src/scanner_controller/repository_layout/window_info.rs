use yas::positioning::{Pos, Rect, Size};
use yas_derive::YasWindowInfo;

#[derive(Clone, YasWindowInfo)]
pub struct GenshinRepositoryScanControllerWindowInfo {
    #[window_info(rename = "genshin_repository_panel_rect")]
    pub panel_rect: Rect<f64>,

    #[window_info(rename = "genshin_repository_flag_pos")]
    pub flag_pos: Pos<f64>,

    #[window_info(rename = "genshin_repository_item_gap_size")]
    pub item_gap_size: Size<f64>,

    #[window_info(rename = "genshin_repository_item_size")]
    pub item_size: Size<f64>,

    #[window_info(rename = "genshin_repository_scan_margin_pos")]
    pub scan_margin_pos: Pos<f64>,

    #[window_info(rename = "genshin_repository_pool_rect")]
    pub pool_rect: Rect<f64>,

    pub genshin_repository_item_row: i32,
    pub genshin_repository_item_col: i32,
}
