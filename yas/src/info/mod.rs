mod convert;
pub use convert::*;

mod const_info;

mod genshin;
mod starrail;

use crate::common::*;

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
        (size.width / self.size.width, size.height/ self.size.height)
    }

    pub fn move_to(&mut self, pos: Pos<f64>) {
        self.origin = pos;
    }
}

pub enum WindowInfo {
    StarRail(starrail::StarRailWindowInfo),
    Genshin(genshin::GenshinWindowInfo),
}

pub trait ScanInfoConvert {
    fn from_pc(width: u32, height: u32, left: i32, top: i32) -> Self;
    fn from_mobile(width: u32, height: u32, left: i32, top: i32) -> Self;
}

// impl ScanInfo for SharedScanInfo {
//     fn from_pc(width: u32, height: u32, left: i32, top: i32) -> SharedScanInfo {
//         if height * 43 == width * 18 {
//             WINDOW_43_18.to_scan_info(height as f64, width as f64, left, top)
//         } else if height * 16 == width * 9 {
//             WINDOW_16_9.to_scan_info(height as f64, width as f64, left, top)
//         } else if height * 8 == width * 5 {
//             WINDOW_8_5.to_scan_info(height as f64, width as f64, left, top)
//         } else if height * 4 == width * 3 {
//             WINDOW_4_3.to_scan_info(height as f64, width as f64, left, top)
//         } else if height * 7 == width * 3 {
//             WINDOW_7_3.to_scan_info(height as f64, width as f64, left, top)
//         } else {
//             // 不支持的分辨率
//             panic!("不支持的分辨率");
//         }
//     }

//     fn from_mobile(width: u32, height: u32, left: i32, top: i32) -> SharedScanInfo {
//         if (height as i32 * 8 - width as i32 * 5).abs() < 20 {
//             // 窗口状态下的 playcover 分辨率长宽无法整除
//             WINDOW_MAC_8_5.to_scan_info(height as f64, width as f64, left, top)
//         } else {
//             // 不支持的分辨率
//             panic!("不支持的分辨率");
//         }
//     }
// }
