use crate::info::genshin::GenshinWindowInfo;

use super::*;

pub fn get_window_info(size: WindowSize) -> &'static GenshinWindowInfo {
    match size {
        WindowSize::Windows43x18 => &WINDOW_43_18,
        // WindowSize::WIndows7x3 => &WINDOW_7_3,
        WindowSize::Windows16x9 => &WINDOW_16_9,
        // WindowSize::Windows8x5 => &WINDOW_8_5,
        // WindowSize::Windows4x3 => &WINDOW_4_3,
        // WindowSize::MacOS8x5 => &WINDOW_MAC_8_5,
        _ => unimplemented!()
    }
}

pub const WINDOW_43_18: GenshinWindowInfo = GenshinWindowInfo {
    sub_stat_pos: [
        R::new(640.0, 3080.0, 680.0, 2590.0),
        R::new(690.0, 3080.0, 730.0, 2590.0),
        R::new(742.0, 3080.0, 782.0, 2590.0),
        R::new(795.0, 3080.0, 835.0, 2590.0),
    ],

    shared: SharedScanInfo {
        size: S::new(3440.0, 1440.0),
        origin: P::new(0.0, 0.0),

        title_pos: R::new(170.0, 3140.0, 220.0, 2560.0),
        main_stat_name_pos: R::new(360.0, 2850.0, 400.0, 2560.0),
        main_stat_value_pos: R::new(400.0, 2850.0, 460.0, 2560.0),
        level_pos: R::new(575.0, 2640.0, 605.0, 2568.0),
        panel_pos: R::new(160.0, 3185.0, 1280.0, 2528.0),

        item_equip_pos: R::new(1220.0, 5630.0, 1260.0, 3140.0),
        item_count_pos: R::new(50.0, 3185.0, 85.0, 2750.0),

        item_row: 5,
        item_col: 11,

        item_size: S::new(2421.0 - 2257.0, 598.0 - 394.0),
        item_gap: S::new(2257.0 - 2225.0, 394.0 - 363.0),

        scan_margin: S::new(305.0, 161.0),
        flag: P::new(580.0, 145.0),
        star: P::new(3130.0, 200.0),

        pool_pos: R::new(170.0, 2610.0 + 30.0, 900.0, 2610.0),
    }
};

// pub const WINDOW_7_3: GenshinWindowInfo = GenshinWindowInfo {
//     width: 2100.0,
//     height: 900.0,

//     title_pos: Rect(106.6, 1800.0, 139.6, 1550.0),
//     main_stat_name_pos: Rect(224.3, 1690.0, 248.0, 1550.0),
//     main_stat_value_pos: Rect(248.4, 1690.0, 286.8, 1550.0),
//     level_pos: Rect(360.0, 1600.0, 378.0, 1557.0),
//     panel_pos: Rect(100.0, 1941.0, 800.0, 1531.0),

//     sub_stat1_pos: Rect(398.1, 1780.0, 427.3, 1570.0),
//     sub_stat2_pos: Rect(427.3, 1780.0, 458.2, 1570.0),
//     sub_stat3_pos: Rect(458.2, 1780.0, 490.9, 1570.0),
//     sub_stat4_pos: Rect(490.9, 1780.0, 523.0, 1570.0),

//     equip_pos: Rect(762.6, 1850.0, 787.8, 1598.0),
//     art_count_pos: Rect(27.1, 1945.0, 52.9, 1785.0),

//     art_width: 1055.0 - 953.0,
//     art_height: 373.0 - 247.0,
//     art_gap_x: 953.0 - 933.0,
//     art_gap_y: 247.0 - 227.0,

//     art_row: 5,
//     art_col: 11,

//     left_margin: 166.0,
//     top_margin: 101.0,

//     flag_x: 340.0,
//     flag_y: 89.8,

//     star_x: 1900.0,
//     star_y: 123.9,
//     pool_pos: Rect(118.2, 1584.0 + 15.0, 510.3, 1584.0),
// };

// pub const WINDOW_16_9: GenshinWindowInfo = GenshinWindowInfo {

// };

pub const WINDOW_16_9: GenshinWindowInfo = GenshinWindowInfo {
    //     sub_stat1_pos: Rect(398.1, 1343.0, 427.3, 1130.2),
    //     sub_stat2_pos: Rect(427.3, 1343.0, 458.2, 1130.2),
    //     sub_stat3_pos: Rect(458.2, 1343.0, 490.9, 1130.2),
    //     sub_stat4_pos: Rect(490.9, 1343.0, 523.0, 1130.2),
    
    sub_stat_pos: [
        R::new(398.1, 1343.0, 427.3, 1130.2),
        R::new(427.3, 1343.0, 458.2, 1130.2),
        R::new(458.2, 1343.0, 490.9, 1130.2),
        R::new(490.9, 1343.0, 523.0, 1130.2),
    ],

    //     width: 1600.0,
    //     height: 900.0,

    //     title_pos: Rect(106.6, 1417.7, 139.6, 1111.8),
    //     main_stat_name_pos: Rect(224.3, 1253.9, 248.0, 1110.0),
    //     main_stat_value_pos: Rect(248.4, 1246.8, 286.8, 1110.0),
    //     level_pos: Rect(360.0, 1160.0, 378.0, 1117.0),
    //     panel_pos: Rect(100.0, 1500.0, 800.0, 1090.0),

    //     equip_pos: Rect(762.6, 1389.4, 787.8, 1154.9),
    //     art_count_pos: Rect(27.1, 1504.7, 52.9, 1314.9),

    //     art_width: 1055.0 - 953.0,
    //     art_height: 373.0 - 247.0,
    //     art_gap_x: 953.0 - 933.0,
    //     art_gap_y: 247.0 - 227.0,

    //     art_row: 5,
    //     art_col: 8,

    //     left_margin: 99.0,
    //     top_margin: 101.0,

    //     flag_x: 271.1,
    //     flag_y: 89.8,

    //     star_x: 1469.4,
    //     star_y: 123.9,

    //     pool_pos: Rect(118.2, 1144.7 + 15.0, 510.3, 1144.7),

    shared: SharedScanInfo {
        size: S::new(1600.0, 900.0),
        origin: P::new(0.0, 0.0),

        title_pos: R::new(106.6, 1417.7, 139.6, 1111.8),
        main_stat_name_pos: R::new(224.3, 1253.9, 248.0, 1110.0),
        main_stat_value_pos: R::new(248.4, 1246.8, 286.8, 1110.0),
        level_pos: R::new(360.0, 1160.0, 378.0, 1117.0),
        panel_pos: R::new(100.0, 1500.0, 800.0, 1090.0),

        item_equip_pos: R::new(762.6, 1389.4, 787.8, 1154.9),
        item_count_pos: R::new(27.1, 1504.7, 52.9, 1314.9),

        item_row: 5,
        item_col: 8,

        item_size: S::new(1055.0 - 953.0, 373.0 - 247.0),
        item_gap: S::new(953.0 - 933.0, 247.0 - 227.0),

        scan_margin: S::new(99.0, 101.0),

        flag: P::new(271.1, 89.8),
        star: P::new(1469.4, 123.9),

        pool_pos: R::new(118.2, 1144.7 + 15.0, 510.3, 1144.7),
    }

};

// pub const WINDOW_8_5: GenshinWindowInfo = GenshinWindowInfo {
//     width: 1440.0,
//     height: 900.0,
//     title_pos: Rect(96.0, 1268.9, 126.1, 1000.9),
//     main_stat_name_pos: Rect(201.6, 1128.1, 223.9, 1000.3),
//     main_stat_value_pos: Rect(225.5, 1128.1, 262.8, 1000.3),
//     level_pos: Rect(324.0, 1043.0, 340.0, 1006.0),
//     panel_pos: Rect(90.0, 1350.0, 810.0, 981.0),
//     sub_stat1_pos: Rect(358.0, 1224.1, 384.1, 1016.2),
//     sub_stat2_pos: Rect(384.1, 1224.1, 412.6, 1016.2),
//     sub_stat3_pos: Rect(412.6, 1224.1, 440.5, 1016.2),
//     sub_stat4_pos: Rect(440.5, 1224.1, 467.1, 1016.2),
//     equip_pos: Rect(776.0, 1247.3, 800.6, 1041.3),
//     art_count_pos: Rect(25.0, 1353.1, 46.8, 1182.8),
//     art_width: 950.0 - 857.0,
//     art_height: 204.0 - 91.0,
//     art_gap_x: 857.0 - 840.0,
//     art_gap_y: 222.0 - 204.0,
//     art_row: 6,
//     art_col: 8,
//     left_margin: 89.0,
//     top_margin: 91.0,
//     flag_x: 245.9,
//     flag_y: 82.1,
//     star_x: 1321.3,
//     star_y: 111.3,
//     pool_pos: Rect(103.6, 1025.8 + 15.0, 460.7, 1028.5),
// };

// pub const WINDOW_4_3: GenshinWindowInfo = GenshinWindowInfo {
//     width: 1280.0,
//     height: 960.0,
//     title_pos: Rect(85.0, 1094.8, 111.7, 889.5),
//     main_stat_name_pos: Rect(181.0, 998.0, 199.8, 889.5),
//     main_stat_value_pos: Rect(199.8, 998.0, 233.4, 889.5),
//     level_pos: Rect(288.0, 927.0, 302.0, 894.0),
//     panel_pos: Rect(80.0, 1200.0, 880.0, 872.0),
//     sub_stat1_pos: Rect(318.2, 1100.5, 342.3, 904.3),
//     sub_stat2_pos: Rect(342.3, 1100.5, 369.4, 904.3),
//     sub_stat3_pos: Rect(369.4, 1100.5, 395.3, 904.3),
//     sub_stat4_pos: Rect(395.3, 1100.5, 420.6, 904.3),
//     equip_pos: Rect(849.8, 1090.8, 870.1, 924.4),
//     art_count_pos: Rect(22.9, 1202.3, 41.4, 1058.6),
//     art_width: 844.0 - 762.0,
//     art_height: 182.0 - 81.0,
//     art_gap_x: 762.0 - 747.0,
//     art_gap_y: 197.0 - 182.0,
//     art_row: 7,
//     art_col: 8,
//     left_margin: 79.0,
//     top_margin: 81.0,
//     flag_x: 218.1,
//     flag_y: 72.1,
//     star_x: 1175.4,
//     star_y: 95.8,
//     pool_pos: Rect(93.2, 912.7 + 15.0, 412.4, 912.7),
// };

// //top, right, bottom, left
// pub const WINDOW_MAC_8_5: GenshinWindowInfo = GenshinWindowInfo {
//     width: 1164.0,
//     height: 755.0 - 28.,
//     title_pos: Rect(122.0 - 28., 1090.0, 157.0 - 28., 770.0),
//     main_stat_name_pos: Rect(230. - 28., 925., 254. - 28., 765.),
//     main_stat_value_pos: Rect(253. - 28., 911., 294. - 28., 767.),
//     level_pos: Rect(353. - 28., 813., 372. - 28., 781.),
//     panel_pos: Rect(117. - 28., 1127., 666. - 28., 756.),
//     sub_stat1_pos: Rect(387. - 28., 1050., 417. - 28., 791.),
//     sub_stat2_pos: Rect(417. - 28., 1050., 446. - 28., 791.),
//     sub_stat3_pos: Rect(446. - 28., 1050., 475. - 28., 791.),
//     sub_stat4_pos: Rect(475. - 28., 1050., 504. - 28., 791.),
//     equip_pos: Rect(627. - 28., 1090., 659. - 28., 815.),
//     art_count_pos: Rect(51. - 28., 1076., 80. - 28., 924.),
//     art_width: 250. - 155.,
//     art_height: 234. - 118.,
//     art_gap_x: 266. - 250.,
//     art_gap_y: 250. - 234.,
//     art_row: 4,
//     art_col: 5,
//     left_margin: 155.,
//     top_margin: 118. - 28.,
//     flag_x: 170., //检测颜色出现重复，则判定换行完成
//     flag_y: 223. - 28.,
//     star_x: 1060.,
//     star_y: 140. - 28.,
//     pool_pos: Rect(390. - 28., 1010., 504. - 28., 792.), //检测平均颜色是否相同，判断圣遗物有没有切换
// };
