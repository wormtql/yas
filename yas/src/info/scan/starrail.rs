use crate::common::RectBound;

use super::SharedScanInfo;
// use crate::info::window_info_starrail::{
//     WINDOW_43_18, WINDOW_16_9, WINDOW_8_5, WINDOW_4_3, WINDOW_7_3, WINDOW_MAC_8_5
// };

#[derive(Clone, Debug)]
pub struct StarRailScanInfo {
    pub shared: SharedScanInfo,

    pub sub_stat1_name_pos: RectBound,
    pub sub_stat1_value_pos: RectBound,

    pub sub_stat2_name_pos: RectBound,
    pub sub_stat2_value_pos: RectBound,

    pub sub_stat3_name_pos: RectBound,
    pub sub_stat3_value_pos: RectBound,

    pub sub_stat4_name_pos: RectBound,
    pub sub_stat4_value_pos: RectBound,
}

// impl ScanInfoConvert for StarRailScanInfo {
//     fn from_pc(width: u32, height: u32, left: i32, top: i32) -> Self {
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

//     fn from_mobile(width: u32, height: u32, left: i32, top: i32) -> Self {
//         if (height as i32 * 8 - width as i32 * 5).abs() < 20 {
//             // 窗口状态下的playcover分辨率长宽无法整除
//             WINDOW_MAC_8_5.to_scan_info(height as f64, width as f64, left, top)
//         } else {
//             // 不支持的分辨率
//             panic!("不支持的分辨率");
//         }
//     }
// }
