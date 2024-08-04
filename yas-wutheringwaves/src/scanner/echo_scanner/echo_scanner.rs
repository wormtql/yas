use std::cell::RefCell;
use std::ops::{CoroutineState, Coroutine};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::SystemTime;

use anyhow::Result;
use image::RgbImage;
use log::{error, info};
use regex::Regex;
use clap::FromArgMatches;

use yas::capture::{Capturer, GenericCapturer};
use yas::game_info::GameInfo;
use yas::ocr::{ImageToText, yas_ocr_model};
use yas::window_info::{WindowInfoRepository, FromWindowInfoRepository};

use crate::scanner::echo_scanner::echo_scanner_config::WWEchoScannerConfig;
use crate::scanner::echo_scanner::echo_scanner_window_info::EchoScannerWindowInfo;
use crate::scanner::echo_scanner::echo_scanner_worker::WWEchoScannerWorker;
use crate::scanner::echo_scanner::message_item::SendItem;
use crate::scanner::echo_scanner::scan_result::WWEchoScanResult;
use crate::scanner_controller::{ReturnResult, WWRepositoryLayoutConfig, WWRepositoryLayoutScanController};

pub struct WWEchoScanner {
    scanner_config: WWEchoScannerConfig,
    window_info: EchoScannerWindowInfo,
    game_info: GameInfo,
    image_to_text: Box<dyn ImageToText<RgbImage> + Send>,
    controller: Rc<RefCell<WWRepositoryLayoutScanController>>,
    capturer: Rc<dyn Capturer<RgbImage>>,
}

impl WWEchoScanner {
    fn get_image_to_text() -> anyhow::Result<Box<dyn ImageToText<RgbImage> + Send>> {
        let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(
            yas_ocr_model!("./models/model_training.onnx", "./models/index_2_word.json")?
        );
        // let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(yas::ocr::PPOCRChV4RecInfer::new()?);
        Ok(model)
    }

    fn get_capturer() -> anyhow::Result<Rc<dyn Capturer<RgbImage>>> {
        Ok(Rc::new(GenericCapturer::new()?))
    }

    pub fn new(
        window_info_repo: &WindowInfoRepository,
        config: WWEchoScannerConfig,
        controller_config: WWRepositoryLayoutConfig,
        game_info: GameInfo
    ) -> anyhow::Result<Self> {
        Ok(Self {
            scanner_config: config,
            window_info: EchoScannerWindowInfo::from_window_info_repository(
                game_info.window.to_rect_usize().size(),
                game_info.ui,
                game_info.platform,
                window_info_repo
            )?,
            controller: Rc::new(RefCell::new(WWRepositoryLayoutScanController::new(
                window_info_repo,
                controller_config,
                game_info.clone()
            )?)),
            game_info,
            image_to_text: Self::get_image_to_text()?,
            capturer: Self::get_capturer()?,
        })
    }

    pub fn from_arg_matches(
        window_info_repo: &WindowInfoRepository,
        arg_matches: &clap::ArgMatches,
        game_info: GameInfo,
    ) -> Result<Self> {
        let window_info = EchoScannerWindowInfo::from_window_info_repository(
            game_info.window.to_rect_usize().size(),
            game_info.ui,
            game_info.platform,
            window_info_repo
        )?;
        Ok(Self {
            scanner_config: WWEchoScannerConfig::from_arg_matches(arg_matches)?,
            window_info,
            controller: Rc::new(RefCell::new(
                WWRepositoryLayoutScanController::from_arg_matches(window_info_repo, arg_matches, game_info.clone())?
            )),
            game_info,
            image_to_text: Self::get_image_to_text()?,
            capturer: Self::get_capturer()?,
        })
    }
}

impl WWEchoScanner {
    fn capture_panel(&self) -> Result<RgbImage> {
        self.capturer.capture_relative_to(
            self.window_info.panel_rect.to_rect_i32(),
            self.game_info.window.origin()
        )
    }

    /// Get Echo count
    fn get_item_count(&self) -> Result<usize> {
        let max_count = 2000;
        if let Some(c) = self.scanner_config.number {
            return Ok(max_count.min(c))
        }

        let im = self.capturer.capture_relative_to(
            self.window_info.item_count_rect.to_rect_i32(),
            self.game_info.window.origin()
        )?;
        let s = self.image_to_text.image_to_text(&im, false)?;
        info!("物品信息: {}", s);

        let re = Regex::new(r"\s*声骸\s*(?<count>[0-9]+)/2000")?;
        let match_result = re.captures(&s);
        if let Some(r) = match_result {
            let count = r["count"].parse::<usize>()?;
            return Ok(count.min(max_count));
        } else {
            return Ok(max_count)
        }
    }

    pub fn scan(&mut self) -> Result<Vec<WWEchoScanResult>> {
        info!("开始扫描，使用鼠标右键中断扫描");

        let now = SystemTime::now();
        let (tx, rx) = mpsc::channel::<Option<SendItem>>();
        // let token = self.cancellation_token.clone();
        let count = self.get_item_count()?;
        let worker = WWEchoScannerWorker::new(
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

    fn send(&mut self, tx: &Sender<Option<SendItem>>, count: usize) {
        let mut generator = WWRepositoryLayoutScanController::get_generator(
            self.controller.clone(),
            count
        );

        loop {
            let pinned_generator = Pin::new(&mut generator);
            match pinned_generator.resume(()) {
                CoroutineState::Yielded(_) => {
                    let panel_image = self.capture_panel().unwrap();

                    if tx.send(Some(SendItem { panel_image })).is_err() {
                        break;
                    }
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
