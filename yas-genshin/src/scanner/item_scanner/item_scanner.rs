use image::{RgbImage, GenericImageView};
use log::{error, info, warn};
use yas::{capture::capture::{self, RelativeCapturable}, common::{color::Color, positioning::{Rect, Pos}}, window_info::{require_window_info::RequireWindowInfo, window_info_repository::WindowInfoRepository}, inference::{model::OCRModel, pre_process::{pre_process, to_gray}}, game_info::GameInfo};
use std::{ops::{Generator, GeneratorState}, pin::Pin, rc::Rc, cell::RefCell, sync::mpsc::{Receiver, Sender, self}, thread::JoinHandle, collections::HashSet, time::SystemTime};

use crate::scanner_controller::repository_layout::scan_logic::{GenshinRepositoryScanController, ReturnResult};

use anyhow::Result;

use super::item_scanner_config::ItemScannerConfig;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ItemScanResult {
    pub name: String,
    pub amount: i32
}

struct ItemScannerWorker {
    model: OCRModel,
    window_info: ItemScannerWindowInfo,
    config: ItemScannerConfig,
}

impl ItemScannerWorker {
    pub fn new(
        model: OCRModel,
        window_info: ItemScannerWindowInfo,
        config: ItemScannerConfig,
    ) -> Self {
        ItemScannerWorker {
            model,
            window_info,
            config,
        }
    }

    fn model_inference(&self, pos: Rect, captured_img: &RgbImage) -> Result<String> {
        // todo move dump mode into a scanner
        // if dump_mode {
            // captured_img.save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        // }

        let relative_rect = pos.translate(Pos {
            x: -self.window_info.panel_pos.left,
            y: -self.window_info.panel_pos.top,
        });

        let raw_img = captured_img.view(
            relative_rect.left as u32, relative_rect.top as u32, relative_rect.width as u32, relative_rect.height as u32
        ).to_image();
        let raw_img_grayed = to_gray(&raw_img);

        let (processed_img, process_flag) = pre_process(raw_img_grayed);
        if !process_flag {
            return Ok(String::new());
        }

        let inference_result = self.model.inference_string(&processed_img)?;

        Ok(inference_result)
    }

    fn scan_item_image(&self, item: SendItem) -> Result<ItemScanResult> {
        let image = &item.panel_image;

        let str_title = self.model_inference(self.window_info.title_pos, &image)?;
        
        anyhow::Ok(ItemScanResult {
            name: str_title,
            amount: 0,
        })
    }

    pub fn run(self, rx: Receiver<Option<SendItem>>) -> JoinHandle<Vec<ItemScanResult>> {
        std::thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;

            let is_verbose = self.config.verbose;
            let info = self.window_info.clone();

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

                if hash.contains(&result) {
                    consecutive_dup_count += 1;
                    warn!("识别到重复物品: {:#?}", result);
                } else {
                    consecutive_dup_count = 0;
                    hash.insert(result.clone());
                    results.push(result);
                }

                if consecutive_dup_count >= info.col {
                    error!("识别到连续多个重复物品，可能为翻页错误，或者为非背包顶部开始扫描");
                    // token.cancel();
                    break;
                }
            }

            info!("识别结束，非重复物品数量: {}", hash.len());

            results
        })
    }
}

#[derive(Clone)]
struct ItemScannerWindowInfo {
    // window origin
    pub origin: Pos,

    pub title_pos: Rect,

    pub star: Pos,

    pub panel_pos: Rect,

    pub col: i32,
}

impl From<&WindowInfoRepository> for ItemScannerWindowInfo {
    fn from(value: &WindowInfoRepository) -> Self {
        ItemScannerWindowInfo {
            origin: value.get("window_origin").unwrap(),
            title_pos: value.get("genshin_artifact_title_pos").unwrap(),
            star: value.get("genshin_artifact_star").unwrap(),

            panel_pos: value.get("genshin_repository_panel_pos").unwrap(),
            col: value.get("genshin_repository_item_col").unwrap(),
        }
    }
}

pub struct GenshinItemScanner {
    scanner_config: ItemScannerConfig,

    window_info: ItemScannerWindowInfo,
    window_info_clone: WindowInfoRepository,

    game_info: GameInfo,
}

impl RequireWindowInfo for GenshinItemScanner {
    fn require_window_info(window_info_builder: &mut yas::window_info::window_info_builder::WindowInfoBuilder) {
        <GenshinRepositoryScanController as RequireWindowInfo>::require_window_info(window_info_builder);

        // window_info_builder.add_required_key("window_origin");
        window_info_builder.add_required_key("genshin_artifact_title_pos");
        window_info_builder.add_required_key("genshin_artifact_main_stat_name_pos");
        window_info_builder.add_required_key("genshin_artifact_main_stat_value_pos");
        window_info_builder.add_required_key("genshin_artifact_level_pos");
        window_info_builder.add_required_key("genshin_artifact_item_equip_pos");
        window_info_builder.add_required_key("genshin_artifact_item_count_pos");
        window_info_builder.add_required_key("genshin_artifact_star");
        window_info_builder.add_required_key("genshin_repository_item_col");
        window_info_builder.add_required_key("genshin_repository_panel_pos");
        window_info_builder.add_required_key("genshin_artifact_sub_stat0");
        window_info_builder.add_required_key("genshin_artifact_sub_stat1");
        window_info_builder.add_required_key("genshin_artifact_sub_stat2");
        window_info_builder.add_required_key("genshin_artifact_sub_stat3");
    }
}

struct SendItem {
    panel_image: RgbImage,
    star: usize,
}

// constructor
impl GenshinItemScanner {
    pub fn new(config: ItemScannerConfig, window_info: &WindowInfoRepository, game_info: GameInfo) -> Self {
        GenshinItemScanner {
            scanner_config: config,
            window_info: ItemScannerWindowInfo::from(window_info),
            window_info_clone: window_info.clone(),
            game_info
        }
    }
}

impl GenshinItemScanner {
    pub fn get_star(&self) -> Result<usize> {
        let pos = self.window_info.origin + self.window_info.star;
        let color = capture::get_color(pos)?;

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

        anyhow::Ok(ret)
    }

    pub fn get_item_count(&self, ocr_model: &OCRModel) -> Result<i32> {
        let count = self.scanner_config.number;
        let item_name = "圣遗物";

        let max_count = 1800;
        if count > 0 {
            return Ok(max_count.min(count));
        }

        let im = match self.window_info.item_count_pos
            .capture_relative(self.window_info.origin)
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

    pub fn scan(&mut self) -> Result<Vec<ItemScanResult>> {
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

        let worker = ArtifactScannerWorker::new(
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
        let controller =  Rc::new(RefCell::new(GenshinRepositoryScanController::new(
            self.scanner_config.genshin_repo_scan_logic_config.clone(),
            &self.window_info_clone,
            // todo normalize types
            count as usize,
            self.game_info.clone(),
        )));
        let mut generator = GenshinRepositoryScanController::into_generator(controller.clone());
        
        loop {
            let pinned_generator = Pin::new(&mut generator);
            match pinned_generator.resume(()) {
                GeneratorState::Yielded(_) => {
                    // let image = self.capture_panel().unwrap();
                    let image = controller.borrow().capture_panel().unwrap();
                    let star = self.get_star().unwrap();

                    // todo normalize types
                    if (star as i32) < self.scanner_config.min_star {
                        info!(
                            "找到满足最低星级要求 {} 的物品，准备退出……",
                            self.scanner_config.min_star
                        );
                        break;
                    }

                    if tx.send(Some(SendItem { panel_image: image, star: star })).is_err() {
                        break;
                    }

                    // scanned_count += 1;
                },
                GeneratorState::Complete(result) => {
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