mod genshin;
mod starrail;

use crate::common::RectBound;
// use crate::info::window::{
//     WINDOW_16_9, WINDOW_43_18, WINDOW_4_3, WINDOW_7_3, WINDOW_8_5, WINDOW_MAC_8_5,
// };

pub struct SharedScanInfo {
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,

    pub item_width: u32,
    pub item_height: u32,
    pub item_gap_x: u32,
    pub item_gap_y: u32,

    pub item_row: u32,
    pub item_col: u32,

    pub left_margin: u32,
    pub top_margin: u32,

    pub flag_x: u32,
    pub flag_y: u32,

    pub star_x: u32,
    pub star_y: u32,

    pub pool_pos: RectBound,
    pub item_count_pos: RectBound,
    pub item_equip_pos: RectBound,

    pub title_pos: RectBound,
    pub main_stat_name_pos: RectBound,
    pub main_stat_value_pos: RectBound,
    pub level_pos: RectBound,
    pub panel_pos: RectBound,
}

pub enum ScanInfo {
    StarRail(starrail::StarRailScanInfo),
    Genshin(genshin::GenshinScanInfo),
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
