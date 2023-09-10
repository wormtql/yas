use yas::common::positioning::{Pos, Rect, Size};

#[derive(Clone, Debug)]
pub struct GenshinRepositoryScanInfo {
    // window origin
    pub origin: Pos<i32>,
    // window size
    pub size: Size<i32>,

    pub item_size: Size<u32>,

    pub item_row: u32,
    pub item_col: u32,
    pub item_gap: Size<u32>,

    pub scan_margin: Size<i32>,

    pub flag: Pos<u32>,

    pub pool_pos: Rect<i32>,

    pub panel_pos: RectBound<i32>,
}