use image::RgbImage;
use yas::{capture::capture, common::color::Color};
use std::{ops::{Generator, GeneratorState}, pin::Pin, rc::Rc, cell::RefCell};

use crate::scanner_controller::repository_layout::{scan_logic::GenshinRepositoryScanController, config::GenshinRepositoryScannerLogicConfig};

use super::{artifact_scanner_config::GenshinArtifactScannerConfig, artifact_scanner_scan_info::GenshinArtifactScannerScanInfo};
use anyhow::Result;

pub struct GenshinArtifactScanner {
    pub scanner_config: GenshinArtifactScannerConfig,
    pub controller_config: GenshinRepositoryScannerLogicConfig,
    pub scan_info: GenshinArtifactScannerScanInfo,
}

struct SendItem {
    panel_image: RgbImage,
    star: usize,
}

impl GenshinArtifactScanner {
    pub fn capture_panel(&mut self) -> Result<RgbImage> {
        Rect::from(&self.scan_info.panel_pos).capture_relative(&self.scan_info.origin)
    }

    pub fn get_star(&self) -> Result<usize> {
        let color = capture::get_color(&self.scan_info.origin + &self.scan_info.star)?;

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
        let item_name = match TARGET_GAME.get().unwrap() {
            Game::Genshin => "圣遗物",
            Game::StarRail => "遗器数量",
        };

        let max_count = match crate::TARGET_GAME.get().unwrap() {
            Game::Genshin => 1800,
            Game::StarRail => 1500,
        };

        if count > 0 {
            return max_count.min(count);
        }

        let im = match Rect::from(&self.scan_info.item_count_pos)
            .capture_relative(&self.scan_info.origin)
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

    pub fn scan(&mut self) -> Result<Vec<ScanResult>> {
        info!("开始扫描，使用鼠标右键中断扫描");

        let now = SystemTime::now();
        let count = self.get_item_count();

        let (tx, rx) = mpsc::channel::<Option<SendItem>>();
        let token = self.cancellation_token.clone();

        let worker = self.worker(rx, count, token);

        self.send(&tx, count);

        match tx.send(None) {
            Ok(_) => info!("扫描结束，等待识别线程结束，请勿关闭程序"),
            Err(_) => info!("扫描结束，识别已完成"),
        }

        match worker.join() {
            Ok(v) => {
                info!("识别耗时: {:?}", now.elapsed()?);
                Ok(v)
            },
            Err(_) => Err(anyhow::anyhow!("识别线程出现错误")),
        }
    }

    fn send(&mut self, tx: &Sender<Option<SendItem>>, count: usize) {
        let mut controller =  Rc::new(RefCell::new(GenshinRepositoryScanController::new(
            self.scanner_config.clone(),
            self.scan_info.genshin_repo_scan_info.clone(),
            count
        )));
        let mut generator = GenshinRepositoryScanController::into_generator(controller.clone());
        let mut pinned_generator = Pin::new(&mut generator);
        
        loop {
            match pinned_generator.resume(()) {
                GeneratorState::Yielded(_) => {
                    let image = self.capture_panel().unwrap();
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

    fn worker(
        &self,
        rx: Receiver<Option<SendItem>>,
        count: usize,
        token: CancellationToken,
    ) -> JoinHandle<Vec<ScanResult>> {
        let is_verbose = self.config.verbose;
        let min_level = self.config.min_level;
        let info = self.scan_info.clone();
        let dump_mode = self.config.dump_mode;
        let model = self.model.clone();
        let panel_origin = Rect::from(&self.scan_info.panel_pos).origin;
        let scan_item_image = match self {
            Scanner::Genshin(_) => GenshinScanner::scan_item_image,
            Scanner::StarRail(_) => StarRailScanner::scan_item_image,
        };
        let progress_bar = ProgressBar::new(count as u64)
            .with_prefix("识别属性")
            .with_style(PROGRESS_STYLE.clone());
        let progress_bar = MULTI_PROGRESS.add(progress_bar);

        thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;
            let model_inference = get_model_inference_func(dump_mode, model, panel_origin);

            for (cnt, item) in rx.into_iter().enumerate() {
                let item = match item {
                    Some(v) => v,
                    None => break,
                };

                let result = match scan_item_image(&model_inference, &info, item, cnt) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("识别错误: {}", e);
                        continue;
                    },
                };

                if is_verbose {
                    info!("{:?}", result);
                }

                progress_bar.inc(1);
                progress_bar.set_message(format!(
                    "{}{}: {}",
                    style(&result.name).bold().cyan(),
                    style(format!("({})", result.level)).yellow(),
                    style(&result.main_stat_name).dim()
                ));

                if result.level < min_level {
                    info!(
                        "找到满足最低等级要求 {} 的物品({})，准备退出……",
                        min_level, result.level
                    );
                    token.cancel();
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

                if consecutive_dup_count >= info.item_row && !CONFIG.ignore_dup {
                    error!("识别到连续多个重复物品，可能为翻页错误，或者为非背包顶部开始扫描");
                    token.cancel();
                    break;
                }

                if token.cancelled() {
                    error!("扫描任务被取消");
                    break;
                }
            }

            info!("识别结束，非重复物品数量: {}", hash.len());

            progress_bar.finish();
            MULTI_PROGRESS.remove(&progress_bar);

            results
        })
    }
}