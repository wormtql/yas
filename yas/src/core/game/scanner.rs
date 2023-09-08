use anyhow::Result;
use console::style;
use enigo::{MouseButton, MouseControllable};
use indicatif::ProgressBar;
use std::collections::HashSet;
use std::ops::DerefMut;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::common::cancel::CancellationToken;
use crate::*;

use super::genshin::GenshinScanner;
use super::starrail::StarRailScanner;
use super::*;

pub enum Scanner {
    Genshin(super::genshin::GenshinScanner),
    StarRail(super::starrail::StarRailScanner),
}

impl Scanner {
    pub fn new(scan_info: ScanInfo, game_info: GameInfo, model: &[u8], content: &str) -> Self {
        let core = ScannerCore::new(scan_info, game_info, model, content);

        match crate::TARGET_GAME.get().unwrap() {
            Game::Genshin => Scanner::Genshin(genshin::GenshinScanner(core)),
            Game::StarRail => Scanner::StarRail(starrail::StarRailScanner(core)),
        }
    }
}

impl Deref for Scanner {
    type Target = ScannerCore;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Scanner::Genshin(scanner) => &scanner.0,
            Scanner::StarRail(scanner) => &scanner.0,
        }
    }
}

impl DerefMut for Scanner {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Scanner::Genshin(scanner) => &mut scanner.0,
            Scanner::StarRail(scanner) => &mut scanner.0,
        }
    }
}

impl Scanner {
    pub fn scan(&mut self) -> Result<Vec<ScanResult>> {
        let count = self.get_item_count();

        let (tx, rx) = mpsc::channel::<Option<ItemImage>>();
        let token = self.cancellation_token.clone();

        let worker = self.worker(rx, count, token);

        self.send(&tx, count);

        tx.send(None).ok();

        info!("扫描结束，等待识别线程结束，请勿关闭程序");

        match worker.join() {
            Ok(v) => Ok(v),
            Err(_) => Err(anyhow::anyhow!("识别线程出现错误")),
        }
    }

    fn send(&mut self, tx: &Sender<Option<ItemImage>>, count: usize) {
        let progress_bar = ProgressBar::new(count as u64)
            .with_prefix("扫描物品")
            .with_style(PROGRESS_STYLE.clone());
        let progress_bar = MULTI_PROGRESS.add(progress_bar);

        let mut scanned_row = 0;
        let mut scanned_count = 0;
        let mut start_row = 0;

        let total_row = (count + self.col - 1) / self.col;
        let last_row_col = if count % self.col == 0 {
            self.col
        } else {
            count % self.col
        };

        info!(
            "开始扫描，共 {} 个物品 {} 行，尾行 {} 个",
            count, total_row, last_row_col
        );

        self.move_to(0, 0);

        #[cfg(target_os = "macos")]
        utils::sleep(20);

        self.enigo.mouse_click(MouseButton::Left);
        utils::sleep(1000);

        self.sample_initial_color();

        'outer: while scanned_count < count {
            '_row: for row in start_row..self.row {
                let row_item_count = if scanned_row == total_row - 1 {
                    last_row_col
                } else {
                    self.col
                };

                '_col: for col in 0..row_item_count {
                    // 大于最大数量 或者 取消 或者 鼠标右键按下
                    if utils::is_rmb_down()
                        || scanned_count > count
                        || self.cancellation_token.cancelled()
                    {
                        break 'outer;
                    }

                    self.move_to(row, col);
                    self.enigo.mouse_click(MouseButton::Left);

                    #[cfg(target_os = "macos")]
                    utils::sleep(20);

                    self.wait_until_switched();

                    let image = self.capture_panel().unwrap();
                    let star = self.get_star();

                    if star < self.config.min_star {
                        info!(
                            "找到满足最低星级要求 {} 的物品，准备退出……",
                            self.config.min_star
                        );
                        break 'outer;
                    }

                    progress_bar.inc(1);
                    progress_bar.set_message(format!(
                        "{} {}",
                        style(format!("{} 星物品", star)).bold().cyan(),
                        style(format!("({},{})", scanned_row + 1, col + 1,)).dim()
                    ));

                    if tx.send(Some(ItemImage { image, star })).is_err() {
                        error!("识别线程已退出，扫描终止……");
                        break 'outer;
                    }

                    scanned_count += 1;
                } // end '_col

                scanned_row += 1;

                if scanned_row >= self.config.max_row {
                    info!("到达最大行数，准备退出……");
                    break 'outer;
                }
            } // end '_row

            let remain = count - scanned_count;
            let remain_row = (remain + self.col - 1) / self.col;
            let scroll_row = remain_row.min(self.row);
            start_row = self.row - scroll_row;

            // 在翻页前检查是否取消
            if self.cancellation_token.cancelled() {
                break 'outer;
            }

            match self.scroll_rows(scroll_row as i32) {
                ScrollResult::TimeLimitExceeded => {
                    error!("翻页超时，扫描终止……");
                    break 'outer;
                },
                ScrollResult::Interrupt => break 'outer,
                _ => (),
            }

            utils::sleep(100);
        }

        progress_bar.finish_with_message(format!("扫描结束，共扫描 {} 个物品", scanned_count));
        MULTI_PROGRESS.remove(&progress_bar);
    }

    fn worker(
        &self,
        rx: Receiver<Option<ItemImage>>,
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
