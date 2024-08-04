use std::cell::RefCell;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Coroutine;
use std::rc::Rc;
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use clap::{ArgMatches, FromArgMatches};
use image::{Rgb, RgbImage};
use log::{error, info};

use yas::capture::{Capturer, GenericCapturer};
use yas::game_info::GameInfo;
use yas::positioning::Pos;
use yas::system_control::SystemControl;
use yas::utils;
use yas::utils::color_distance;
use yas::window_info::{FromWindowInfoRepository, WindowInfoRepository};

use crate::scanner_controller::repository::{WWRepositoryLayoutConfig, WWRepositoryLayoutWindowinfo};

pub struct WWRepositoryLayoutScanController {
    /// A value computed from a region of the panel, to detect whether an item changes
    pool: u64,

    /// Stores initial gap colors for line gap detection
    initial_flag: Rgb<u8>,

    /// How many rows were scrolled
    scrolled_rows: u32,
    /// Average wheel event to scroll a row
    avg_scroll_one_row: f64,

    /// Average waiting time for an Echo to be switched and fully displayed
    avg_switch_time: f64,
    /// How many items were scanned
    scanned_count: usize,

    game_info: GameInfo,

    /// How many rows/cols a page have
    row: usize,
    col: usize,
    // item_count: usize,

    config: WWRepositoryLayoutConfig,
    window_info: WWRepositoryLayoutWindowinfo,

    /// An instance for mouse control utility
    system_control: SystemControl,
    /// An instance for capturer
    capturer: Rc<dyn Capturer<RgbImage>>,
}

impl WWRepositoryLayoutScanController {
    pub fn new(
        window_info_repo: &WindowInfoRepository,
        config: WWRepositoryLayoutConfig,
        game_info: GameInfo
    ) -> Result<Self> {
        let window_info = WWRepositoryLayoutWindowinfo::from_window_info_repository(
            game_info.window.to_rect_usize().size(),
            game_info.ui,
            game_info.platform,
            window_info_repo
        )?;

        let row_count = window_info.ww_repository_item_row;
        let col_count = window_info.ww_repository_item_col;

        let capturer = Rc::new(GenericCapturer::new()?);

        Ok(WWRepositoryLayoutScanController {
            system_control: SystemControl::new(),

            row: row_count as usize,
            col: col_count as usize,

            window_info,
            config,

            pool: 0,

            initial_flag: Rgb([0, 0, 0]),

            scrolled_rows: 0,
            avg_scroll_one_row: 0.0,

            avg_switch_time: 0.0,

            game_info,
            scanned_count: 0,

            capturer,
        })
    }

    pub fn from_arg_matches(
        window_info_repo: &WindowInfoRepository,
        arg_matches: &ArgMatches,
        game_info: GameInfo
    ) -> Result<Self> {
        Self::new(
            window_info_repo,
            WWRepositoryLayoutConfig::from_arg_matches(arg_matches)?,
            game_info
        )
    }
}

pub enum ReturnResult {
    Interrupted,
    Finished,
}

#[derive(Debug)]
enum ScrollResult {
    TimeLimitExceeded,
    Interrupt,
    Success,
    Failed,
    Skip,
}

/// Calculate the background pixel ratio
fn calc_pool(im: &RgbImage) -> u64 {
    let mut hasher = DefaultHasher::new();
    for p in im.pixels() {
        // let color_dis = color_distance(&background_pixel_color, p);
        // if color_dis < 5 {
        //     counter += 1;
        // }
        p.hash(&mut hasher);
    }

    hasher.finish()
}

impl WWRepositoryLayoutScanController {
    /// Get a generator, which controls an item switch
    pub fn get_generator(
        object: Rc<RefCell<WWRepositoryLayoutScanController>>,
        item_count: usize,
    ) -> impl Coroutine<Yield = (), Return = Result<ReturnResult>> {
        let generator = #[coroutine] move || {
            let mut scanned_row = 0;
            let mut scanned_count = 0;
            let mut start_row = 0;

            let total_row = (item_count + object.borrow().col - 1) / object.borrow().col;
            let last_row_col = if item_count % object.borrow().col == 0 {
                object.borrow().col
            } else {
                item_count % object.borrow().col
            };

            info!(
                "扫描任务共{}个物品，共计{}行，尾行{}个",
                item_count, total_row, last_row_col
            );

            // Set cursor to the first item and sleep for a few time
            object.borrow_mut().move_to(0, 0);
            object.borrow_mut().system_control.mouse_click()?;
            utils::sleep(1000);

            // Sample initial flag color, for scroll determination
            object.borrow_mut().sample_initial_color()?;

            let row = object.borrow().row.min(total_row);

            'outer: while scanned_count < item_count {
                '_row: for row in start_row..row {
                    // Determine how many items this row have
                    let row_item_count = if scanned_row == total_row - 1 {
                        last_row_col
                    } else {
                        object.borrow().col
                    };

                    '_col: for col in 0..row_item_count {
                        // Exit if right mouse button is down, or if we've scanned more than the maximum count
                        if utils::is_rmb_down() {
                            return Ok(ReturnResult::Interrupted);
                        }
                        if scanned_count > item_count {
                            return Ok(ReturnResult::Finished);
                        }

                        object.borrow_mut().move_to(row, col);
                        object.borrow_mut().system_control.mouse_click()?;

                        object.borrow_mut().wait_until_switched()?;

                        // have to make sure at this point no mut ref exists
                        yield;

                        scanned_count += 1;
                        object.borrow_mut().scanned_count = scanned_count;
                    } // end '_col

                    scanned_row += 1;

                    if let Some(max_row) = object.borrow().config.max_row {
                        if scanned_row >= max_row {
                            info!("到达最大行数，准备退出……");
                            break 'outer;
                        }
                    }
                } // end '_row

                let remain = item_count - scanned_count;
                let remain_row = (remain + object.borrow().col - 1) / object.borrow().col;
                let scroll_row = remain_row.min(object.borrow().row);
                start_row = object.borrow().row - scroll_row;

                match object.borrow_mut().scroll_rows(scroll_row as i32)? {
                    ScrollResult::TimeLimitExceeded => {
                        return Err(anyhow!("翻页超时，扫描终止……"));
                    },
                    ScrollResult::Interrupt => {
                        return Ok(ReturnResult::Interrupted);
                    },
                    _ => (),
                }

                utils::sleep(100);
            }

            Ok(ReturnResult::Finished)
        };

        generator
    }

    pub fn capture_flag(&self) -> Result<Rgb<u8>> {
        let window_origin = self.game_info.window.to_rect_f64().origin();
        let pos: Pos<i32> = Pos {
            x: (window_origin.x + self.window_info.flag_pos.x) as i32,
            y: (window_origin.y + self.window_info.flag_pos.y) as i32,
        };
        let color = self.capturer.capture_color(pos)?;

        Ok(color)
    }

    pub fn sample_initial_color(&mut self) -> Result<()> {
        self.initial_flag = self.capture_flag()?;
        Ok(())
    }

    pub fn check_flag(&self) -> Result<bool> {
        let flag = self.capture_flag()?;
        // println!("{:?}", &flag[..20]);
        // let mut same_count = 0;

        if color_distance(&self.initial_flag, &flag) < 10 {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn align_row(&mut self) -> Result<()> {
        for _ in 0..10 {
            let check_result = self.check_flag()?;
            if !check_result {
                self.mouse_scroll(1, false);
                utils::sleep(self.config.scroll_delay.try_into()?);
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Set cursor to the specified item
    pub fn move_to(&mut self, row: usize, col: usize) {
        let (row, col) = (row as u32, col as u32);
        let origin = self.game_info.window.to_rect_f64().origin();

        let gap = self.window_info.item_gap_size;
        let margin = self.window_info.scan_margin_pos;
        let size = self.window_info.item_size;

        let left = origin.x + margin.x + (gap.width + size.width) * (col as f64) + size.width / 2.0;
        let top = origin.y + margin.y + (gap.height + size.height) * (row as f64) + size.height / 2.0;

        self.system_control.mouse_move_to(left as i32, top as i32).unwrap();

        #[cfg(target_os = "macos")]
        utils::sleep(20);
    }

    pub fn scroll_one_row(&mut self) -> Result<ScrollResult> {
        let mut state = 0;
        let mut count = 0;
        let max_scroll = 25;

        while count < max_scroll {
            if utils::is_rmb_down() {
                return Ok(ScrollResult::Interrupt);
            }

            #[cfg(windows)]
            self.system_control.mouse_scroll(1, false)?;

            utils::sleep(self.config.scroll_delay.try_into().unwrap());
            count += 1;

            match (state, self.check_flag()?) {
                (0, false) => state = 1,
                (1, true) => {
                    self.update_avg_row(count);
                    return Ok(ScrollResult::Success);
                }
                _ => {}
            }
        }

        Ok(ScrollResult::TimeLimitExceeded)
    }

    fn scroll_rows(&mut self, count: i32) -> Result<ScrollResult> {
        if cfg!(not(target_os = "macos")) && self.scrolled_rows >= 5 {
            let length = self.estimate_scroll_length(count);

            for _ in 0..length {
                self.system_control.mouse_scroll(1, false)?;
            }

            utils::sleep(self.config.scroll_delay.try_into().unwrap());

            self.align_row()?;
            return Ok(ScrollResult::Skip);
        }

        for _ in 0..count {
            match self.scroll_one_row()? {
                ScrollResult::Success | ScrollResult::Skip => continue,
                ScrollResult::Interrupt => return Ok(ScrollResult::Interrupt),
                v => {
                    error!("Scrolling failed: {:?}", v);
                    return Ok(v);
                },
            }
        }

        Ok(ScrollResult::Success)
    }

    fn wait_until_switched(&mut self) -> Result<bool> {
        // if self.game_info.is_cloud {
        //     utils::sleep(self.config.cloud_wait_switch_item.try_into()?);
        //     return Ok(());
        // }

        let consecutive_threshold = 1;
        let now = SystemTime::now();

        let mut consecutive_time = 0;
        let mut diff_flag = false;
        let mut it = 0;
        while now.elapsed()?.as_millis() < self.config.max_wait_switch_item as u128 {
            let im = self.capturer.capture_relative_to(
                self.window_info.pool_rect.to_rect_i32(),
                self.game_info.window.origin()
            )?;
            let pool = calc_pool(&im);
            // im.save(format!("{}_{}.png", it, pool))?;

            if pool != self.pool {
                self.pool = pool;
                diff_flag = true;
                consecutive_time = 0;
            } else if diff_flag {
                consecutive_time += 1;
                if consecutive_time == consecutive_threshold {
                    self.avg_switch_time = (self.avg_switch_time * self.scanned_count as f64
                        + now.elapsed().unwrap().as_millis() as f64)
                        / (self.scanned_count as f64 + 1.0);
                    self.scanned_count += 1;
                    return Ok(true);
                }
            }

            it += 1;
        }

        Ok(false)
        // Err(anyhow!("Wait until switched failed"))
    }

    pub fn mouse_scroll(&mut self, length: i32, try_find: bool) {
        #[cfg(windows)]
        self.system_control.mouse_scroll(length, try_find).unwrap();

        #[cfg(target_os = "linux")]
        self.system_control.mouse_scroll(length, try_find);

        #[cfg(target_os = "macos")]
        {
            match self.game_info.ui {
                crate::common::UI::Desktop => {
                    self.system_control.mouse_scroll(length);
                    utils::sleep(20);
                },
                crate::common::UI::Mobile => {
                    if try_find {
                        self.system_control.mac_scroll_fast(length);
                    } else {
                        self.system_control.mac_scroll_slow(length);
                    }
                },
            }
        }
    }

    fn update_avg_row(&mut self, count: i32) {
        let current = self.avg_scroll_one_row * self.scrolled_rows as f64 + count as f64;
        self.scrolled_rows += 1;
        self.avg_scroll_one_row = current / self.scrolled_rows as f64;

        info!(
            "avg scroll one row: {} ({})",
            self.avg_scroll_one_row, self.scrolled_rows
        );
    }

    fn estimate_scroll_length(&self, count: i32) -> i32 {
        ((self.avg_scroll_one_row * count as f64 - 3.0).round() as i32).max(0)
    }
}