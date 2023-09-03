use crate::common::{PixelRect, PixelRectBound};
use crate::info::window_info::{
    WINDOW_16_9, WINDOW_43_18, WINDOW_4_3, WINDOW_7_3, WINDOW_8_5, WINDOW_MAC_8_5,
};

use crate::info::window_info_starrail::{
    WINDOW_43_18_STARRAIL, WINDOW_16_9_STARRAIL, WINDOW_8_5_STARRAIL, WINDOW_4_3_STARRAIL, WINDOW_7_3_STARRAIL, WINDOW_MAC_8_5_STARRAIL
};

#[derive(Clone, Debug)]
pub struct ScanInfo {
    // pub panel_height: u32,
    // pub panel_width: u32,

    // pub panel_position: PixelRectBound,
    pub title_position: PixelRectBound,
    pub main_stat_name_position: PixelRectBound,
    pub main_stat_value_position: PixelRectBound,
    pub level_position: PixelRectBound,
    pub panel_position: PixelRectBound,

    pub sub_stat1_position: PixelRectBound,
    pub sub_stat2_position: PixelRectBound,
    pub sub_stat3_position: PixelRectBound,
    pub sub_stat4_position: PixelRectBound,

    pub sub_stat1_name_pos: PixelRectBound,
    pub sub_stat1_value_pos: PixelRectBound,
    pub sub_stat2_name_pos: PixelRectBound,
    pub sub_stat2_value_pos: PixelRectBound,
    pub sub_stat3_name_pos: PixelRectBound,
    pub sub_stat3_value_pos: PixelRectBound,
    pub sub_stat4_name_pos: PixelRectBound,
    pub sub_stat4_value_pos: PixelRectBound,

    pub equip_position: PixelRectBound,
    pub art_count_position: PixelRectBound,

    pub art_width: u32,
    pub art_height: u32,
    pub art_gap_x: u32,
    pub art_gap_y: u32,

    pub art_row: u32,
    pub art_col: u32,

    pub left_margin: u32,
    pub top_margin: u32,

    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,

    pub flag_x: u32,
    pub flag_y: u32,

    pub star_x: u32,
    pub star_y: u32,

    pub pool_position: PixelRectBound,
}

impl ScanInfo {
    pub fn from_genshin(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        if height * 43 == width * 18 {
            WINDOW_43_18.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 16 == width * 9 {
            WINDOW_16_9.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 8 == width * 5 {
            WINDOW_8_5.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 4 == width * 3 {
            WINDOW_4_3.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 7 == width * 3 {
            WINDOW_7_3.to_scan_info(height as f64, width as f64, left, top)
        } else {
            // 不支持的分辨率
            panic!("不支持的分辨率");
        }
    }
    pub fn from_mobile_genshin(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        if (height as i32 * 8 - width as i32 * 5).abs() < 20 {
            // 窗口状态下的playcover分辨率长宽无法整除
            WINDOW_MAC_8_5.to_scan_info(height as f64, width as f64, left, top)
        } else {
            // 不支持的分辨率
            panic!("不支持的分辨率");            
        }
    }
    pub fn from_starrail(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        if height * 43 == width * 18 {
            WINDOW_43_18_STARRAIL.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 16 == width * 9 {
            WINDOW_16_9_STARRAIL.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 8 == width * 5 {
            WINDOW_8_5_STARRAIL.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 4 == width * 3 {
            WINDOW_4_3_STARRAIL.to_scan_info(height as f64, width as f64, left, top)
        } else if height * 7 == width * 3 {
            WINDOW_7_3_STARRAIL.to_scan_info(height as f64, width as f64, left, top)
        } else {
            // 不支持的分辨率
            panic!("不支持的分辨率");
        }
    }
    pub fn from_mobile_starrail(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        if (height as i32 * 8 - width as i32 * 5).abs() < 20 {
            // 窗口状态下的playcover分辨率长宽无法整除
            WINDOW_MAC_8_5_STARRAIL.to_scan_info(height as f64, width as f64, left, top)
        } else {
            // 不支持的分辨率
            panic!("不支持的分辨率");            
        }
    }
}

// impl ScanInfo {
//     pub fn from_rect(rect: &PixelRect) -> Result<ScanInfo, String> {
//         let info: ScanInfo;
//         if rect.height * 16 == rect.width * 9 {
//             info = ScanInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left, rect.top);
//         } else if rect.height * 8 == rect.width * 5 {
//             info = ScanInfo::from_8_5(rect.width as u32, rect.height as u32, rect.left, rect.top);
//         } else if rect.height * 4 == rect.width * 3 {
//             info = ScanInfo::from_4_3(rect.width as u32, rect.height as u32, rect.left, rect.top);
//         } else if rect.height * 7 == rect.width * 3 {
//             info = ScanInfo::from_7_3(rect.width as u32, rect.height as u32, rect.left, rect.top);
//         } else if cfg!(target_os = "macos") {
//             info = ScanInfo::from_mobile_8_5(
//                 rect.width as u32,
//                 rect.height as u32,
//                 rect.left,
//                 rect.top,
//             );
//         } else {
//             return Err(String::from("不支持的分辨率"));
//         }

//         Ok(info)
//     }
// }
