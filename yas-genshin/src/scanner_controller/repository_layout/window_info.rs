use yas::positioning::{Pos, Rect, Size};
use yas_derive::YasWindowInfo;

// todo macro key renaming
#[derive(YasWindowInfo)]
pub struct GenshinRepositoryScanControllerWindowInfo {
    #[window_info(rename = "genshin")]
    pub panel_rect: Rect<f64>,
    pub flag_pos: Pos<f64>,
    pub item_gap_size: Size<f64>,
    pub item_size: Size<f64>,
    pub scan_margin_pos: Pos<f64>,
    pub pool_rect: Rect<f64>,

    pub genshin_repository_item_row: i32,
    pub genshin_repository_item_col: i32,
}
