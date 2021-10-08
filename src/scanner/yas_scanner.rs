use std::time::{SystemTime};
use std::thread;
use std::sync::mpsc;
use std::convert::From;
use std::collections::HashSet;
use std::io::stdin;
use std::fs;

use enigo::*;
use log::{info, warn, error, debug};
use clap::{ArgMatches};

use crate::info::info::ScanInfo;
use crate::inference::inference::CRNNModel;
use crate::common::{utils, RawImage, PixelRect, RawCaptureImage, PixelRectBound};
use crate::capture;
use crate::common::color::Color;
use crate::artifact::internal_artifact::{ArtifactSlot, ArtifactStat, ArtifactSetName, InternalArtifact};

pub struct YasScannerConfig {
    max_row: u32,
    capture_only: bool,
    min_star: u32,
    max_wait_switch_artifact: u32,
}

impl YasScannerConfig {
    pub fn from_match(matches: &ArgMatches) -> YasScannerConfig {
        YasScannerConfig {
            max_row: matches.value_of("max-row").unwrap_or("1000").parse::<u32>().unwrap(),
            capture_only: matches.is_present("capture-only"),
            min_star: matches.value_of("min-star").unwrap_or("4").parse::<u32>().unwrap(),
            max_wait_switch_artifact: matches.value_of("max-wait-switch-artifact").unwrap_or("500").parse::<u32>().unwrap(),
        }
    }
}

pub struct YasScanner {
    model: CRNNModel,
    enigo: Enigo,

    info: ScanInfo,
    config: YasScannerConfig,

    row: u32,
    col: u32,

    pool: f64,

    initial_color: Color,

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: u32,
}

#[derive(Debug)]
pub struct YasScanResult {
    name: String,
    main_stat_name: String,
    main_stat_value: String,
    sub_stat_1: String,
    sub_stat_2: String,
    sub_stat_3: String,
    sub_stat_4: String,
    level: String,
    equip: String,
    star: u32,
}

impl YasScanResult {
    pub fn to_internal_artifact(&self) -> Option<InternalArtifact> {
        let set_name = ArtifactSetName::from_zh_cn(&self.name)?;
        let slot = ArtifactSlot::from_zh_cn(&self.name)?;
        let star = self.star;
        if !self.level.contains("+") {
            return None;
        }
        let level = self.level.chars().skip(1).collect::<String>().parse::<u32>().ok()?;
        let main_stat = ArtifactStat::from_zh_cn_raw(
            (self.main_stat_name.clone() + "+" + self.main_stat_value.as_str()).as_str()
        )?;
        let sub1 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_1);
        let sub2 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_2);
        let sub3 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_3);
        let sub4 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_4);

        let equip = if self.equip.contains("已装备") {
            Some(self.equip.chars().take(self.equip.len() - 3).collect::<String>())
        } else {
            None
        };

        let art = InternalArtifact {
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
        Some(art)
    }
}

fn calc_pool(row: &Vec<u8>) -> f64 {
    let len = row.len() / 4;
    let mut pool: f64 = 0.0;

    for i in 0..len {
        pool += row[i * 4] as f64;
    }
    pool /= len as f64;
    pool
}

impl YasScanner {
    pub fn new(info: ScanInfo, config: YasScannerConfig) -> YasScanner {
        let row = info.art_row;
        let col = info.art_col;

        YasScanner {
            model: CRNNModel::new(
                String::from("model_training.onnx"),
                String::from("index_2_word.json")
            ),
            enigo: Enigo::new(),
            info,
            config,

            row,
            col,

            pool: -1.0,
            initial_color: Color::new(),
            scrolled_rows: 0,
            avg_scroll_one_row: 0.0,

            avg_switch_time: 0.0,
            scanned_count: 0,
        }
    }
}

impl YasScanner {
    pub fn move_to(&mut self, row: u32, col: u32) {
        let info = &self.info;
        let left = info.left + info.left_margin + (info.art_width + info.art_gap_x) * col + info.art_width / 2;
        let top = info.top + info.top_margin + (info.art_height + info.art_gap_y) * row + info.art_height / 4;
        self.enigo.mouse_move_to(left as i32, top as i32);
    }

    fn sample_initial_color(&mut self) {
        self.initial_color = self.get_color();
    }

    fn get_color(&self) -> Color {
        let flag_x = self.info.flag_x + self.info.left;
        let flag_y = self.info.flag_y + self.info.top;
        let color = capture::get_color(flag_x, flag_y);

        color
    }

    fn get_art_count(&mut self) -> Result<u32, String> {
        let info = &self.info;
        let raw_after_pp = self.info.art_count_position.capture_relative(info).unwrap();
        // raw_after_pp.to_gray_image().save("count.png");
        let s = self.model.inference_string(&raw_after_pp);
        info!("raw count string: {}", s);
        if s.starts_with("圣遗物") {
            let chars = s.chars().collect::<Vec<char>>();
            let count_str = (&chars[4..chars.len() - 5]).iter().collect::<String>();
            let count = match count_str.parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(String::from("无法识别圣遗物数量"));
                }
            };
            return Ok(count);
        }

        Err(String::from("无法识别圣遗物数量"))
    }

    fn scroll_one_row(&mut self) -> bool {
        let mut state = 0;
        let mut count = 0;
        let max_scroll = 20;
        while count < max_scroll {
            self.enigo.mouse_scroll_y(-5);
            utils::sleep(80);
            count += 1;
            let color: Color = self.get_color();
            // println!("{:?}", color);
            if state == 0 && !color.is_same(&self.initial_color) {
                state = 1;
            } else if state == 1 && self.initial_color.is_same(&color) {
                self.avg_scroll_one_row = (self.avg_scroll_one_row * self.scrolled_rows as f64 + count as f64) / (self.scrolled_rows as f64 + 1.0);
                info!("avg scroll/row: {}", self.avg_scroll_one_row);
                self.scrolled_rows += 1;
                return true;
            }
        }

        false
    }

    fn scroll_rows(&mut self, count: u32) {
        if self.scrolled_rows >= 5 {
            let scroll = ((self.avg_scroll_one_row * count as f64 - 3.0).round() as u32).max(0);
            for _ in 0..scroll {
                self.enigo.mouse_scroll_y(-1);
            }
            utils::sleep(400);
            self.align_row();
            return;
        }

        for _ in 0..count {
            if !self.scroll_one_row() {
                break;
            }
        }
    }

    fn align_row(&mut self) -> bool {
        let mut count = 0;
        while count < 10 {
            let color = self.get_color();
            if color.is_same(&self.initial_color) {
                return true;
            }

            self.enigo.mouse_scroll_y(-1);
            utils::sleep(50);
            count += 1;
        }

        false
    }

    fn wait_until_switched(&mut self) -> bool {
        let now = SystemTime::now();
        while now.elapsed().unwrap().as_millis() < self.config.max_wait_switch_artifact as u128 {
            // let pool_start = SystemTime::now();
            let rect = PixelRect {
                left: self.info.left as i32 + self.info.pool_position.left,
                top: self.info.top as i32 + self.info.pool_position.top,
                width: self.info.pool_position.right - self.info.pool_position.left,
                height: self.info.pool_position.bottom - self.info.pool_position.top,
            };
            let im = capture::capture_absolute(&rect).unwrap();
            let pool = calc_pool(&im);
            // info!("pool: {}", pool);
            // println!("pool time: {}ms", pool_start.elapsed().unwrap().as_millis());

            if (pool - self.pool).abs() > 0.000001 {
                // if pool != pool1 {
                //     pool1 = pool;
                // } else {
                self.pool = pool;
                // }

                self.avg_switch_time = (self.avg_switch_time * self.scanned_count as f64 + now.elapsed().unwrap().as_millis() as f64) / (self.scanned_count as f64 + 1.0);
                self.scanned_count += 1;
                // info!("avg switch time: {}ms", self.avg_switch_time);
                return true;
            }
        }

        false
    }

    fn capture_panel(&mut self) -> Result<RawCaptureImage, String> {
        let now = SystemTime::now();
        let w = self.info.panel_position.right - self.info.panel_position.left;
        let h = self.info.panel_position.bottom - self.info.panel_position.top;
        let rect: PixelRect = PixelRect {
            left: self.info.left as i32 + self.info.panel_position.left,
            top: self.info.top as i32 + self.info.panel_position.top,
            width: w,
            height: h,
        };
        let u8_arr = capture::capture_absolute(&rect)?;
        // info!("capture time: {}ms", now.elapsed().unwrap().as_millis());
        Ok(RawCaptureImage {
            data: u8_arr,
            w: w as u32,
            h: h as u32,
        })
    }

    fn get_star(&self) -> u32 {
        let color = capture::get_color(
            self.info.star_x + self.info.left,
            self.info.star_y + self.info.top
        );

        let color_1 = Color::from(113, 119, 139);
        let color_2 = Color::from(42, 143, 114);
        let color_3 = Color::from(81, 127, 203);
        let color_4 = Color::from(161, 86, 224);
        let color_5 = Color::from(188, 105, 50);

        let mut min_dis: u32 = color_1.dis_2(&color);
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

    fn start_capture_only(&mut self) {
        fs::create_dir("captures");
        let info = &self.info.clone();

        let count = self.info.art_count_position.capture_relative(info).unwrap();
        count.to_gray_image().save("captures/count.png");

        let convert_rect = |rect: &PixelRectBound| {
            PixelRect {
                left: rect.left - info.panel_position.left,
                top: rect.top - info.panel_position.top,
                width: rect.right - rect.left,
                height: rect.bottom - rect.top,
            }
        };

        let panel = self.capture_panel().unwrap();
        let im_title = panel.crop_and_preprocess(&convert_rect(&info.title_position));
        im_title.to_gray_image().save("captures/title.png");
        let im_main_stat_name = panel.crop_and_preprocess(&convert_rect(&info.main_stat_name_position));
        im_main_stat_name.to_gray_image().save("captures/main_stat_name.png");
        let im_main_stat_value = panel.crop_and_preprocess(&convert_rect(&info.main_stat_value_position));
        im_main_stat_value.to_gray_image().save("captures/main_stat_value.png");
        let im_sub_stat_1 = panel.crop_and_preprocess(&convert_rect(&info.sub_stat1_position));
        im_sub_stat_1.to_gray_image().save("captures/sub_stat_1.png");
        let im_sub_stat_2 = panel.crop_and_preprocess(&convert_rect(&info.sub_stat2_position));
        im_sub_stat_2.to_gray_image().save("captures/sub_stat_2.png");
        let im_sub_stat_3 = panel.crop_and_preprocess(&convert_rect(&info.sub_stat3_position));
        im_sub_stat_3.to_gray_image().save("captures/sub_stat_3.png");
        let im_sub_stat_4 = panel.crop_and_preprocess(&convert_rect(&info.sub_stat4_position));
        im_sub_stat_4.to_gray_image().save("captures/sub_stat_4.png");
        let im_level = panel.crop_and_preprocess(&convert_rect(&info.level_position));
        im_level.to_gray_image().save("captures/level.png");
        let im_equip = panel.crop_and_preprocess(&convert_rect(&info.equip_position));
        im_equip.to_gray_image().save("captures/equip.png");
    }

    pub fn start(&mut self) -> Vec<InternalArtifact> {
        if self.config.capture_only {
            self.start_capture_only();
            return Vec::new();
        }

        let mut count = match self.get_art_count() {
            Ok(v) => v,
            Err(_) => 1000,
        };

        let total_row = (count + self.col - 1) / self.col;
        let last_row_col = if count % self.col == 0 {
            self.col
        } else {
            count % self.col
        };

        // println!("检测到圣遗物数量：{}，若无误请按回车，否则输入正确的圣遗物数量：", count);
        // let mut s: String = String::new();
        // stdin().read_line(&mut s);
        // if s.trim() != "" {
        //     count = s.trim().parse::<u32>().unwrap();
        // }

        info!("detected count: {}", count);
        info!("total row: {}", total_row);
        info!("last column: {}", last_row_col);

        let (tx, rx) = mpsc::channel::<Option<(RawCaptureImage, u32)>>();
        let info_2 = self.info.clone();
        // v bvvmnvbm
        let handle = thread::spawn(move || {
            let mut results: Vec<InternalArtifact> = Vec::new();
            let mut model = CRNNModel::new(
                String::from("model_training.onnx"),
                String::from("index_2_word.json")
            );
            let mut error_count = 0;
            let mut dup_count = 0;
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;
            let info = info_2;

            let convert_rect = |rect: &PixelRectBound| {
                PixelRect {
                    left: rect.left - info.panel_position.left,
                    top: rect.top - info.panel_position.top,
                    width: rect.right - rect.left,
                    height: rect.bottom - rect.top,
                }
            };

            for i in rx {
                let (capture, star) = match i {
                    Some(v) => v,
                    None => break,
                };
                let now = SystemTime::now();
                let str_title = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.title_position)));
                let str_main_stat_name = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.main_stat_name_position)));
                let str_main_stat_value = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.main_stat_value_position)));
                let str_sub_stat_1 = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.sub_stat1_position)));
                let str_sub_stat_2 = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.sub_stat2_position)));
                let str_sub_stat_3 = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.sub_stat3_position)));
                let str_sub_stat_4 = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.sub_stat4_position)));
                let str_level = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.level_position)));
                let str_equip = model.inference_string(&capture.crop_and_preprocess(&convert_rect(&info.equip_position)));
                let predict_time = now.elapsed().unwrap().as_millis();
                // println!("predict time: {}ms", predict_time);

                let result = YasScanResult {
                    name: str_title,
                    main_stat_name: str_main_stat_name,
                    main_stat_value: str_main_stat_value,
                    sub_stat_1: str_sub_stat_1,
                    sub_stat_2: str_sub_stat_2,
                    sub_stat_3: str_sub_stat_3,
                    sub_stat_4: str_sub_stat_4,
                    level: str_level,
                    equip: str_equip,
                    star,
                };
                // println!("{:?}", result);
                let art = result.to_internal_artifact();
                if let Some(a) = art {
                    if hash.contains(&a) {
                        dup_count += 1;
                        consecutive_dup_count += 1;
                        warn!("dup artifact detected: {:?}", result);
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
                    error!("检测到连续多个重复圣遗物，可能为翻页错误，或者为非背包顶部开始扫描");
                    break;
                }
            }

            info!("error count: {}", error_count);
            info!("dup count: {}", dup_count);

            results
        });


        let mut scanned_row = 0_u32;
        let mut scanned_count = 0_u32;
        let mut start_row = 0_u32;

        self.move_to(0, 0);
        self.enigo.mouse_click(MouseButton::Left);
        utils::sleep(1000);
        self.sample_initial_color();

        'outer: while scanned_count < count {
            'row: for row in start_row..self.row {
                let c = if scanned_row == total_row - 1 { last_row_col } else { self.col };
                'col: for col in 0..c {
                    if scanned_count > count {
                        break 'outer;
                    }

                    self.move_to(row, col);
                    self.enigo.mouse_click(MouseButton::Left);

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
            self.scroll_rows(scroll_row);

            utils::sleep(100);
        }

        tx.send(None).unwrap();

        info!("扫描结束，等待识别线程结束，请勿关闭程序");
        let results: Vec<InternalArtifact> = handle.join().unwrap();
        info!("count: {}", results.len());
        results
    }
}