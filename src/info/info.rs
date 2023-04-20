use crate::common::{PixelRect, PixelRectBound};
use crate::info::window_info::{WINDOW_43_18, WINDOW_7_3, WINDOW_16_9, WINDOW_4_3, WINDOW_8_5};

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
    pub fn from_pixel_rect(rect: PixelRect) -> Option<ScanInfo> {
        if rect.height * 43 == rect.width * 18 {
            Some(ScanInfo::from_43_18(rect.width as u32, rect.height as u32, rect.left, rect.top))
        } else if rect.height * 16 == rect.width * 9 {
            Some(ScanInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left, rect.top))
        } else if rect.height * 8 == rect.width * 5 {
            Some(ScanInfo::from_8_5(rect.width as u32, rect.height as u32, rect.left, rect.top))
        } else if rect.height * 4 == rect.width * 3 {
            Some(ScanInfo::from_4_3(rect.width as u32, rect.height as u32, rect.left, rect.top))
        } else if rect.height * 7 == rect.width * 3 {
            Some(ScanInfo::from_7_3(rect.width as u32, rect.height as u32, rect.left, rect.top))
        } else {
            None
        }
    }

    pub fn from_43_18(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        WINDOW_43_18.to_scan_info(height as f64, width as f64, left, top)
    }

    pub fn from_7_3(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        WINDOW_7_3.to_scan_info(height as f64, width as f64, left, top)
    }

    pub fn from_16_9(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        WINDOW_16_9.to_scan_info(height as f64, width as f64, left, top)
    }

    pub fn from_8_5(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        WINDOW_8_5.to_scan_info(height as f64, width as f64, left, top)
        // let w: u32 = 1440;
        // let h: u32 = 900;
        //
        // let my_get_rect = |rect: (u32, u32, u32, u32)| {
        //     get_rect(rect, h, w, height, width)
        // };
        //
        // let info = ScanInfo {
        //     // panel_height: get_scalar(700.0, w, width),
        //     // panel_width: get_scalar(410.0, h, height),
        //
        //     title_position: my_get_rect((990, 95, 1240, 125)),
        //     main_stat_name_position: my_get_rect((990, 194, 1105, 223)),
        //     main_stat_value_position: my_get_rect((990, 223, 1105, 262)),
        //     level_position: my_get_rect((993, 323, 1032, 340)),
        //     panel_position: my_get_rect((969, 90, 1338, 810)),
        //
        //     sub_stat1_position: my_get_rect((1006, 356, 1188, 383)),
        //     sub_stat2_position: my_get_rect((1006, 385, 1188, 411)),
        //     sub_stat3_position: my_get_rect((1006, 413, 1188, 439)),
        //     sub_stat4_position: my_get_rect((1006, 442, 1188, 467)),
        //
        //     equip_position: my_get_rect((1028, 777, 1189, 799)),
        //     art_count_position: my_get_rect((1173, 25, 1351, 45)),
        //
        //     art_width: get_scalar(92.0, w, width),
        //     art_height: get_scalar(115.0, h, height),
        //     art_gap_x: get_scalar(17.0, w, width),
        //     art_gap_y: get_scalar(17.0, h, height),
        //
        //     art_row: 6,
        //     art_col: 7,
        //
        //     left_margin: get_scalar(155.0, w, width),
        //     top_margin: get_scalar(90.0, h, height),
        //
        //     width,
        //     height,
        //     left,
        //     top,
        //
        //     flag_x: get_scalar(312.0, w, width),
        //     flag_y: get_scalar(87.0, h, height),
        //
        //     star_x: get_scalar(1310.0, w, width),
        //     star_y: get_scalar(111.0, h, height),
        //
        //     pool_position: my_get_rect((1081, 100, 1092, 408)),
        // };
        //
        // info
    }

    pub fn from_4_3(width: u32, height: u32, left: i32, top: i32) -> ScanInfo {
        WINDOW_4_3.to_scan_info(height as f64, width as f64, left, top)
        // let w: u32 = 1280;
        // let h: u32 = 960;
        //
        // let my_get_rect = |rect: (u32, u32, u32, u32)| {
        //     get_rect(rect, h, w, height, width)
        // };
        //
        // let info = ScanInfo {
        //     title_position: my_get_rect((880, 85, 1092, 110)),
        //     main_stat_name_position: my_get_rect((880, 175, 984, 200)),
        //     main_stat_value_position: my_get_rect((880, 200, 970, 233)),
        //     level_position: my_get_rect((883, 287, 916, 303)),
        //     panel_position: my_get_rect((862, 80, 1189, 879)),
        //
        //     sub_stat1_position: my_get_rect((894, 320, 1054, 339)),
        //     sub_stat2_position: my_get_rect((894, 345, 1054, 365)),
        //     sub_stat3_position: my_get_rect((894, 373, 1054, 392)),
        //     sub_stat4_position: my_get_rect((894, 398, 1054, 418)),
        //
        //     equip_position: my_get_rect((913, 850, 1057, 870)),
        //     art_count_position: my_get_rect((1057, 21, 1204, 41)),
        //
        //     art_width: get_scalar(82.0, w, width),
        //     art_height: get_scalar(102.0, h, height),
        //     art_gap_x: get_scalar(15.0, w, width),
        //     art_gap_y: get_scalar(15.0, h, height),
        //
        //     art_row: 7,
        //     art_col: 7,
        //
        //     left_margin: get_scalar(138.0, w, width),
        //     top_margin: get_scalar(80.0, h, height),
        //
        //     width,
        //     height,
        //     left,
        //     top,
        //
        //     flag_x: get_scalar(277.0, w, width),
        //     flag_y: get_scalar(77.0, h, height),
        //
        //     star_x: get_scalar(1162.0, w, width),
        //     star_y: get_scalar(100.0, h, height),
        //
        //     pool_position: my_get_rect((959, 95, 974, 365)),
        // };
        //
        // info
    }
}

impl ScanInfo {
    pub fn from_rect(rect: &PixelRect) -> Result<ScanInfo, String> {
        let mut info: ScanInfo;
        if rect.height * 16 == rect.width * 9 {
            info = ScanInfo::from_16_9(rect.width as u32, rect.height as u32, rect.left, rect.top);
        } else if rect.height * 8 == rect.width * 5 {
            info = ScanInfo::from_8_5(rect.width as u32, rect.height as u32, rect.left, rect.top);
        } else if rect.height * 4 == rect.width * 3 {
            info = ScanInfo::from_4_3(rect.width as u32, rect.height as u32, rect.left, rect.top);
        } else if rect.height * 7 == rect.width * 3 {
            info = ScanInfo::from_7_3(rect.width as u32, rect.height as u32, rect.left, rect.top);
        } else {
            return Err(String::from("不支持的分辨率"));
        }

        Ok(info)
    }
}
