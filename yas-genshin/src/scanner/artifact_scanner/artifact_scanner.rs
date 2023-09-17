use image::{RgbImage, GenericImageView};
use yas::{capture::capture, common::{color::Color, positioning::{Rect, Pos}, RelativeCapturable}, core::WindowInfo, window_info::require_window_info::RequireWindowInfo, inference::{model::OCRModel, pre_process::{pre_process, to_gray}}};
use std::{ops::{Generator, GeneratorState}, pin::Pin, rc::Rc, cell::RefCell, sync::{mpsc::{Receiver, Sender}, Arc}, thread::JoinHandle, os::windows::thread};

use crate::scanner_controller::repository_layout::{scan_logic::GenshinRepositoryScanController, config::GenshinRepositoryScannerLogicConfig};

use super::{artifact_scanner_config::GenshinArtifactScannerConfig};
use anyhow::Result;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct GenshinArtifactScanResult {
    pub name: String,
    pub main_stat_name: String,
    pub main_stat_value: String,
    pub sub_stat: [String; 4],
    pub equip: String,
    pub level: i32,
    pub star: i32,
}

struct ArtifactScannerWorker {

    model: OCRModel,
    rx: Receiver<Option<SendItem>>,
    window_info: ArtifactScannerWindowInfo,
    config: GenshinArtifactScannerConfig,
}

fn parse_level(s: &str) -> Result<usize> {
    let pos = s.find('+');

    if pos.is_none() {
        let level = s.parse::<i32>()?;
        return anyhow::Ok(level);
    }

    let level = s[pos.unwrap()..].parse::<i32>()?;
    return anyhow::Ok(level);
}

impl ArtifactScannerWorker {
    pub fn new(
        rx: Receiver<Option<SendItem>>,
        model: OCRModel,
        window_info: ArtifactScannerWindowInfo,
        config: GenshinArtifactScannerConfig,
    ) -> Self {
        ArtifactScannerWorker {
            model,
            rx,
            window_info,
            config,
        }
    }

    fn model_inference(&self, pos: Rect, captured_img: &RgbImage) -> Result<String> {
        // todo move dump mode into a scanner
        if dump_mode {
            captured_img.save(Path::new("dumps").join(format!("{}_{}.rgb.png", name, cnt)))?;
        }

        let relative_rect = pos;
        relative_rect.translate(Pos {
            x: -self.window_info.panel_pos.left,
            y: -self.window_info.panel_pos.top,
        });

        let raw_img = captured_img.view(
            relative_rect.left, relative_rect.top, relative_rect.width, relative_rect.height
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

        let processed_img = match pre_process(raw_img_grayed) {
            Some(im) => im,
            None => return Err(anyhow::anyhow!("图像预处理失败")),
        };

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

    fn scan_item_image(&self, item: SendItem) -> Result<GenshinArtifactScanResult> {
        let image = &item.panel_image;

        let str_title = self.model_inference(self.window_info.title_pos, &image)?;
        let str_main_stat_name = self.model_inference(self.window_info.main_stat_name_pos, &image)?;
        let str_main_stat_value = self.model_inference(self.window_info.main_stat_value_pos, &image)?;
    
        let genshin_info = &info.inner_genshin();
    
        let str_sub_stat0 = self.model_inference(self.window_info.sub_stat_pos[0], &image)?;
        let str_sub_stat1 = self.model_inference(self.window_info.sub_stat_pos[1], &image)?;
        let str_sub_stat2 = self.model_inference(self.window_info.sub_stat_pos[2], &image)?;
        let str_sub_stat3 = self.model_inference(self.window_info.sub_stat_pos[3], &image)?;
    
        let str_level = self.model_inference(self.window_info.level_pos, &image)?;
        let str_equip = self.model_inference(self.window_info.item_equip_pos, &image)?;
    
        anyhow::Ok(GenshinArtifactScanResult {
            name: str_title,
            main_stat_name: str_main_stat_name,
            main_stat_value: str_main_stat_value,
            sub_stat: [
                str_sub_stat0,
                str_sub_stat1,
                str_sub_stat2,
                str_sub_stat3,
            ],
            level: parse_level(&str_level)?,
            equip: str_equip,
            star: item.star as i32,
        })
    }

    pub fn run(self) -> JoinHandle<Vec<GenshinArtifactScanResult>> {
        thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;

            let is_verbose = self.config.verbose;
            let min_level = self.config.min_level;
            let info = self.window_info.clone();
            // todo remove dump mode to another scanner
            let dump_mode = false;
            let model = self.model.clone();
            let panel_origin = Pos { x: self.window_info.panel_pos.left, y: self.window_info.panel_pos.top };

            for (cnt, item) in self.rx.into_iter().enumerate() {
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

                if consecutive_dup_count >= info.item_row && !self.config.ignore_dup {
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
struct ArtifactScannerWindowInfo {
    pub origin: Pos,

    pub title_pos: Rect,
    pub main_stat_name_pos: Rect,
    pub main_stat_value_pos: Rect,
    pub sub_stat_pos: [Rect; 4],

    pub level_pos: Rect,

    pub item_equip_pos: Rect,
    pub item_count_pos: Rect,

    pub star: Pos,

    pub panel_pos: Rect,
}

impl From<&WindowInfo> for ArtifactScannerWindowInfo {
    fn from(value: &WindowInfo) -> Self {
        ArtifactScannerWindowInfo {
            origin: value.get("window_origin"),
            title_pos: value.get("genshin_artifact_title_pos"),
            main_stat_name_pos: value.get("genshin_artifact_main_stat_name_pos"),
            main_stat_value_pos: value.get("genshin_artifact_main_stat_value_pos"),
            level_pos: value.get("genshin_artifact_level_pos"),
            item_equip_pos: value.get("genshin_artifact_item_equip_pos"),
            item_count_pos: value.get("genshin_artifact_item_count_pos"),
            star: value.get("genshin_artifact_star"),
            panel_pos: value.get("genshin_repository_panel_pos"),

            sub_stat_pos: [
                value.get("genshin_artifact_sub_stat0"),
                value.get("genshin_artifact_sub_stat1"),
                value.get("genshin_artifact_sub_stat2"),
                value.get("genshin_artifact_sub_stat3"),
            ]
        }
    }
}

pub struct GenshinArtifactScanner {
    pub scanner_config: GenshinArtifactScannerConfig,

    pub window_info: ArtifactScannerWindowInfo,
    window_info_clone: WindowInfo,
}

impl RequireWindowInfo for GenshinArtifactScanner {
    fn require_window_info(window_info_builder: &mut yas::window_info::window_info_builder::WindowInfoBuilder) {
        <GenshinRepositoryScanController as RequireWindowInfo>::require_window_info(window_info_builder);

        window_info_builder.add_required_key("window_origin");
        window_info_builder.add_required_key("genshin_artifact_title_pos");
        window_info_builder.add_required_key("genshin_artifact_main_stat_name_pos");
        window_info_builder.add_required_key("genshin_artifact_main_stat_value_pos");
        window_info_builder.add_required_key("genshin_artifact_level_pos");
        window_info_builder.add_required_key("genshin_artifact_item_equip_pos");
        window_info_builder.add_required_key("genshin_artifact_item_count_pos");
        window_info_builder.add_required_key("genshin_artifact_star");
    }
}

struct SendItem {
    panel_image: RgbImage,
    star: usize,
}

// constructor
impl GenshinArtifactScanner {
    pub fn new(config: GenshinArtifactScannerConfig, window_info: &WindowInfo) -> Self {
        GenshinArtifactScanner {
            scanner_config: config.genshin_repo_scan_logic_config.clone(),
            window_info: ArtifactScannerWindowInfo::from(window_info),
            window_info_clone: window_info.clone(),
        }
    }
}

impl GenshinArtifactScanner {
    pub fn capture_panel(&mut self) -> Result<RgbImage> {
        Rect::from(&self.scan_info.panel_pos).capture_relative(&self.scan_info.origin)
    }

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

        anyhow::Ok(ret as u8)
    }

    pub fn get_item_count(&self) -> Result<usize> {
        let count = self.config.number;
        let item_name = "圣遗物";

        let max_count = 1800;
        if count > 0 {
            return max_count.min(count);
        }

        let im = match self.window_info.item_count_pos
            .capture_relative(self.window_info.origin)
        {
            Ok(im) => im,
            Err(e) => {
                error!("Error when capturing item count: {}", e);
                return max_count;
            },
        };

        let s = match self.model.inference_string(&im) {
            Ok(s) => s,
            Err(e) => {
                error!("Error when inferring item count: {}", e);
                return max_count;
            },
        };

        info!("物品信息: {}", s);

        if s.starts_with(item_name) {
            let chars = s.chars().collect::<Vec<char>>();
            let count_str = chars[4..chars.len() - 5].iter().collect::<String>();
            match count_str.parse::<usize>() {
                Ok(v) => v.min(max_count),
                Err(_) => max_count,
            }
        } else {
            max_count
        }
    }

    pub fn scan(&mut self) -> Result<Vec<GenshinArtifactScanResult>> {
        info!("开始扫描，使用鼠标右键中断扫描");

        let now = SystemTime::now();
        let count = self.get_item_count();

        let (tx, rx) = mpsc::channel::<Option<SendItem>>();
        // let token = self.cancellation_token.clone();

        let model = {
            let model_bytes = include_bytes!("./models/model_training.onnx");
            let index_to_world = include_str!("./models/index_2_word.json");

            OCRModel::new(
                model_bytes, index_to_world
            )
        };

        let worker = ArtifactScannerWorker::new(
            rx,
            model,
            self.window_info.clone(),
            self.scanner_config.clone()
        );

        let join_handle = worker.run();

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

    fn send(&mut self, tx: &Sender<Option<SendItem>>, count: usize) {
        let mut controller =  Rc::new(RefCell::new(GenshinRepositoryScanController::new(
            self.scanner_config.genshin_repo_scan_logic_config.clone(),
            &self.window_info_clone,
            count
        )));
        let mut generator = GenshinRepositoryScanController::into_generator(controller.clone());
        let mut pinned_generator = Pin::new(&mut generator);
        
        loop {
            match pinned_generator.resume(()) {
                GeneratorState::Yielded(_) => {
                    // let image = self.capture_panel().unwrap();
                    let image = controller.borrow().capture_panel().unwrap();
                    let star = self.get_star().unwrap();

                    if star < self.scanner_config.min_star {
                        info!(
                            "找到满足最低星级要求 {} 的物品，准备退出……",
                            self.scanner_config.min_star
                        );
                        break;
                    }

                    if tx.send(Some(SendItem { panel_image: image, star: star })).is_err() {
                        break;
                    }

                    scanned_count += 1;
                },
                GeneratorState::Complete(_) => {
                    break;
                }
            }
        }

        
    }
}