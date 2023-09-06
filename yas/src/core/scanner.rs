use super::*;
use crate::common::{capture::Capturable, color::Color};
use crate::core::inference::CRNNModel;
use crate::TARGET_GAME;
use anyhow::Result;
use enigo::{Enigo, MouseControllable};
use image::RgbImage;
use std::sync::Arc;
use std::time::SystemTime;

#[cfg(target_os = "macos")]
use crate::common::utils::*;

pub struct ScannerCore {
    pool: f64,

    initial_color: Color,

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: u32,

    is_cloud: bool,

    pub row: usize,
    pub col: usize,

    pub model: Arc<CRNNModel>,
    pub scan_info: Arc<ScanInfo>,
    pub config: YasScannerConfig,

    pub enigo: Enigo,
}

pub trait ItemScanner {
    fn scan(&mut self) -> Result<Vec<ScanResult>>;
}

pub fn calc_pool(row: &Vec<u8>) -> f32 {
    let len = row.len() / 3;
    let mut pool: f32 = 0.0;

    for i in 0..len {
        pool += row[i * 3] as f32;
    }

    pool
}

impl ScannerCore {
    pub fn new(
        scan_info: ScanInfo,
        config: YasScannerConfig,
        game_info: GameInfo,
        model: &[u8],
        content: String,
    ) -> Self {
        let model = CRNNModel::new(model, content).expect("Failed to load model");
        let row = scan_info.item_row;
        let col = scan_info.item_col;

        Self {
            model: Arc::new(model),
            enigo: Enigo::new(),

            scan_info: Arc::new(scan_info),
            config,

            row,
            col,

            pool: 0.0,

            initial_color: Color::new(0, 0, 0),

            scrolled_rows: 0,
            avg_scroll_one_row: 0.0,

            avg_switch_time: 0.0,
            scanned_count: 0,

            is_cloud: game_info.is_cloud,
        }
    }

    fn get_flag_color(&self) -> Result<Color> {
        let target = &self.scan_info.flag + &self.scan_info.origin;

        capture::get_color(target)
    }

    pub fn align_row(&mut self) {
        #[cfg(target_os = "macos")]
        let (_, ui) = get_pid_and_ui();
        let mut count = 0;
        while count < 10 {
            if self.get_flag_color().unwrap() == self.initial_color {
                return;
            }

            #[cfg(windows)]
            self.enigo.mouse_scroll_y(-1);

            #[cfg(target_os = "linux")]
            self.enigo.mouse_scroll_y(1);

            #[cfg(target_os = "macos")]
            {
                match ui {
                    crate::common::UI::Desktop => {
                        self.enigo.mouse_scroll_y(-1);
                        utils::sleep(20);
                    },
                    crate::common::UI::Mobile => {
                        mac_scroll(&mut self.enigo, 1);
                    },
                }
            }

            utils::sleep(self.config.scroll_delay);
            count += 1;
        }
    }

    pub fn capture_panel(&mut self) -> Result<RgbImage> {
        let mut rect: Rect<u32, u32> = Rect::from(self.scan_info.panel_pos);
        rect += self.scan_info.origin;
        rect.capture()
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        // let left = info.left
        //     + (info.left_margin + (info.art_width + info.art_gap_x) * col + info.art_width / 2)
        //         as i32;
        // let top = info.top
        //     + (info.top_margin + (info.art_height + info.art_gap_y) * row + info.art_height / 4)
        //         as i32;
        let (row, col) = (row as u32, col as u32);
        let origin = &self.scan_info.origin;

        let gap = &self.scan_info.item_gap;
        let margin = &self.scan_info.scan_margin;
        let size = &self.scan_info.item_size;

        let left = origin.x + margin.width + (gap.width + size.width) * col + size.width / 2;
        let top = origin.y + margin.height + (gap.height + size.height) * row + size.height / 4;

        self.enigo.mouse_move_to(left as i32, top as i32);

        #[cfg(target_os = "macos")]
        utils::sleep(20);
    }

    pub fn sample_initial_color(&mut self) {
        self.initial_color = self.get_flag_color().unwrap();
    }

    pub fn scroll_one_row(&mut self) -> ScrollResult {
        #[cfg(target_os = "macos")]
        let (_, ui) = get_pid_and_ui();
        let mut state = 0;
        let mut count = 0;
        let max_scroll = 25;
        while count < max_scroll {
            if utils::is_rmb_down() {
                return ScrollResult::Interrupt;
            }

            #[cfg(windows)]
            self.enigo.mouse_scroll_y(-5);
            #[cfg(target_os = "linux")]
            self.enigo.mouse_scroll_y(1);
            #[cfg(target_os = "macos")]
            {
                match ui {
                    crate::common::UI::Desktop => {
                        self.enigo.mouse_scroll_y(-1);
                        utils::sleep(20);
                    },
                    crate::common::UI::Mobile => {
                        mac_scroll(&mut self.enigo, 1);
                    },
                }
            }
            utils::sleep(self.config.scroll_delay);
            count += 1;
            if let Ok(color) = self.get_flag_color() {
                if state == 0 && color != self.initial_color {
                    state = 1;
                } else if state == 1 && self.initial_color == color {
                    self.avg_scroll_one_row = (self.avg_scroll_one_row * self.scrolled_rows as f64
                        + count as f64)
                        / (self.scrolled_rows as f64 + 1.0);
                    info!("avg scroll/row: {}", self.avg_scroll_one_row);
                    self.scrolled_rows += 1;
                    return ScrollResult::Success;
                }
            } else {
                return ScrollResult::Failed;
            }
        }

        ScrollResult::TimeLimitExceeded
    }

    pub fn scroll_rows(&mut self, count: usize) -> ScrollResult {
        #[cfg(target_os = "macos")]
        let (_, ui) = get_pid_and_ui();
        if self.scrolled_rows >= 5 {
            let scroll = ((self.avg_scroll_one_row * count as f64 - 3.0).round() as u32).max(0);
            for _ in 0..scroll {
                #[cfg(windows)]
                self.enigo.mouse_scroll_y(-1);
                #[cfg(target_os = "linux")]
                self.enigo.mouse_scroll_y(1);
                #[cfg(target_os = "macos")]
                {
                    match ui {
                        crate::common::UI::Desktop => {
                            // mac_scroll(&mut self.enigo, 1);
                            self.enigo.mouse_scroll_y(-1);
                            utils::sleep(20);
                        },
                        crate::common::UI::Mobile => {
                            mac_scroll(&mut self.enigo, 1);
                        },
                    }
                }
            }
            utils::sleep(400);
            self.align_row();
            return ScrollResult::Skip;
        }

        for _ in 0..count {
            match self.scroll_one_row() {
                ScrollResult::TimeLimitExceeded => return ScrollResult::TimeLimitExceeded,
                ScrollResult::Interrupt => return ScrollResult::Interrupt,
                _ => (),
            }
        }

        ScrollResult::Success
    }

    pub fn get_star(&self) -> u32 {
        let color = capture::get_color(&self.scan_info.origin + &self.scan_info.star).unwrap();

        let match_colors = [
            Color::new(113, 119, 139),
            Color::new(42, 143, 114),
            Color::new(81, 127, 203),
            Color::new(161, 86, 224),
            Color::new(188, 105, 50),
        ];

        let mut min_dis: u32 = 0xdeadbeef;
        let mut ret: usize = 1;
        for (i, match_color) in match_colors.iter().enumerate() {
            let dis = match_color.distance(&color);
            if dis < min_dis {
                min_dis = dis;
                ret = i + 1;
            }
        }

        ret as u32
    }

    pub fn get_item_count(&self) -> Result<u32> {
        let count = self.config.number;
        let item_name = match TARGET_GAME.get().unwrap() {
            Game::Genshin => "圣遗物",
            Game::StarRail => "遗器数量",
            _ => unimplemented!("不支持的游戏"),
        };

        if let 0 = count {
            let mut rect: Rect<u32, u32> = Rect::from(self.scan_info.item_count_pos);
            rect += self.scan_info.origin;

            if let Ok(s) = self.model.inference_string(&rect.capture()?) {
                info!("raw count string: {}", s);
                if s.starts_with(item_name) {
                    let chars = s.chars().collect::<Vec<char>>();
                    let count_str = chars[4..chars.len() - 5].iter().collect::<String>();
                    let count = match count_str.parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(anyhow::anyhow!("无法识别物品数量"));
                        },
                    };
                    return Ok(count);
                }
            }

            Err(anyhow::anyhow!("无法识别物品数量"))
        } else {
            Ok(count)
        }
    }

    pub fn wait_until_switched(&mut self) -> bool {
        if self.is_cloud {
            utils::sleep(self.config.cloud_wait_switch_item);
            return true;
        }
        let now = SystemTime::now();

        let mut consecutive_time = 0;
        let mut diff_flag = false;
        while now.elapsed().unwrap().as_millis() < self.config.max_wait_switch_item as u128 {
            // let pool_start = SystemTime::now();
            // let rect = Rect {
            //     left: self.info.left as i32 + self.info.pool_position.left,
            //     top: self.info.top as i32 + self.info.pool_position.top,
            //     width: self.info.pool_position.right - self.info.pool_position.left,
            //     height: self.info.pool_position.bottom - self.info.pool_position.top,
            // };
            // let im = capture::capture_absolute(&rect).unwrap();
            let rect: Rect<u32, u32> =
                &Rect::from(self.scan_info.pool_pos) + &self.scan_info.origin;
            let im: RgbImage = rect.capture().unwrap();

            let pool = calc_pool(im.as_raw()) as f64;
            // info!("pool: {}", pool);
            // println!("pool time: {}ms", pool_start.elapsed().unwrap().as_millis());

            if (pool - self.pool).abs() > 0.000001 {
                self.pool = pool;
                diff_flag = true;
                consecutive_time = 0;
            } else if diff_flag {
                consecutive_time += 1;
                if consecutive_time == 1 {
                    self.avg_switch_time = (self.avg_switch_time * self.scanned_count as f64
                        + now.elapsed().unwrap().as_millis() as f64)
                        / (self.scanned_count as f64 + 1.0);
                    self.scanned_count += 1;
                    return true;
                }
            }
        }

        false
    }
}
