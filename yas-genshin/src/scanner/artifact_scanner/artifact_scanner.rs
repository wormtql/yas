use std::{cell::RefCell, ops::{Coroutine, CoroutineState}, pin::Pin, rc::Rc, sync::{mpsc::{self, Sender}}, time::SystemTime};

use anyhow::Result;
use clap::FromArgMatches;
use image::RgbImage;
use log::{error, info};

use yas::capture;
use yas::capture::capture::RelativeCapturable;
use yas::common::color::Color;
use yas::game_info::GameInfo;
use yas::ocr::{ImageToText, yas_ocr_model};
use yas::window_info::FromWindowInfoRepository;
use yas::window_info::WindowInfoRepository;

use crate::scanner::artifact_scanner::artifact_scanner_worker::ArtifactScannerWorker;
use crate::scanner::artifact_scanner::message_items::SendItem;
use crate::scanner::artifact_scanner::scan_result::GenshinArtifactScanResult;
use crate::scanner_controller::{GenshinRepositoryControllerReturnResult, GenshinRepositoryScanController, GenshinRepositoryScannerLogicConfig};

use super::artifact_scanner_config::GenshinArtifactScannerConfig;
use super::ArtifactScannerWindowInfo;

pub struct GenshinArtifactScanner {
    scanner_config: GenshinArtifactScannerConfig,
    window_info: ArtifactScannerWindowInfo,
    game_info: GameInfo,
    image_to_text: Rc<dyn ImageToText<RgbImage>>,
    controller: Rc<RefCell<GenshinRepositoryScanController>>,
}

// constructor
impl GenshinArtifactScanner {
    fn get_image_to_text() -> Rc<dyn ImageToText<RgbImage>> {
        let model: Rc<dyn ImageToText<RgbImage>> = Rc::new(
            yas_ocr_model!("./models/model_training.onnx", "./models/index_2_word.json")?
        );
        model
    }

    pub fn new(
        window_info_repo: &WindowInfoRepository,
        config: GenshinArtifactScannerConfig,
        controller_config: GenshinRepositoryScannerLogicConfig,
        game_info: GameInfo,
    ) -> Result<Self> {
        Ok(Self {
            scanner_config: config,
            window_info: ArtifactScannerWindowInfo::from_window_info_repository(game_info.window, window_info_repo)?,
            game_info,
            image_to_text: Self::get_image_to_text(),
            // item count will be set later, once the scan starts
            controller: Rc::new(RefCell::new(
                GenshinRepositoryScanController::new(window_info_repo, controller_config, 0, game_info.clone())?
            )),
        })
    }

    pub fn from_arg_matches(
        window_info_repo: &WindowInfoRepository,
        arg_matches: &clap::ArgMatches,
        game_info: GameInfo,
    ) -> Result<Self> {
        Ok(GenshinArtifactScanner {
            scanner_config: GenshinArtifactScannerConfig::from_arg_matches(arg_matches)?,
            window_info: ArtifactScannerWindowInfo::from_window_info_repository(game_info.window, window_info_repo)?,
            game_info,
            image_to_text: Self::get_image_to_text(),
            controller: Rc::new(RefCell::new(
                GenshinRepositoryScanController::from_arg_matches(window_info_repo, arg_matches, 0, game_info.clone())?
            ))
        })
    }
}

impl GenshinArtifactScanner {
    pub fn capture_panel(&self) -> Result<RgbImage> {
        self.window_info.panel_rect.capture_relative(self.window_info.window_origin_pos)
    }

    pub fn get_star(&self) -> Result<usize> {
        let pos = self.game_info.window + self.window_info.star_pos;
        let color = capture::capture::get_color(pos)?;

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

    pub fn get_item_count(&self) -> Result<i32> {
        let count = self.scanner_config.number;
        let item_name = "圣遗物";

        let max_count = 1800;
        if count > 0 {
            return Ok(max_count.min(count));
        }

        let im = match self.window_info.item_count_rect
            .capture_relative(self.game_info.origin_pos)
        {
            Ok(im) => im,
            Err(e) => {
                error!("Error when capturing item count: {}", e);
                return Ok(max_count);
            }
        };

        let s = self.image_to_text.image_to_text(&im, false)?;

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

    pub fn scan(&mut self) -> Result<Vec<GenshinArtifactScanResult>> {
        info!("开始扫描，使用鼠标右键中断扫描");

        let now = SystemTime::now();
        let (tx, rx) = mpsc::channel::<Option<SendItem>>();
        // let token = self.cancellation_token.clone();
        let count = self.get_item_count()?;
        let worker = ArtifactScannerWorker::new(
            self.image_to_text.clone(),
            self.window_info.clone(),
            self.scanner_config.clone(),
        );

        let join_handle = worker.run(rx);

        self.send(&tx, count);

        match tx.send(None) {
            Ok(_) => info!("扫描结束，等待识别线程结束，请勿关闭程序"),
            Err(_) => info!("扫描结束，识别已完成"),
        }

        match join_handle.join() {
            Ok(v) => {
                info!("识别耗时: {:?}", now.elapsed()?);
                Ok(v)
            }
            Err(_) => Err(anyhow::anyhow!("识别线程出现错误")),
        }
    }

    fn send(&mut self, tx: &Sender<Option<SendItem>>, count: i32) {
        self.controller.borrow_mut().set_item_count(count as usize);
        let mut generator = GenshinRepositoryScanController::get_generator(self.controller.clone());

        loop {
            let pinned_generator = Pin::new(&mut generator);
            match pinned_generator.resume(()) {
                CoroutineState::Yielded(_) => {
                    let image = self.capture_panel().unwrap();
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
                }
                CoroutineState::Complete(result) => {
                    match result {
                        Err(e) => error!("扫描发生错误：{}", e),
                        Ok(value) => {
                            match value {
                                GenshinRepositoryControllerReturnResult::Interrupted => info!("用户中断"),
                                GenshinRepositoryControllerReturnResult::Finished => ()
                            }
                        }
                    }

                    break;
                }
            }
        }
    }
}