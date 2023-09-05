use super::*;

#[derive(Clone, Debug)]
pub struct GenshinScanInfo<T = ScanInfoType> {
    pub shared: SharedScanInfo<T>,

    pub sub_stat_pos: [RectBound<T>; 4],
}

pub type GenshinWindowInfo = GenshinScanInfo<WindowInfoType>;

impl ConvertToScanInfo<GenshinScanInfo> for GenshinWindowInfo {
    fn to_scan_info(&self, size: Size<f64>) -> GenshinScanInfo {
        let radio = self.shared.get_radio(size);

        GenshinScanInfo {
            shared: self.shared.to_scan_info(size),

            sub_stat_pos: [
                self.sub_stat_pos[0].scale_to_scan(radio),
                self.sub_stat_pos[1].scale_to_scan(radio),
                self.sub_stat_pos[2].scale_to_scan(radio),
                self.sub_stat_pos[3].scale_to_scan(radio),
            ],
        }
    }
}

// impl ScanInfoConvert for GenshinScanInfo {
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
//             // 窗口状态下的 playcover 分辨率长宽无法整除
//             WINDOW_MAC_8_5.to_scan_info(height as f64, width as f64, left, top)
//         } else {
//             // 不支持的分辨率
//             panic!("不支持的分辨率");
//         }
//     }
// }
