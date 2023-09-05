use std::collections::HashSet;
use std::convert::From;
use std::fs;

use std::sync::{mpsc, Arc};
use std::thread;
use std::time::SystemTime;

use clap::Parser;
use enigo::*;
use anyhow::Result;
use image::{GenericImageView, RgbImage};
use log::{error, info, warn};

use crate::item::starrail_relic::{
    RelicSetName, RelicSlot, RelicStat, StarrailRelic,
};
use crate::capture::{self};
use crate::common::color::Color;
#[cfg(target_os = "macos")]
use crate::common::utils::get_pid_and_ui;
use crate::common::{utils, Rect, PixelRectBound};
use crate::inference::inference::CRNNModel;
use crate::inference::pre_process::{pre_process, to_gray, ImageConvExt};
use crate::info::starrail::ScanInfoStarRail;

// Playcover only, wine should not need this.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use crate::common::utils::mac_scroll;

use super::YasScannerConfig;


impl YasScanResult {
    pub fn to_internal_relic(&self) -> Option<StarrailRelic> {
        let set_name = RelicSetName::from_zh_cn(&self.name)?;
        let slot = RelicSlot::from_zh_cn(&self.name)?;
        let star = self.star;
        if !self.level.contains("+") {
            return None;
        }
        let level = self
            .level
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<u32>()
            .ok()?;
        let main_stat = RelicStat::from_zh_cn_raw(
            (self.main_stat_name.clone() + "+" + self.main_stat_value.as_str()).as_str(),
        )?;
        let sub1 = RelicStat::from_zh_cn_raw(&self.sub_stat_1);
        let sub2 = RelicStat::from_zh_cn_raw(&self.sub_stat_2);
        let sub3 = RelicStat::from_zh_cn_raw(&self.sub_stat_3);
        let sub4 = RelicStat::from_zh_cn_raw(&self.sub_stat_4);

        let equip = None;

        let relic = StarrailRelic {
            set_name,
            slot,
            star,
            level,
            main_stat,
            sub_stat_1: sub1,
            sub_stat_2: sub2,
            sub_stat_3: sub3,
            sub_stat_4: sub4,
            equip,
        };
        Some(relic)
    }
}

fn calc_pool(row: &Vec<u8>) -> f32 {
    let len = row.len() / 3;
    let mut pool: f32 = 0.0;

    for i in 0..len {
        pool += row[i * 3] as f32;
    }
    // pool /= len as f64;
    pool
}

impl YasScanner {
    fn align_row(&mut self) -> bool {
        #[cfg(target_os = "macos")]
        let (_, ui) = get_pid_and_ui();
        let mut count = 0;
        while count < 10 {
            let color = self.get_flag_color();
            if color.is_same(&self.initial_color) {
                return true;
            }

            #[cfg(windows)]
            self.enigo.mouse_scroll_y(-1);
            #[cfg(any(target_os = "linux"))]
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

            utils::sleep(self.config.scroll_stop);
            count += 1;
        }

        false
    }

    fn capture_panel(&mut self) -> Result<RgbImage> {
        let _now = SystemTime::now();
        let w = self.info.panel_position.right - self.info.panel_position.left;
        let h = self.info.panel_position.bottom - self.info.panel_position.top;
        let rect: Rect = Rect {
            left: self.info.left as i32 + self.info.panel_position.left,
            top: self.info.top as i32 + self.info.panel_position.top,
            width: w,
            height: h,
        };

        capture::capture_absolute(&rect)
    }

    fn get_relic_count(&mut self) -> Result<u32> {
        let count = self.config.number;
        if let 0 = count {
            let info = &self.info;
            let raw_after_pp = self
                .info
                .art_count_position
                .capture_relative(info.left, info.top, true)
                .unwrap();
            let s = self.model.inference_string(&raw_after_pp)?;
            info!("raw count string: {}", s);
            if s.starts_with("遗器数量") {
                let chars = s.chars().collect::<Vec<char>>();
                let count_str = (&chars[4..chars.len() - 5]).iter().collect::<String>();
                let count = match count_str.parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(String::from("无法识别遗器数量").into());
                    }
                };
                return Ok(count);
            }
            Err(String::from("无法识别遗器数量"))
        } else {
            Ok(count)
        }
    }

    fn get_flag_color(&self) -> Color {
        let flag_x = self.info.flag_x as i32 + self.info.left;
        let flag_y = self.info.flag_y as i32 + self.info.top;
        let color = capture::get_color(flag_x as u32, flag_y as u32);

        color
    }

    fn get_star(&self) -> u32 {
        let color = capture::get_color(
            (self.info.star_x as i32 + self.info.left) as u32,
            (self.info.star_y as i32 + self.info.top) as u32,
        );

        let color_1 = Color::from(113, 119, 139); // 未核实
        let color_2 = Color::from(42, 143, 114); // 未核实
        let color_3 = Color::from(81, 127, 203); // 未核实
        let color_4 = Color::from(155, 117, 206);
        let color_5 = Color::from(194, 159, 112);

        let min_dis: u32 = color_1.dis_2(&color);
        let mut star = 1_u32;
        if color_2.dis_2(&color) < min_dis {
            star = 2;
        }
        if color_3.dis_2(&color) < min_dis {
            star = 3;
        }
        if color_4.dis_2(&color) < min_dis {
            star = 4;
        }
        if color_5.dis_2(&color) < min_dis {
            star = 5;
        }

        star
    }

    pub fn move_to(&mut self, row: u32, col: u32) {
        let info = &self.info;
        let left = info.left
            + (info.left_margin + (info.art_width + info.art_gap_x) * col + info.art_width / 2)
                as i32;
        let top = info.top
            + (info.top_margin + (info.art_height + info.art_gap_y) * row + info.art_height / 4)
                as i32;
        self.enigo.mouse_move_to(left as i32, top as i32);
        #[cfg(target_os = "macos")]
        utils::sleep(20);
    }

    fn sample_initial_color(&mut self) {
        self.initial_color = self.get_flag_color();
    }

    fn scroll_one_row(&mut self) -> ScrollResult {
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
            #[cfg(any(target_os = "linux"))]
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
            utils::sleep(self.config.scroll_stop);
            count += 1;
            let color: Color = self.get_flag_color();
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
        }

        ScrollResult::TimeLimitExceeded
    }

    fn scroll_rows(&mut self, count: u32) -> ScrollResult {
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

    pub fn start(&mut self) -> Vec<StarrailRelic> {
        let count = match self.get_relic_count() {
            Ok(v) => v,
            Err(_) => 1500,
        };

        let total_row = (count + self.col - 1) / self.col;
        let last_row_col = if count % self.col == 0 {
            self.col
        } else {
            count % self.col
        };

        info!("detected count: {}", count);
        info!("total row: {}", total_row);
        info!("last column: {}", last_row_col);

        let (tx, rx) = mpsc::channel::<Option<(RgbImage, u32)>>();
        let info_2 = self.info.clone();

        let is_verbose = self.config.verbose;
        let is_dump_mode = self.config.dump_mode;
        let min_level = self.config.min_level;
        let model = self.model.clone();

        let handle = thread::spawn(move || {
            let mut results: Vec<StarrailRelic> = Vec::new();
            let mut error_count = 0;
            let mut dup_count = 0;
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;
            let info = info_2;

            let mut cnt = 0;
            if is_dump_mode {
                fs::create_dir("dumps").unwrap();
            }

            let convert_rect = |rect: &PixelRectBound| Rect {
                left: rect.left - info.panel_position.left,
                top: rect.top - info.panel_position.top,
                width: rect.right - rect.left,
                height: rect.bottom - rect.top,
            };

            for i in rx {
                let (capture, star) = match i {
                    Some(v) => v,
                    None => break,
                };
                //info!("raw capture image: width = {}, height = {}", capture.width(), capture.height());
                let _now = SystemTime::now();

                let model_inference = |pos: &PixelRectBound,
                                       name: &str,
                                       captured_img: &RgbImage,
                                       cnt: i32|
                 -> String {
                    let rect = convert_rect(pos);
                    let raw_img = to_gray(captured_img)
                        .view(
                            rect.left as u32,
                            rect.top as u32,
                            rect.width as u32,
                            rect.height as u32,
                        )
                        .to_image();
                    //info!("raw_img: width = {}, height = {}", raw_img.width(), raw_img.height());

                    if is_dump_mode {
                        raw_img
                            .to_common_grayscale()
                            .save(format!("dumps/{}_{}.png", name, cnt))
                            .expect("Err");
                    }

                    let processed_img = match pre_process(raw_img) {
                        Some(im) => im,
                        None => {
                            return String::new();
                        },
                    };
                    if is_dump_mode {
                        processed_img
                            .to_common_grayscale()
                            .save(format!("dumps/p_{}_{}.png", name, cnt))
                            .expect("Err");
                    }

                    let inference_result = model.inference_string(&processed_img);
                    if is_dump_mode {
                        fs::write(format!("dumps/{}_{}.txt", name, cnt), &inference_result)
                            .expect("Err");
                    }

                    inference_result
                };

                let str_title = model_inference(&info.title_position, "title", &capture, cnt);
                let str_main_stat_name = model_inference(
                    &info.main_stat_name_position,
                    "main_stat_name",
                    &capture,
                    cnt,
                );
                let str_main_stat_value = model_inference(
                    &info.main_stat_value_position,
                    "main_stat_value",
                    &capture,
                    cnt,
                );

                let str_sub_stat_1_name = model_inference(
                    &info.sub_stat1_name_pos, "sub_stat_1_name", &capture, cnt);
                let str_sub_stat_1_value = model_inference(
                    &info.sub_stat1_value_pos, "sub_stat_1_value", &capture, cnt);
                let str_sub_stat_2_name = model_inference(
                    &info.sub_stat2_name_pos, "sub_stat_2_name", &capture, cnt);
                let str_sub_stat_2_value = model_inference(
                    &info.sub_stat2_value_pos, "sub_stat_2_value", &capture, cnt);
                let str_sub_stat_3_name = model_inference(
                    &info.sub_stat3_name_pos, "sub_stat_3_name", &capture, cnt);
                let str_sub_stat_3_value = model_inference(
                    &info.sub_stat3_value_pos, "sub_stat_3_value", &capture, cnt);
                let str_sub_stat_4_name = model_inference(
                    &info.sub_stat4_name_pos, "sub_stat_4_name", &capture, cnt);
                let str_sub_stat_4_value = model_inference(
                    &info.sub_stat4_value_pos, "sub_stat_4_value", &capture, cnt);

                let str_level = model_inference(&info.level_position, "level", &capture, cnt);
                // let str_equip = model_inference(&info.equip_position, "equip", &capture, cnt);

                cnt += 1;

                // let predict_time = now.elapsed().unwrap().as_millis();
                // println!("predict time: {}ms", predict_time);

                let result = YasScanResult {
                    name: str_title,
                    main_stat_name: str_main_stat_name,
                    main_stat_value: str_main_stat_value,
                    sub_stat_1: str_sub_stat_1_name + "+" + &str_sub_stat_1_value,
                    sub_stat_2: str_sub_stat_2_name + "+" + &str_sub_stat_2_value,
                    sub_stat_3: str_sub_stat_3_name + "+" + &str_sub_stat_3_value,
                    sub_stat_4: str_sub_stat_4_name + "+" + &str_sub_stat_4_value,
                    level: str_level,
                    equip: String::new(),
                    star,
                };
                if is_verbose {
                    info!("{:?}", result);
                }
                // println!("{:?}", result);
                let relic = result.to_internal_relic();
                if let Some(a) = relic {
                    if hash.contains(&a) {
                        dup_count += 1;
                        consecutive_dup_count += 1;
                        warn!("dup relic detected: {:?}", result);
                    } else {
                        consecutive_dup_count = 0;
                        hash.insert(a.clone());
                        results.push(a);
                    }
                } else {
                    error!("wrong detection: {:?}", result);
                    error_count += 1;
                    // println!("error parsing results");
                }
                if consecutive_dup_count >= info.art_row {
                    error!("检测到连续多个重复遗器，可能为翻页错误，或者为非背包顶部开始扫描");
                    break;
                }
            }

            info!("error count: {}", error_count);
            info!("dup count: {}", dup_count);

            if min_level > 0 {
                results
                    .into_iter()
                    .filter(|result| result.level >= min_level)
                    .collect::<Vec<_>>()
            } else {
                results
            }
        });

        let mut scanned_row = 0_u32;
        let mut scanned_count = 0_u32;
        let mut start_row = 0_u32;
        self.move_to(0, 0);
        #[cfg(target_os = "macos")]
        utils::sleep(20);
        self.enigo.mouse_click(MouseButton::Left);
        utils::sleep(1000);
        // self.wait_until_switched();
        self.sample_initial_color();

        'outer: while scanned_count < count {
            'row: for row in start_row..self.row {
                let c = if scanned_row == total_row - 1 {
                    last_row_col
                } else {
                    self.col
                };
                'col: for col in 0..c {
                    // 大于最大数量则退出
                    if scanned_count > count {
                        break 'outer;
                    }

                    // 右键终止
                    if utils::is_rmb_down() {
                        break 'outer;
                    }

                    self.move_to(row, col);
                    self.enigo.mouse_click(MouseButton::Left);
                    #[cfg(target_os = "macos")]
                    utils::sleep(20);

                    self.wait_until_switched();
                    let capture = self.capture_panel().unwrap();
                    let star = self.get_star();
                    if star < self.config.min_star {
                        break 'outer;
                    }
                    tx.send(Some((capture, star))).unwrap();

                    scanned_count += 1;
                } // end 'col

                scanned_row += 1;

                if scanned_row >= self.config.max_row {
                    info!("max row reached, quiting...");
                    break 'outer;
                }
            } // end 'row

            let remain = count - scanned_count;
            let remain_row = (remain + self.col - 1) / self.col;
            let scroll_row = remain_row.min(self.row);
            start_row = self.row - scroll_row;
            match self.scroll_rows(scroll_row) {
                ScrollResult::TimeLimitExceeded => {
                    error!("翻页出现问题");
                    break 'outer;
                },
                ScrollResult::Interrupt => break 'outer,
                _ => (),
            }

            utils::sleep(100);
        }

        tx.send(None).unwrap();

        info!("扫描结束，等待识别线程结束，请勿关闭程序");
        let results: Vec<StarrailRelic> = handle.join().unwrap();
        info!("count: {}", results.len());
        results
    }
    fn wait_until_switched(&mut self) -> bool {
        if self.is_cloud {
            utils::sleep(self.config.cloud_wait_switch_item);
            return true;
        }
        let now = SystemTime::now();

        let mut consecutive_time = 0;
        let mut diff_flag = false;
        while now.elapsed().unwrap().as_millis() < self.config.max_wait_switch_item as u128 {
            // let pool_start = SystemTime::now();
            let rect = Rect {
                left: self.info.left as i32 + self.info.pool_position.left,
                top: self.info.top as i32 + self.info.pool_position.top,
                width: self.info.pool_position.right - self.info.pool_position.left,
                height: self.info.pool_position.bottom - self.info.pool_position.top,
            };
            let im = capture::capture_absolute(&rect).unwrap();
            let pool = calc_pool(im.as_raw()) as f64;
            // info!("pool: {}", pool);
            // println!("pool time: {}ms", pool_start.elapsed().unwrap().as_millis());

            if (pool - self.pool).abs() > 0.000001 {
                self.pool = pool;
                diff_flag = true;
                consecutive_time = 0;
                // info!("avg switch time: {}ms", self.avg_switch_time);
            } else {
                if diff_flag {
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
        }

        false
    }
}
