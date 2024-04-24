use image::{RgbImage, GenericImageView};
use log::{error, info, warn};
use yas::{capture::capture::{self, RelativeCapturable}, common::{color::Color, positioning::{Rect, Pos}}, window_info::{require_window_info::RequireWindowInfo, window_info_repository::WindowInfoRepository}, inference::{model::OCRModel, pre_process::{pre_process, to_gray, ImageConvExt}}, game_info::GameInfo};
use std::{ops::{Coroutine, CoroutineState}, pin::Pin, rc::Rc, cell::RefCell, sync::{mpsc::{Receiver, Sender, self}, Arc}, thread::JoinHandle, os::windows::thread, collections::HashSet, time::SystemTime};

use crate::scanner_controller::repository_layout::{scan_logic::{StarRailRepositoryScanController, ReturnResult}, config::StarRailRepositoryScannerLogicConfig};

use super::relic_scanner_config::StarRailRelicScannerConfig;
use anyhow::Result;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct StarRailRelicScanResult {
    pub name: String,
    pub main_stat_name: String,
    pub main_stat_value: String,
    pub sub_stat_name: [String; 4],
    pub sub_stat_value: [String; 4],
    pub equip: String,
    pub level: i32,
    pub star: i32,
    pub lock: bool,
    pub discard: bool,
}

struct RelicScannerWorker {
    model: OCRModel,
    window_info: RelicScannerWindowInfo,
    config: StarRailRelicScannerConfig,
}

fn parse_level(s: &str) -> Result<i32> {
    let pos = s.find('+');

    if pos.is_none() {
        let level = s.parse::<i32>()?;
        return Ok(level);
    }

    let level = s[pos.unwrap()..].parse::<i32>()?;
    return Ok(level);
}

impl RelicScannerWorker {
    pub fn new(
        model: OCRModel,
        window_info: RelicScannerWindowInfo,
        config: StarRailRelicScannerConfig,
    ) -> Self {
        RelicScannerWorker {
            model,
            window_info,
            config,
        }
    }

    fn model_inference(&self, rect: Rect, captured_img: &RgbImage) -> Result<String> {
        // todo move dump mode into a scanner
        // if dump_mode {
            // captured_img.save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        // }

        let relative_rect = rect.translate(Pos {
            x: -self.window_info.panel_rect.left,
            y: -self.window_info.panel_rect.top,
        });

        let raw_img = captured_img.view(
            relative_rect.left as u32, relative_rect.top as u32, relative_rect.width as u32, relative_rect.height as u32
        ).to_image();
        let raw_img_grayed = to_gray(&raw_img);

        // let raw_img = to_gray(captured_img)
        //     .view(
        //         relative_rect.left,
        //         relative_rect.top,
        //         rect.size.width,
        //         rect.size.height,
        //     )
        //     .to_image();

        // if dump_mode {
        //     raw_img
        //         .to_common_grayscale()
        //         .save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        // }

        let (processed_img, process_flag) = pre_process(raw_img_grayed);
        if !process_flag {
            return Ok(String::new());
        }

        // if dump_mode {
        //     processed_img
        //         .to_common_grayscale()
        //         .save(Path::new("dumps").join(format!("{}_{}.pp.png", name, cnt)))?;
        // }

        let inference_result = self.model.inference_string(&processed_img)?;

        // if dump_mode {
        //     dump_text(
        //         &inference_result,
        //         Path::new("dumps").join(format!("{}_{}.txt", name, cnt)),
        //     );
        // }

        Ok(inference_result)
    }

    fn scan_item_image(&self, item: SendItem) -> Result<StarRailRelicScanResult> {
        let image = &item.panel_image;

        let str_title = self.model_inference(self.window_info.title_rect, &image)?;
        let str_main_stat_name = self.model_inference(self.window_info.main_stat_name_rect, &image)?;
        let str_main_stat_value = self.model_inference(self.window_info.main_stat_value_rect, &image)?;

        let str_sub_stat0_name = self.model_inference(self.window_info.sub_stat_name_rect[0], &image)?;
        let str_sub_stat1_name = self.model_inference(self.window_info.sub_stat_name_rect[1], &image)?;
        let str_sub_stat2_name = self.model_inference(self.window_info.sub_stat_name_rect[2], &image)?;
        let str_sub_stat3_name = self.model_inference(self.window_info.sub_stat_name_rect[3], &image)?;
        let str_sub_stat0_value = self.model_inference(self.window_info.sub_stat_value_rect[0], &image)?;
        let str_sub_stat1_value = self.model_inference(self.window_info.sub_stat_value_rect[1], &image)?;
        let str_sub_stat2_value = self.model_inference(self.window_info.sub_stat_value_rect[2], &image)?;
        let str_sub_stat3_value = self.model_inference(self.window_info.sub_stat_value_rect[3], &image)?;

        let str_level = self.model_inference(self.window_info.level_rect, &image)?;
        let str_equip = self.model_inference(self.window_info.equip_rect, &image)?;

        Ok(StarRailRelicScanResult {
            name: str_title,
            main_stat_name: str_main_stat_name,
            main_stat_value: str_main_stat_value,
            sub_stat_name: [
                str_sub_stat0_name,
                str_sub_stat1_name,
                str_sub_stat2_name,
                str_sub_stat3_name,
            ],
            sub_stat_value: [
                str_sub_stat0_value,
                str_sub_stat1_value,
                str_sub_stat2_value,
                str_sub_stat3_value,
            ],
            level: parse_level(&str_level)?,
            equip: item.equip + &str_equip,
            star: item.star as i32,
            lock: item.lock,
            discard: item.discard,
        })
    }

    pub fn run(self, rx: Receiver<Option<SendItem>>) -> JoinHandle<Vec<StarRailRelicScanResult>> {
        std::thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;

            let is_verbose = self.config.verbose;
            let min_level = self.config.min_level;
            let info = self.window_info.clone();
            // todo remove dump mode to another scanner
            // let dump_mode = false;
            // let model = self.model.clone();
            // let panel_origin = Pos { x: self.window_info.panel_rect.left, y: self.window_info.panel_rect.top };

            for (_cnt, item) in rx.into_iter().enumerate() {
                let item = match item {
                    Some(v) => v,
                    None => break,
                };

                let result = match self.scan_item_image(item) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("识别错误: {}", e);
                        continue;
                    },
                };

                if is_verbose {
                    info!("{:?}", result);
                }

                // progress_bar.inc(1);
                // progress_bar.set_message(format!(
                //     "{}{}: {}",
                //     style(&result.name).bold().cyan(),
                //     style(format!("({})", result.level)).yellow(),
                //     style(&result.main_stat_name).dim()
                // ));

                if result.level < min_level {
                    info!(
                        "找到满足最低等级要求 {} 的物品({})，准备退出……",
                        min_level, result.level
                    );
                    // token.cancel();
                    break;
                }

                if hash.contains(&result) {
                    consecutive_dup_count += 1;
                    warn!("识别到重复物品: {:#?}", result);
                } else {
                    consecutive_dup_count = 0;
                    hash.insert(result.clone());
                    results.push(result);
                }

                if consecutive_dup_count >= info.col && !self.config.ignore_dup {
                    error!("识别到连续多个重复物品，可能为翻页错误，或者为非背包顶部开始扫描");
                    // token.cancel();
                    break;
                }

                // if token.cancelled() {
                    // error!("扫描任务被取消");
                    // break;
                // }
            }

            info!("识别结束，非重复物品数量: {}", hash.len());

            // progress_bar.finish();
            // MULTI_PROGRESS.remove(&progress_bar);

            results
        })
    }
}

#[derive(Clone)]
struct RelicScannerWindowInfo {
    pub origin_pos: Pos,

    pub title_rect: Rect,
    pub main_stat_name_rect: Rect,
    pub main_stat_value_rect: Rect,
    pub sub_stat_name_rect: [Rect; 4],
    pub sub_stat_value_rect: [Rect; 4],

    pub level_rect: Rect,

    pub equip_rect: Rect,
    pub equipper_pos: Pos,
    pub item_count_rect: Rect,

    pub star_pos: Pos,
    pub lock_pos: Pos,
    pub discard_pos: Pos,

    pub panel_rect: Rect,

    pub col: i32,
}

impl From<&WindowInfoRepository> for RelicScannerWindowInfo {
    fn from(value: &WindowInfoRepository) -> Self {
        RelicScannerWindowInfo {
            origin_pos: value.get("window_origin_pos").unwrap(),
            title_rect: value.get("starrail_relic_title_rect").unwrap(),
            main_stat_name_rect: value.get("starrail_relic_main_stat_name_rect").unwrap(),
            main_stat_value_rect: value.get("starrail_relic_main_stat_value_rect").unwrap(),
            level_rect: value.get("starrail_relic_level_rect").unwrap(),
            equip_rect: value.get("starrail_relic_equip_rect").unwrap(),
            equipper_pos: value.get("starrail_relic_equipper_pos").unwrap(),
            item_count_rect: value.get("starrail_relic_item_count_rect").unwrap(),
            star_pos: value.get("starrail_relic_star_pos").unwrap(),
            lock_pos: value.get("starrail_relic_lock_pos").unwrap(),
            discard_pos: value.get("starrail_relic_discard_pos").unwrap(),

            panel_rect: value.get("starrail_repository_panel_rect").unwrap(),
            col: value.get("starrail_repository_item_col").unwrap(),

            sub_stat_name_rect: [
                value.get("starrail_relic_sub_stat0_name_rect").unwrap(),
                value.get("starrail_relic_sub_stat1_name_rect").unwrap(),
                value.get("starrail_relic_sub_stat2_name_rect").unwrap(),
                value.get("starrail_relic_sub_stat3_name_rect").unwrap(),
            ],
            sub_stat_value_rect: [
                value.get("starrail_relic_sub_stat0_value_rect").unwrap(),
                value.get("starrail_relic_sub_stat1_value_rect").unwrap(),
                value.get("starrail_relic_sub_stat2_value_rect").unwrap(),
                value.get("starrail_relic_sub_stat3_value_rect").unwrap(),
            ],
        }
    }
}

pub struct StarRailRelicScanner {
    scanner_config: StarRailRelicScannerConfig,

    window_info: RelicScannerWindowInfo,
    window_info_clone: WindowInfoRepository,

    game_info: GameInfo,

    match_colors_star: [Color; 5],
    match_colors_lock: [Color; 3],
    match_colors_discard: [Color; 3],
    match_colors_equipper: [(&'static str, Color); 43],
}

impl RequireWindowInfo for StarRailRelicScanner {
    fn require_window_info(window_info_builder: &mut yas::window_info::window_info_builder::WindowInfoBuilder) {
        <StarRailRepositoryScanController as RequireWindowInfo>::require_window_info(window_info_builder);

        // window_info_builder.add_required_key("window_origin_pos");
        window_info_builder.add_required_key("starrail_relic_title_rect");
        window_info_builder.add_required_key("starrail_relic_main_stat_name_rect");
        window_info_builder.add_required_key("starrail_relic_main_stat_value_rect");
        window_info_builder.add_required_key("starrail_relic_level_rect");
        window_info_builder.add_required_key("starrail_relic_equip_rect");
        window_info_builder.add_required_key("starrail_relic_equipper_pos");
        window_info_builder.add_required_key("starrail_relic_item_count_rect");
        window_info_builder.add_required_key("starrail_relic_star_pos");
        window_info_builder.add_required_key("starrail_relic_lock_pos");
        window_info_builder.add_required_key("starrail_relic_discard_pos");
        window_info_builder.add_required_key("starrail_repository_item_col");
        window_info_builder.add_required_key("starrail_repository_panel_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat0_name_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat1_name_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat2_name_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat3_name_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat0_value_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat1_value_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat2_value_rect");
        window_info_builder.add_required_key("starrail_relic_sub_stat3_value_rect");
    }
}

struct SendItem {
    panel_image: RgbImage,
    equip: String,
    star: usize,
    lock: bool,
    discard: bool,
}

// constructor
impl StarRailRelicScanner {
    pub fn new(config: StarRailRelicScannerConfig, window_info: &WindowInfoRepository, game_info: GameInfo) -> Self {
        StarRailRelicScanner {
            scanner_config: config,
            window_info: RelicScannerWindowInfo::from(window_info),
            window_info_clone: window_info.clone(),
            game_info,
            match_colors_star: [
                Color::new(113, 119, 139), // todo
                Color::new(42, 143, 114), // todo
                Color::new(96, 142, 197), // 3
                Color::new(157, 117, 206), // 4
                Color::new(193, 158, 112), // 5
            ],
            match_colors_lock: [
                Color::new(18, 18, 18), // locked
                Color::new(249, 249, 249), // unlocked
                Color::new(116, 108, 99), // discard
            ],
            match_colors_discard: [
                Color::new(235, 77, 61), // discard
                Color::new(249, 249, 249), // not discard
                Color::new(115, 108, 98), // locked
            ],
            // todo use better match color set
            match_colors_equipper: [
                ("Argenti", Color::new(216, 174, 161)),
                ("Arlan", Color::new(146, 134, 124)),
                ("Asta", Color::new(188, 130, 117)),
                ("Bailu", Color::new(160, 127, 174)),
                ("BlackSwan", Color::new(252, 242, 239)),
                ("Blade", Color::new(191, 162, 162)),
                ("Bronya", Color::new(83, 66, 83)),
                ("Clara", Color::new(181, 107, 129)),
                ("DanHeng", Color::new(124, 100, 100)),
                ("DanHengImbibitorLunae", Color::new(181, 169, 163)),
                ("DrRatio", Color::new(134, 120, 143)),
                ("FuXuan", Color::new(231, 166, 145)),
                ("Gepard", Color::new(192, 199, 223)),
                ("Guinaifen", Color::new(219, 137, 111)),
                ("Hanya", Color::new(247, 238, 232)),
                ("Herta", Color::new(246, 239, 227)),
                ("Himeko", Color::new(177, 92, 85)),
                ("Hook", Color::new(190, 161, 86)),
                ("Huohuo", Color::new(230, 250, 250)),
                ("Jingliu", Color::new(193, 194, 218)),
                ("JingYuan", Color::new(169, 154, 147)),
                ("Kafka", Color::new(126, 50, 80)),
                ("Luka", Color::new(218, 198, 183)),
                ("Luocha", Color::new(191, 160, 116)),
                ("Lynx", Color::new(247, 213, 197)),
                ("March7th", Color::new(251, 243, 243)),
                ("Misha", Color::new(234, 215, 213)),
                ("Natasha", Color::new(238, 208, 196)),
                ("Pela", Color::new(241, 217, 217)),
                ("Qingque", Color::new(18, 27, 11)),
                ("RuanMei", Color::new(129, 101, 101)),
                ("Sampo", Color::new(241, 217, 213)),
                ("Seele", Color::new(91, 65, 111)),
                ("Serval", Color::new(158, 141, 150)),
                ("SilverWolf", Color::new(222, 210, 210)),
                ("Sushang", Color::new(101, 65, 58)),
                ("Tingyun", Color::new(127, 116, 57)),
                ("TopazNumby", Color::new(254, 250, 246)),
                ("Trailblazer_Preservation", Color::new(153, 125, 111)),
                ("Welt", Color::new(158, 114, 99)),
                ("Xueyi", Color::new(250, 242, 230)),
                ("Yanqing", Color::new(255, 242, 232)),
                ("Yukong", Color::new(174, 167, 174))
            ],
        }
    }
}

impl StarRailRelicScanner {
    pub fn get_star(&self) -> Result<usize> {
        let pos = self.window_info.origin_pos + self.window_info.star_pos;
        let color = capture::get_color(pos)?;

        let (index, _) = self.match_colors_star
            .iter()
            .enumerate()
            .min_by_key(|&(_, match_color)| match_color.distance(&color))
            .unwrap();

        Ok(index + 1)
    }

    pub fn get_lock(&self) -> Result<bool> {
        let pos = self.window_info.origin_pos + self.window_info.lock_pos;
        let color = capture::get_color(pos)?;

        let (index, _) = self.match_colors_lock
            .iter()
            .enumerate()
            .min_by_key(|&(_, match_color)| match_color.distance(&color))
            .unwrap();

        Ok(index == 0)
    }

    pub fn get_discard(&self) -> Result<bool> {
        let pos = self.window_info.origin_pos + self.window_info.discard_pos;
        let color = capture::get_color(pos)?;

        let (index, _) = self.match_colors_discard
            .iter()
            .enumerate()
            .min_by_key(|&(_, match_color)| match_color.distance(&color))
            .unwrap();

        Ok(index == 0)
    }

    pub fn get_equipper(&self) -> Result<String> {
        let pos = self.window_info.origin_pos + self.window_info.equipper_pos;
        let color = capture::get_color(pos)?;

        let (name, _) = self.match_colors_equipper
            .iter()
            .min_by_key(|&(_, match_color)| match_color.distance(&color))
            .unwrap();

        Ok(name.to_string())
    }

    pub fn get_item_count(&self, ocr_model: &OCRModel) -> Result<i32> {
        let count = self.scanner_config.number;
        let item_name = "遗器";

        let max_count = 1500;
        if count > 0 {
            return Ok(max_count.min(count));
        }

        let im = match self.window_info.item_count_rect
            .capture_relative(self.window_info.origin_pos)
        {
            Ok(im) => im,
            Err(e) => {
                error!("Error when capturing item count: {}", e);
                return Ok(max_count);
            },
        };

        // todo use better preprocess function set
        let im_grayed = to_gray(&im);
        let (im_preprocessed, preprocess_flag) = pre_process(im_grayed);
        assert!(preprocess_flag);

        let s = match ocr_model.inference_string(&im_preprocessed) {
            Ok(s) => s,
            Err(e) => {
                error!("Error when inferring item count: {}", e);
                return Ok(max_count);
            },
        };

        info!("物品信息: {}", s);

        if s.starts_with(item_name) {
            let chars = s.chars().collect::<Vec<char>>();
            let count_str = chars[4..chars.len() - 5].iter().collect::<String>();
            Ok(match count_str.parse::<usize>() {
                Ok(v) => (v as i32).min(max_count),
                Err(_) => max_count,
            })
        } else {
            Ok(max_count)
        }
    }

    pub fn scan(&mut self) -> Result<Vec<StarRailRelicScanResult>> {
        info!("开始扫描，使用鼠标右键中断扫描");

        let now = SystemTime::now();

        let (tx, rx) = mpsc::channel::<Option<SendItem>>();
        // let token = self.cancellation_token.clone();

        let model = {
            let model_bytes = include_bytes!("./models/model_training.onnx");
            let index_to_world = include_str!("./models/index_2_word.json");

            OCRModel::new(
                model_bytes, index_to_world
            )
        }?;
        let count = self.get_item_count(&model)?;

        let worker = RelicScannerWorker::new(
            model,
            self.window_info.clone(),
            self.scanner_config.clone()
        );

        let join_handle = worker.run(rx);

        // let worker = self.worker(rx, count, token);

        self.send(&tx, count);

        match tx.send(None) {
            Ok(_) => info!("扫描结束，等待识别线程结束，请勿关闭程序"),
            Err(_) => info!("扫描结束，识别已完成"),
        }

        match join_handle.join() {
            Ok(v) => {
                info!("识别耗时: {:?}", now.elapsed()?);
                Ok(v)
            },
            Err(_) => Err(anyhow::anyhow!("识别线程出现错误")),
        }
    }

    fn send(&mut self, tx: &Sender<Option<SendItem>>, count: i32) {
        let controller =  Rc::new(RefCell::new(StarRailRepositoryScanController::new(
            self.scanner_config.starrail_repo_scan_logic_config.clone(),
            &self.window_info_clone,
            // todo normalize types
            count as usize,
            self.game_info.clone(),
        )));
        let mut generator = StarRailRepositoryScanController::into_generator(controller.clone());

        loop {
            let pinned_generator = Pin::new(&mut generator);
            match pinned_generator.resume(()) {
                CoroutineState::Yielded(_) => {
                    // let image = self.capture_panel().unwrap();
                    let panel_image = controller.borrow().capture_panel().unwrap();
                    let equip = self.get_equipper().unwrap();
                    let star = self.get_star().unwrap();
                    let lock = self.get_lock().unwrap();
                    let discard = self.get_discard().unwrap();

                    // todo normalize types
                    if (star as i32) < self.scanner_config.min_star {
                        info!(
                            "找到满足最低星级要求 {} 的物品，准备退出……",
                            self.scanner_config.min_star
                        );
                        break;
                    }

                    if tx.send(Some(SendItem { panel_image, equip, star, lock, discard })).is_err() {
                        break;
                    }

                    // scanned_count += 1;
                },
                CoroutineState::Complete(result) => {
                    match result {
                        Err(e) => error!("扫描发生错误：{}", e),
                        Ok(value) => {
                            match value {
                                ReturnResult::Interrupted => info!("用户中断"),
                                ReturnResult::Finished => ()
                            }
                        }
                    }

                    break;
                }
            }
        }
    }
}