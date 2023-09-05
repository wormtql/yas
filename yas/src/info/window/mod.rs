use crate::common::*;

mod genshin;
mod starrail;

pub type FloatRect = Rect<f64, f64>;

pub struct SharedWindowInfo {
    pub width: f64,
    pub height: f64,

    pub title_pos: FloatRect,
    pub main_stat_name_pos: FloatRect,
    pub main_stat_value_pos: FloatRect,

    pub level_pos: FloatRect,
    pub panel_pos: FloatRect,

    pub equip_pos: FloatRect,
    pub item_count_pos: FloatRect,

    pub item_width: f64,
    pub item_height: f64,
    pub item_gap_x: f64,
    pub item_gap_y: f64,

    pub item_row: usize,
    pub item_col: usize,

    pub left_margin: f64,
    pub top_margin: f64,

    pub flag_x: f64,
    pub flag_y: f64,

    pub star_x: f64,
    pub star_y: f64,

    pub pool_pos: FloatRect,
}
