use std::collections::HashSet;
use std::convert::From;
use std::fs;

use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;

use clap::Parser;
use enigo::*;
use image::{GenericImageView, RgbImage};
use log::{error, info, warn};
use tract_onnx::prelude::tract_itertools::Itertools;

use crate::artifact::internal_artifact::{
    ArtifactSetName, ArtifactSlot, ArtifactStat, InternalArtifact,
};
use crate::capture::{self};
use crate::common::character_name::CHARACTER_NAMES;
use crate::common::color::Color;
#[cfg(target_os = "macos")]
use crate::common::utils::get_pid_and_ui;
use crate::common::{utils, PixelRect, PixelRectBound};
use crate::inference::inference::CRNNModel;
use crate::inference::pre_process::{pre_process, to_gray, ImageConvExt};
use crate::info::info::ScanInfo;

// Playcover only, wine should not need this.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use crate::common::utils::mac_scroll;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    // 莫娜占卜铺 https://www.mona-uranai.com/
    Mona,
    // 原魔计算器 https://genshin.mingyulab.com/
    #[value(name = "mingyulab")]
    MingyuLab,
    // 原魔计算器 https://genshin.mingyulab.com/
    Genmo,
    // Genshin Optimizer https://frzyc.github.io/genshin-optimizer/
    Good,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Mona => "mona",
            OutputFormat::MingyuLab => "mingyulab",
            OutputFormat::Genmo => "genmo",
            OutputFormat::Good => "good",
        }
    }

    // 以防某些网站不接受 JSON 格式的数据.
    pub fn filename(&self) -> std::path::PathBuf {
        let mut path = std::ffi::OsString::from(self.as_str());
        path.push(".json");
        std::path::PathBuf::from(path)
    }
}

#[derive(Debug, Parser)]
#[command(version = utils::VERSION)]
#[command(name = "YAS - 原神圣遗物导出器")]
#[command(author = "wormtql <584130248@qq.com>")]
#[command(about = "Genshin Impact Artifact Exporter")]
pub struct YasScannerConfig {
    /// 最大扫描行数
    #[arg(long)]
    #[arg(default_value_t = 1000)]
    pub max_row: u32,
    /// 输出模型预测结果、二值化图像和灰度图像，debug 专用
    #[arg(long = "dump")]
    pub dump_mode: bool,
    /// 只保存截图，不进行扫描，debug专用
    #[arg(long)]
    pub capture_only: bool,
    /// 最小星级
    #[arg(long)]
    #[arg(default_value_t = 4)]
    pub min_star: u32,
    /// 最小等级
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub min_level: u32,
    /// 切换圣遗物最大等待时间(ms)
    #[arg(long)]
    #[arg(default_value_t = 800)]
    pub max_wait_switch_artifact: u32,
    /// 输出目录
    #[arg(long)]
    #[arg(short)]
    #[arg(default_value_t = String::from("."))]
    pub output_dir: String,
    /// 翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项，默认为80）
    #[arg(long)]
    #[arg(default_value_t = 80)]
    pub scroll_stop: u32,
    /// 指定圣遗物数量（在自动识别数量不准确时使用）
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub number: u32,
    /// 显示详细信息
    #[arg(long)]
    pub verbose: bool,
    /// 人为指定横坐标偏移（截图有偏移时可用该选项校正）
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub offset_x: i32,
    /// 人为指定纵坐标偏移（截图有偏移时可用该选项校正）
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub offset_y: i32,
    /// 输出格式
    #[arg(long)]
    #[arg(short = 'f')]
    #[arg(default_value_t = OutputFormat::Mona)]
    pub output_format: OutputFormat,
    /// 指定云·原神切换圣遗物等待时间(ms)
    #[arg(long, default_value_t = 300)]
    pub cloud_wait_switch_artifact: u32,
}

pub struct YasScanner<'a> {
    model: CRNNModel,
    enigo: Enigo,

    info: &'a ScanInfo,
    config: &'a YasScannerConfig,

    row: u32,
    col: u32,

    pool: f64,

    initial_color: Color,

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: u32,

    is_cloud: bool,
}

enum ScrollResult {
    TLE,
    // time limit exceeded
    Interrupt,
    Success,
    Skip,
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
        let level = self
            .level
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<u32>()
            .ok()?;
        let main_stat = ArtifactStat::from_zh_cn_raw(
            (self.main_stat_name.clone() + "+" + self.main_stat_value.as_str()).as_str(),
        )?;
        let sub1 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_1);
        let sub2 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_2);
        let sub3 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_3);
        let sub4 = ArtifactStat::from_zh_cn_raw(&self.sub_stat_4);

        let equip = if self.equip.contains("已装备") {
            //let equip_name = self.equip.clone();
            //equip_name.remove_matches("已装备");
            //let equip_name = &self.equip[..self.equip.len()-9];
            let chars = self.equip.chars().collect_vec();
            let chars2 = &chars[..chars.len() - 3];
            let equip_name = chars2.iter().collect::<String>();
            if CHARACTER_NAMES.contains(equip_name.as_str()) {
                Some(equip_name)
            } else {
                None
            }
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

fn calc_pool(row: &Vec<u8>) -> f32 {
    let len = row.len() / 3;
    let mut pool: f32 = 0.0;

    for i in 0..len {
        pool += row[i * 3] as f32;
    }
    // pool /= len as f64;
    pool
}

impl<'a> YasScanner<'a> {
    pub fn new(info: &'a ScanInfo, config: &'a YasScannerConfig, is_cloud: bool) -> Self {
        let row = info.art_row;
        let col = info.art_col;

        YasScanner {
            model: CRNNModel::new(
                String::from("model_training.onnx"),
                String::from("index_2_word.json"),
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

            is_cloud,
        }
    }
}

impl YasScanner<'_> {
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

    /*
    pub fn panel_down(&mut self) {
        let info = &self.info;
        let max_scroll = 20;
        let mut count = 0;
        self.enigo.mouse_move_to(
            info.left + info.star_x as i32,
            info.top + info.star_y as i32,
        );
        let level_color = Color::from(57, 67, 79);
        let mut color = capture::get_color(
            info.level_position.left as u32,
            info.level_position.bottom as u32,
        );
        while !level_color.is_same(&color) && count < max_scroll {
            #[cfg(windows)]
            self.enigo.mouse_scroll_y(5);
            #[cfg(target_os = "linux")]
            self.enigo.mouse_scroll_y(-1);

            utils::sleep(self.config.scroll_stop);
            color = capture::get_color(
                info.level_position.left as u32,
                info.level_position.bottom as u32,
            );
            count += 1;
        }
    }
    */

    fn capture_panel(&mut self) -> Result<RgbImage, String> {
        let _now = SystemTime::now();
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
        Ok(u8_arr)
    }

    fn get_art_count(&mut self) -> Result<u32, String> {
        let count = self.config.number;
        if let 0 = count {
            let info = &self.info;
            let raw_after_pp = self
                .info
                .art_count_position
                .capture_relative(info.left, info.top, true)
                .unwrap();
            let s = self.model.inference_string(&raw_after_pp);
            info!("raw count string: {}", s);
            if s.starts_with("圣遗物") {
                let chars = s.chars().collect::<Vec<char>>();
                let count_str = (&chars[4..chars.len() - 5]).iter().collect::<String>();
                let count = match count_str.parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(String::from("无法识别圣遗物数量"));
                    },
                };
                return Ok(count);
            }
            Err(String::from("无法识别圣遗物数量"))
        } else {
            return Ok(count);
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

        let color_1 = Color::from(113, 119, 139);
        let color_2 = Color::from(42, 143, 114);
        let color_3 = Color::from(81, 127, 203);
        let color_4 = Color::from(161, 86, 224);
        let color_5 = Color::from(188, 105, 50);

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
        let max_scroll = 20;
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
            if state == 0 && !color.is_same(&self.initial_color) {
                state = 1;
            } else if state == 1 && self.initial_color.is_same(&color) {
                self.avg_scroll_one_row = (self.avg_scroll_one_row * self.scrolled_rows as f64
                    + count as f64)
                    / (self.scrolled_rows as f64 + 1.0);
                info!("avg scroll/row: {}", self.avg_scroll_one_row);
                self.scrolled_rows += 1;
                return ScrollResult::Success;
            }
        }

        ScrollResult::TLE
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
                ScrollResult::TLE => return ScrollResult::TLE,
                ScrollResult::Interrupt => return ScrollResult::Interrupt,
                _ => (),
            }
        }

        ScrollResult::Success
    }

    /*
        fn start_capture_only(&mut self) {
            fs::create_dir("captures");
            let info = &self.info.clone();

            println!("capture l:{}, t:{}, w:{},h:{}", info.left, info.top, info.width, info.height);
            let raw_im_rect = PixelRectBound {
                left: info.left as i32,
                top: info.top as i32,
                right: info.left + info.width as i32,
                bottom: info.top + info.height as i32,
            };

            let raw_im = raw_im_rect.capture_relative(info, false).unwrap();
            raw_im.grayscale_to_gray_image().save("captures/raw_im.png");
            println!("Finish capture raw");
            panic!();

            let count = self.info.art_count_position.capture_relative(info, false).unwrap();
            count.to_gray_image().save("captures/count.png");

            let convert_rect = |rect: &PixelRectBound| PixelRect {
                left: rect.left - info.panel_position.left,
                top: rect.top - info.panel_position.top,
                width: rect.right - rect.left,
                height: rect.bottom - rect.top,
            };

            let panel = self.capture_panel().unwrap();
            let im_title = pre_process(panel.crop_to_raw_img(&convert_rect(&info.title_position)));
            if let Some(im) = im_title {
                im.to_gray_image().save("captures/title.png").expect("Err");
            }

            let im_main_stat_name =
                pre_process(panel.crop_to_raw_img(&convert_rect(&info.main_stat_name_position)));
            if let Some(im) = im_main_stat_name {
                im.to_gray_image()
                    .save("captures/main_stat_name.png")
                    .expect("Err");
            }

            let im_main_stat_value =
                pre_process(panel.crop_to_raw_img(&convert_rect(&info.main_stat_value_position)));
            if let Some(im) = im_main_stat_value {
                im.to_gray_image()
                    .save("captures/main_stat_value.png")
                    .expect("Err");
            }

            let im_sub_stat_1 =
                pre_process(panel.crop_to_raw_img(&convert_rect(&info.sub_stat1_position)));
            if let Some(im) = im_sub_stat_1 {
                im.to_gray_image()
                    .save("captures/sub_stat_1.png")
                    .expect("Err");
            }

            let im_sub_stat_2 =
                pre_process(panel.crop_to_raw_img(&convert_rect(&info.sub_stat2_position)));
            if let Some(im) = im_sub_stat_2 {
                im.to_gray_image()
                    .save("captures/sub_stat_2.png")
                    .expect("Err");
            }

            let im_sub_stat_3 =
                pre_process(panel.crop_to_raw_img(&convert_rect(&info.sub_stat3_position)));
            if let Some(im) = im_sub_stat_3 {
                im.to_gray_image()
                    .save("captures/sub_stat_3.png")
                    .expect("Err");
            }

            let im_sub_stat_4 =
                pre_process(panel.crop_to_raw_img(&convert_rect(&info.sub_stat4_position)));
            if let Some(im) = im_sub_stat_4 {
                im.to_gray_image()
                    .save("captures/sub_stat_4.png")
                    .expect("Err");
            }

            let im_level = pre_process(panel.crop_to_raw_img(&convert_rect(&info.level_position)));
            if let Some(im) = im_level {
                im.to_gray_image().save("captures/level.png").expect("Err");
            }

            let im_equip = pre_process(panel.crop_to_raw_img(&convert_rect(&info.equip_position)));
            if let Some(im) = im_equip {
                im.to_gray_image().save("captures/equip.png").expect("Err");
            }
        }
    */
    pub fn start(&mut self) -> Vec<InternalArtifact> {
        //self.panel_down();
        /*         if self.config.capture_only {
           self.start_capture_only();
           return Vec::new();
        } */

        let count = match self.get_art_count() {
            Ok(v) => v,
            Err(_) => 1500,
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

        let (tx, rx) = mpsc::channel::<Option<(RgbImage, u32)>>();
        let info_2 = self.info.clone();
        // v bvvmnvbm
        let is_verbose = self.config.verbose;
        let is_dump_mode = self.config.dump_mode;
        let min_level = self.config.min_level;
        let handle = thread::spawn(move || {
            let mut results: Vec<InternalArtifact> = Vec::new();
            let model = CRNNModel::new(
                String::from("model_training.onnx"),
                String::from("index_2_word.json"),
            );
            let mut error_count = 0;
            let mut dup_count = 0;
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;
            let info = info_2;

            let mut cnt = 0;
            if is_dump_mode {
                fs::create_dir("dumps").unwrap();
            }

            let convert_rect = |rect: &PixelRectBound| PixelRect {
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

                let str_sub_stat_1 =
                    model_inference(&info.sub_stat1_position, "sub_stat_1", &capture, cnt);
                let str_sub_stat_2 =
                    model_inference(&info.sub_stat2_position, "sub_stat_2", &capture, cnt);
                let str_sub_stat_3 =
                    model_inference(&info.sub_stat3_position, "sub_stat_3", &capture, cnt);
                let str_sub_stat_4 =
                    model_inference(&info.sub_stat4_position, "sub_stat_4", &capture, cnt);

                let str_level = model_inference(&info.level_position, "level", &capture, cnt);
                let str_equip = model_inference(&info.equip_position, "equip", &capture, cnt);

                cnt += 1;

                // let predict_time = now.elapsed().unwrap().as_millis();
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
                if is_verbose {
                    info!("{:?}", result);
                }
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
                ScrollResult::TLE => {
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
        let results: Vec<InternalArtifact> = handle.join().unwrap();
        info!("count: {}", results.len());
        results
    }
    fn wait_until_switched(&mut self) -> bool {
        if self.is_cloud {
            utils::sleep(self.config.cloud_wait_switch_artifact);
            return true;
        }
        let now = SystemTime::now();

        let mut consecutive_time = 0;
        let mut diff_flag = false;
        while now.elapsed().unwrap().as_millis() < self.config.max_wait_switch_artifact as u128 {
            // let pool_start = SystemTime::now();
            let rect = PixelRect {
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
                // info!("pool: {}", pool);
                // let raw = RawCaptureImage {
                //     data: im,
                //     w: rect.width as u32,
                //     h: rect.height as u32,
                // };
                // let raw = raw.to_raw_image();
                // println!("{:?}", &raw.data[..10]);
                // raw.save(&format!("captures/{}.png", rand::thread_rng().gen::<u32>()));

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

impl YasScanner<'_> {
    // pub fn start_from_scratch(config: YasScannerConfig) -> Result<Vec<InternalArtifact>, String> {
    //     set_dpi_awareness();
    //     let mut is_cloud = false;
    //     let hwnd = match find_window_local() {
    //         Ok(v) => {is_cloud = true; v},
    //         Err(s) => {
    //             match find_window_cloud() {
    //                 Ok(v) => v,
    //                 Err(s) => return Err(String::from("未找到原神窗口"))
    //             }
    //         }
    //     };
    //
    //     show_window_and_set_foreground(hwnd);
    //     sleep(1000);
    //
    //     let mut rect = match get_client_rect(hwnd) {
    //         Ok(v) => v,
    //         Err(_) => return Err(String::from("未能获取窗口大小"))
    //     };
    //
    //     let info = match ScanInfo::from_rect(&rect) {
    //         Ok(v) => v,
    //         Err(e) => return Err(e)
    //     };
    //
    //     let mut scanner = YasScanner::new(info, config, is_cloud);
    //     let result = scanner.start();
    //
    //     Ok(result)
    // }
}
