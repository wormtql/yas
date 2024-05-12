use std::{cell::RefCell, ops::{Coroutine, CoroutineState}, pin::Pin, rc::Rc, sync::mpsc::{self, Sender}, time::SystemTime};

use anyhow::Result;
use clap::FromArgMatches;
use image::RgbImage;
use log::{error, info};

use yas::capture::{Capturer, GenericCapturer};
use yas::game_info::GameInfo;
use yas::ocr::{ImageToText, yas_ocr_model};
use yas::positioning::Pos;
use yas::utils::color_distance;
use yas::window_info::{FromWindowInfoRepository, WindowInfoRepository};

use crate::scanner::relic_scanner::match_colors::{MATCH_COLORS, MatchColors};
use crate::scanner::relic_scanner::message_items::SendItem;
use crate::scanner::relic_scanner::relic_scanner_window_info::RelicScannerWindowInfo;
use crate::scanner::relic_scanner::relic_scanner_worker::RelicScannerWorker;
use crate::scanner::relic_scanner::scan_result::StarRailRelicScanResult;
use crate::scanner_controller::repository_layout::{ReturnResult, StarRailRepositoryScanController, StarRailRepositoryScannerLogicConfig};

use super::relic_scanner_config::StarRailRelicScannerConfig;

pub struct StarRailRelicScanner {
    scanner_config: StarRailRelicScannerConfig,
    window_info: RelicScannerWindowInfo,
    game_info: GameInfo,
    image_to_text: Box<dyn ImageToText<RgbImage> + Send>,
    controller: Rc<RefCell<StarRailRepositoryScanController>>,
    capturer: Rc<dyn Capturer<RgbImage>>,

    match_colors: MatchColors,
}

// constructor
impl StarRailRelicScanner {
    fn get_image_to_text() -> Result<Box<dyn ImageToText<RgbImage> + Send>> {
        let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(
            yas_ocr_model!("./models/model_training.onnx", "./models/index_2_word.json")?
        );
        // let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(yas::ocr::PPOCRChV4RecInfer::new()?);
        Ok(model)
    }

    fn get_capturer() -> Result<Rc<dyn Capturer<RgbImage>>> {
        Ok(Rc::new(GenericCapturer::new()?))
    }

    pub fn new(
        window_info_repo: &WindowInfoRepository,
        config: StarRailRelicScannerConfig,
        controller_config: StarRailRepositoryScannerLogicConfig,
        game_info: GameInfo
    ) -> Result<Self> {
        Ok(StarRailRelicScanner {
            scanner_config: config,
            window_info: RelicScannerWindowInfo::from_window_info_repository(
                game_info.window.to_rect_usize().size(),
                game_info.ui,
                game_info.platform,
                window_info_repo
            )?,
            controller: Rc::new(RefCell::new(StarRailRepositoryScanController::new(
                window_info_repo,
                controller_config,
                game_info.clone()
            )?)),
            game_info,
            image_to_text: Self::get_image_to_text()?,
            capturer: Self::get_capturer()?,

            match_colors: MATCH_COLORS,
        })
    }

    pub fn from_arg_matches(
        window_info_repo: &WindowInfoRepository,
        arg_matches: &clap::ArgMatches,
        game_info: GameInfo,
    ) -> Result<Self> {
        let window_info = RelicScannerWindowInfo::from_window_info_repository(
            game_info.window.to_rect_usize().size(),
            game_info.ui,
            game_info.platform,
            window_info_repo
        )?;
        Ok(StarRailRelicScanner {
            scanner_config: StarRailRelicScannerConfig::from_arg_matches(arg_matches)?,
            window_info,
            controller: Rc::new(RefCell::new(
                StarRailRepositoryScanController::from_arg_matches(window_info_repo, arg_matches, game_info.clone())?
            )),
            game_info,
            image_to_text: Self::get_image_to_text()?,
            capturer: Self::get_capturer()?,
            match_colors: MATCH_COLORS,
        })
    }
}

impl StarRailRelicScanner {
    pub fn capture_panel(&self) -> Result<RgbImage> {
        self.capturer.capture_relative_to(
            self.window_info.panel_rect.to_rect_i32(),
            self.game_info.window.origin()
        )
    }

    pub fn get_star(&self) -> Result<usize> {
        let pos: Pos<i32> = Pos {
            x: self.game_info.window.left + self.window_info.star_pos.x as i32,
            y: self.game_info.window.top + self.window_info.star_pos.y as i32,
        };
        let color = self.capturer.capture_color(pos)?;

        let (index, _) = self.match_colors.match_colors_star
            .iter()
            .enumerate()
            .min_by_key(|&(_, match_color)| color_distance(match_color, &color))
            .unwrap();

        Ok(index + 1)
    }

    pub fn get_lock(&self) -> Result<bool> {
        let pos: Pos<i32> = Pos {
            x: self.game_info.window.left + self.window_info.lock_pos.x as i32,
            y: self.game_info.window.top + self.window_info.lock_pos.y as i32,
        };
        let color = self.capturer.capture_color(pos)?;

        let (index, _) = self.match_colors.match_colors_lock
            .iter()
            .enumerate()
            .min_by_key(|&(_, match_color)| color_distance(match_color, &color))
            .unwrap();

        Ok(index == 0)
    }

    pub fn get_discard(&self) -> Result<bool> {
        let pos: Pos<i32> = Pos {
            x: self.game_info.window.left + self.window_info.discard_pos.x as i32,
            y: self.game_info.window.top + self.window_info.discard_pos.y as i32,
        };
        let color = self.capturer.capture_color(pos)?;

        let (index, _) = self.match_colors.match_colors_discard
            .iter()
            .enumerate()
            .min_by_key(|&(_, match_color)| color_distance(match_color, &color))
            .unwrap();

        Ok(index == 0)
    }

    pub fn get_equipper(&self) -> Result<String> {
        let pos: Pos<i32> = Pos {
            x: self.game_info.window.left + self.window_info.equipper_pos.x as i32,
            y: self.game_info.window.top + self.window_info.equipper_pos.y as i32,
        };
        let color = self.capturer.capture_color(pos)?;

        let (name, _) = self.match_colors.match_colors_equipper
            .iter()
            .min_by_key(|&(_, match_color)| color_distance(match_color, &color))
            .unwrap();

        Ok(name.to_string())
    }

    pub fn get_item_count(&self) -> Result<i32> {
        let count = self.scanner_config.number;
        let item_name = "遗器数量";

        let max_count = 2000;
        if count > 0 {
            return Ok(max_count.min(count));
        }

        let im = self.capturer.capture_relative_to(
            self.window_info.item_count_rect.to_rect_i32(),
            self.game_info.window.origin()
        )?;
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

    pub fn scan(&mut self) -> Result<Vec<StarRailRelicScanResult>> {
        info!("开始扫描，使用鼠标右键中断扫描");

        let now = SystemTime::now();
        let (tx, rx) = mpsc::channel::<Option<SendItem>>();
        // let token = self.cancellation_token.clone();
        let count = self.get_item_count()?;
        let worker = RelicScannerWorker::new(
            self.window_info.clone(),
            self.scanner_config.clone()
        )?;

        let join_handle = worker.run(rx);
        info!("Worker created");

        self.send(&tx, count);

        match tx.send(None) {
            Ok(_) => info!("扫描结束，等待识别线程结束，请勿关闭程序"),
            Err(_) => info!("扫描结束，识别已完成"),
        }

        let average_inference_time = self.image_to_text.get_average_inference_time();
        if let Some(t) = average_inference_time {
            let ms = t.as_micros() as f64 / 1000.0;
            info!("平均模型推理时间：{} ms", ms);
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
        let mut generator = StarRailRepositoryScanController::get_generator(
            self.controller.clone(),
            count as usize
        );

        loop {
            let pinned_generator = Pin::new(&mut generator);
            match pinned_generator.resume(()) {
                CoroutineState::Yielded(_) => {
                    // let image = self.capture_panel().unwrap();
                    let panel_image = self.capture_panel().unwrap();
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