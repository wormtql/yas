use anyhow::Result;
use enigo::{MouseButton, MouseControllable};
use std::ops::DerefMut;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::{collections::HashSet, fs};
use std::path::Path;

use super::genshin::GenshinScanner;
use super::starrail::StarRailScanner;
use super::*;

pub enum Scanner {
    Genshin(super::genshin::GenshinScanner),
    StarRail(super::starrail::StarRailScanner),
}

impl Scanner {
    pub fn new(
        scan_info: ScanInfo,
        config: YasScannerConfig,
        game_info: GameInfo,
        model: &[u8],
        content: &str,
    ) -> Self {
        let core = ScannerCore::new(scan_info, config, game_info, model, content);

        match crate::TARGET_GAME.get().unwrap() {
            Game::Genshin => Scanner::Genshin(genshin::GenshinScanner(core)),
            Game::StarRail => Scanner::StarRail(starrail::StarRailScanner(core)),
            _ => crate::error_and_quit!("不支持的游戏类型"),
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

        let worker = self.worker(rx);

        self.send(&tx, count);

        tx.send(None)?;

        info!("扫描结束，等待识别线程结束，请勿关闭程序");

        match worker.join() {
            Ok(v) => Ok(v),
            Err(_) => Err(anyhow::anyhow!("识别线程出现错误")),
        }
    }

    fn send(&mut self, tx: &Sender<Option<ItemImage>>, count: usize) {
        let mut scanned_row = 0;
        let mut scanned_count = 0;
        let mut start_row = 0;

        let total_row = (count + self.col - 1) / self.col;
        let last_row_col = if count % self.col == 0 {
            self.col
        } else {
            count % self.col
        };

        info!("Detected count: {}", count);
        info!("Total row: {}", total_row);
        info!("Last column: {}", last_row_col);

        self.move_to(0, 0);

        #[cfg(target_os = "macos")]
        utils::sleep(20);

        self.enigo.mouse_click(MouseButton::Left);
        utils::sleep(1000);

        self.sample_initial_color();

        'outer: while scanned_count < count {
            '_row: for row in start_row..self.row {
                let c = if scanned_row == total_row - 1 {
                    last_row_col
                } else {
                    self.col
                };

                '_col: for col in 0..c {
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

                    let image = self.capture_panel().unwrap();
                    let star = self.get_star();

                    if star < self.config.min_star {
                        break 'outer;
                    }

                    tx.send(Some(ItemImage { image, star })).ok();

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

            match self.scroll_rows(scroll_row) {
                ScrollResult::TimeLimitExceeded => {
                    error!("翻页出现问题");
                    break 'outer;
                },
                ScrollResult::Interrupt => break 'outer,
                _ => (),
            }

            utils::sleep(100);
        }
    }

    fn worker(&self, rx: Receiver<Option<ItemImage>>) -> JoinHandle<Vec<ScanResult>> {
        let is_verbose = self.config.verbose;
        let is_dump_mode = self.config.dump_mode;
        let min_level = self.config.min_level;
        let info = self.scan_info.clone();
        let dump_mode = self.config.dump_mode;
        let model = self.model.clone();
        let panel_origin = Rect::from(&self.scan_info.panel_pos).origin;
        let scan_item_image = match self {
            Scanner::Genshin(_) => GenshinScanner::scan_item_image,
            Scanner::StarRail(_) => StarRailScanner::scan_item_image,
        };

        thread::spawn(move || {
            let mut results: Vec<ScanResult> = Vec::new();
            let mut dup_count = 0;
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;
            let model_inference = get_model_inference_func(dump_mode, model, panel_origin);

            let dump_path = Path::new("dumps");
            if is_dump_mode && !dump_path.exists() {
                fs::create_dir(dump_path).unwrap();
            }

            for (cnt, item) in rx.into_iter().enumerate() {
                let item = match item {
                    Some(v) => v,
                    None => break,
                };

                let result = match scan_item_image(&model_inference, &info, item, cnt) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("扫描错误: {}", e);
                        continue;
                    },
                };

                if is_verbose {
                    info!("{:?}", &result);
                }

                if hash.contains(&result) {
                    dup_count += 1;
                    consecutive_dup_count += 1;
                    warn!("Dup artifact detected: {:#?}", result);
                } else {
                    consecutive_dup_count = 0;
                    hash.insert(result.clone());
                    results.push(result);
                }

                if consecutive_dup_count >= info.item_row {
                    error!("检测到连续多个重复物品，可能为翻页错误，或者为非背包顶部开始扫描");
                    break;
                }
            }

            info!("Dup count: {}", dup_count);

            if min_level > 0 {
                results
                    .into_iter()
                    .filter(|result| result.level >= min_level)
                    .collect::<Vec<_>>()
            } else {
                results
            }
        })
    }
}
