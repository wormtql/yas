use super::*;

#[derive(Clone, Debug)]
pub struct StarRailScanInfo<T = ScanInfoType> {
    pub shared: SharedScanInfo<T>,

    pub sub_stat_name_pos: [RectBound<T>; 4],
    pub sub_stat_value_pos: [RectBound<T>; 4],
}

pub type StarRailWindowInfo = StarRailScanInfo<WindowInfoType>;

impl ConvertToScanInfo<StarRailScanInfo> for StarRailWindowInfo {
    fn to_scan_info(&self, size: Size<f64>) -> StarRailScanInfo {
        let radio = self.shared.get_radio(size);

        StarRailScanInfo {
            shared: self.shared.to_scan_info(size),

            sub_stat_name_pos: [
                self.sub_stat_name_pos[0].scale_to_scan(radio),
                self.sub_stat_name_pos[1].scale_to_scan(radio),
                self.sub_stat_name_pos[2].scale_to_scan(radio),
                self.sub_stat_name_pos[3].scale_to_scan(radio),
            ],

            sub_stat_value_pos: [
                self.sub_stat_value_pos[0].scale_to_scan(radio),
                self.sub_stat_value_pos[1].scale_to_scan(radio),
                self.sub_stat_value_pos[2].scale_to_scan(radio),
                self.sub_stat_value_pos[3].scale_to_scan(radio),
            ],
        }
    }
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
