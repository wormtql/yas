use super::ui::Resolution;
use super::*;

pub fn get_window_info(size: Resolution) -> &'static GenshinWindowInfo {
    match size {
        Resolution::Windows43x18 => &WINDOW_43_18,
        Resolution::WIndows7x3 => &WINDOW_7_3,
        Resolution::Windows16x9 => &WINDOW_16_9,
        Resolution::Windows8x5 => &WINDOW_8_5,
        Resolution::Windows4x3 => &WINDOW_4_3,
        Resolution::MacOS8x5 => &MACOS_8_5,
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
    },
};

pub const WINDOW_7_3: GenshinWindowInfo = GenshinWindowInfo {
    sub_stat_pos: [
        R::new(398.1, 1780.0, 427.3, 1570.0),
        R::new(427.3, 1780.0, 458.2, 1570.0),
        R::new(458.2, 1780.0, 490.9, 1570.0),
        R::new(490.9, 1780.0, 523.0, 1570.0),
    ],

    shared: SharedScanInfo {
        size: S::new(2100.0, 900.0),
        origin: P::new(0.0, 0.0),

        title_pos: R::new(106.6, 1800.0, 139.6, 1550.0),
        main_stat_name_pos: R::new(224.3, 1690.0, 248.0, 1550.0),
        main_stat_value_pos: R::new(248.4, 1690.0, 286.8, 1550.0),
        level_pos: R::new(360.0, 1600.0, 378.0, 1557.0),
        panel_pos: R::new(100.0, 1941.0, 800.0, 1531.0),

        item_equip_pos: R::new(762.6, 1850.0, 787.8, 1598.0),
        item_count_pos: R::new(27.1, 1945.0, 52.9, 1785.0),

        item_row: 5,
        item_col: 11,

        item_size: S::new(1055.0 - 953.0, 373.0 - 247.0),
        item_gap: S::new(953.0 - 933.0, 247.0 - 227.0),

        scan_margin: S::new(166.0, 101.0),

        flag: P::new(340.0, 89.8),
        star: P::new(1900.0, 123.9),

        pool_pos: R::new(118.2, 1584.0 + 15.0, 510.3, 1584.0),
    },
};

pub const WINDOW_16_9: GenshinWindowInfo = GenshinWindowInfo {
    sub_stat_pos: [
        R::new(398.1, 1343.0, 427.3, 1130.2),
        R::new(427.3, 1343.0, 458.2, 1130.2),
        R::new(458.2, 1343.0, 490.9, 1130.2),
        R::new(490.9, 1343.0, 523.0, 1130.2),
    ],

    shared: SharedScanInfo {
        size: S::new(1600.0, 900.0),
        origin: P::new(0.0, 0.0),

        // top right bottom left
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
    },
};

pub const WINDOW_8_5: GenshinWindowInfo = GenshinWindowInfo {
    sub_stat_pos: [
        R::new(358.0, 1224.1, 384.1, 1016.2),
        R::new(384.1, 1224.1, 412.6, 1016.2),
        R::new(412.6, 1224.1, 440.5, 1016.2),
        R::new(440.5, 1224.1, 467.1, 1016.2),
    ],

    shared: SharedScanInfo {
        size: S::new(1440.0, 900.0),
        origin: P::new(0.0, 0.0),

        title_pos: R::new(96.0, 1268.9, 126.1, 1000.9),
        main_stat_name_pos: R::new(201.6, 1128.1, 223.9, 1000.3),
        main_stat_value_pos: R::new(225.5, 1128.1, 262.8, 1000.3),
        level_pos: R::new(324.0, 1043.0, 340.0, 1006.0),
        panel_pos: R::new(90.0, 1350.0, 810.0, 981.0),

        item_equip_pos: R::new(776.0, 1247.3, 800.6, 1041.3),
        item_count_pos: R::new(25.0, 1353.1, 46.8, 1182.8),

        item_row: 6,
        item_col: 8,

        item_size: S::new(950.0 - 857.0, 204.0 - 91.0),
        item_gap: S::new(857.0 - 840.0, 222.0 - 204.0),

        scan_margin: S::new(89.0, 91.0),

        flag: P::new(245.9, 82.1),
        star: P::new(1321.3, 111.3),

        pool_pos: R::new(103.6, 1025.8 + 15.0, 460.7, 1028.5),
    },
};

pub const WINDOW_4_3: GenshinWindowInfo = GenshinWindowInfo {
    //     sub_stat1_pos: Rect(318.2, 1100.5, 342.3, 904.3),
    //     sub_stat2_pos: Rect(342.3, 1100.5, 369.4, 904.3),
    //     sub_stat3_pos: Rect(369.4, 1100.5, 395.3, 904.3),
    //     sub_stat4_pos: Rect(395.3, 1100.5, 420.6, 904.3),
    sub_stat_pos: [
        R::new(318.2, 1100.5, 342.3, 904.3),
        R::new(342.3, 1100.5, 369.4, 904.3),
        R::new(369.4, 1100.5, 395.3, 904.3),
        R::new(395.3, 1100.5, 420.6, 904.3),
    ],

    shared: SharedScanInfo {
        size: S::new(1280.0, 960.0),
        origin: P::new(0.0, 0.0),

        title_pos: R::new(85.0, 1094.8, 111.7, 889.5),
        main_stat_name_pos: R::new(181.0, 998.0, 199.8, 889.5),
        main_stat_value_pos: R::new(199.8, 998.0, 233.4, 889.5),
        level_pos: R::new(288.0, 927.0, 302.0, 894.0),
        panel_pos: R::new(80.0, 1200.0, 880.0, 872.0),

        item_equip_pos: R::new(849.8, 1090.8, 870.1, 924.4),
        item_count_pos: R::new(22.9, 1202.3, 41.4, 1058.6),

        item_row: 7,
        item_col: 8,

        item_size: S::new(844.0 - 762.0, 182.0 - 81.0),
        item_gap: S::new(762.0 - 747.0, 197.0 - 182.0),

        scan_margin: S::new(79.0, 81.0),

        flag: P::new(218.1, 72.1),
        star: P::new(1175.4, 95.8),

        pool_pos: R::new(93.2, 912.7 + 15.0, 412.4, 912.7),
    },
};

pub const MACOS_8_5: GenshinWindowInfo = GenshinWindowInfo {
    sub_stat_pos: [
        R::new(387.0 - 28.0, 1050.0, 417.0 - 28.0, 791.0),
        R::new(417.0 - 28.0, 1050.0, 446.0 - 28.0, 791.0),
        R::new(446.0 - 28.0, 1050.0, 475.0 - 28.0, 791.0),
        R::new(475.0 - 28.0, 1050.0, 504.0 - 28.0, 791.0),
    ],

    shared: SharedScanInfo {
        size: S::new(1164.0, 755.0 - 28.),
        origin: P::new(0.0, 0.0),

        title_pos: R::new(122.0 - 28., 1090.0, 157.0 - 28., 770.0),
        main_stat_name_pos: R::new(230.0 - 28., 925.0, 254.0 - 28., 765.0),
        main_stat_value_pos: R::new(253.0 - 28., 911.0, 294.0 - 28., 767.0),
        level_pos: R::new(353.0 - 28., 813.0, 372.0 - 28., 781.0),
        panel_pos: R::new(117.0 - 28., 1127.0, 666.0 - 28., 756.0),

        item_equip_pos: R::new(627.0 - 28., 1090.0, 659.0 - 28., 815.0),
        item_count_pos: R::new(51.0 - 28., 1076.0, 80.0 - 28., 924.0),

        item_row: 4,
        item_col: 5,

        item_size: S::new(250.0 - 155.0, 234.0 - 118.0),
        item_gap: S::new(266.0 - 250.0, 250.0 - 234.0),

        scan_margin: S::new(155.0, 118.0 - 28.0),

        flag: P::new(240.0, 223.0 - 32.0),
        star: P::new(1060.0, 140.0 - 28.0),

        pool_pos: R::new(390.0 - 28.0, 1010.0, 504.0 - 28.0, 792.0),
    },
};
