use crate::common::*;
use std::ops::Deref;
pub mod ui;
pub use ui::*;

mod config;
pub use config::*;

mod convert;
pub use convert::*;

mod game;
pub use game::*;

pub mod inference;
pub mod scanner;

crate::scan_info_convert! {
    pub type ScanInfoType = u32;
    pub type WindowInfoType = f64;

    #[derive(Clone, Debug)]
    pub struct SharedScanInfo<T = ScanInfoType> {
        // pub width: u32,
        // pub height: u32,
        pub size: Size<T>,

        // pub left: i32,
        // pub top: i32,
        pub origin: Pos<T>,

        pub title_pos: RectBound<T>,
        pub main_stat_name_pos: RectBound<T>,
        pub main_stat_value_pos: RectBound<T>,

        pub level_pos: RectBound<T>,
        pub panel_pos: RectBound<T>,

        pub item_equip_pos: RectBound<T>,
        pub item_count_pos: RectBound<T>,

        // pub item_width: u32,
        // pub item_height: u32,
        pub item_size: Size<T>,

        pub item_row: usize,
        pub item_col: usize,
        pub item_gap: Size<T>,

        // pub left_margin: u32,
        // pub top_margin: u32,
        pub scan_margin: Size<T>,

        // pub flag_x: u32,
        // pub flag_y: u32,
        pub flag: Pos<T>,

        // pub star_x: u32,
        // pub star_y: u32,
        pub star: Pos<T>,

        pub pool_pos: RectBound<T>,
    }

}

pub type SharedWindowInfo = SharedScanInfo<WindowInfoType>;

impl SharedScanInfo<f64> {
    pub fn get_radio(&self, size: Size<f64>) -> (f64, f64) {
        (size.width / self.size.width, size.height / self.size.height)
    }

    pub fn move_to(&mut self, pos: Pos<f64>) {
        self.origin = pos;
    }
}
