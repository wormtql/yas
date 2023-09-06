pub mod os;

pub enum WindowSize {
    // PC
    Windows43x18,
    WIndows7x3,
    Windows16x9,
    Windows8x5,
    Windows4x3,
    // Mobile
    MacOS8x5,
}

// pub fn from_pc(size: Size, pos: Pos) {}

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
