use super::*;
use enigo::MouseControllable;
use image::RgbImage;
use std::time::SystemTime;

#[cfg(target_os = "macos")]
use crate::common::utils::*;

impl ScannerCore {
    pub fn align_row(&mut self) {
        for _ in 0..10 {
            let color = match self.get_flag_color() {
                Ok(color) => color,
                Err(_) => return,
            };

            if self.initial_color.distance(&color) > 10 {
                self.mouse_scroll(1, false);
                utils::sleep(self.config.scroll_delay);
            } else {
                break;
            }
        }
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        let (row, col) = (row as u32, col as u32);
        let origin = &self.scan_info.origin;

        let gap = &self.scan_info.item_gap;
        let margin = &self.scan_info.scan_margin;
        let size = &self.scan_info.item_size;

        let left = origin.x + margin.width + (gap.width + size.width) * col + size.width / 2;
        let top = origin.y + margin.height + (gap.height + size.height) * row + size.height / 4;

        self.enigo.mouse_move_to(left as i32, top as i32);

        #[cfg(target_os = "macos")]
        utils::sleep(20);
    }

    pub fn scroll_one_row(&mut self) -> ScrollResult {
        let mut state = 0;

        for count in 0..25 {
            if utils::is_rmb_down() || self.cancellation_token.cancelled() {
                return ScrollResult::Interrupt;
            }

            // FIXME: Why -5 for windows?
            // #[cfg(windows)]
            // self.enigo.mouse_scroll_y(-5);

            self.mouse_scroll(1, count < 1);

            utils::sleep(self.config.scroll_delay);

            let color = match self.get_flag_color() {
                Ok(color) => color,
                Err(_) => return ScrollResult::Failed,
            };

            if state == 0 && self.initial_color.distance(&color) > 10 {
                state = 1;
            } else if state == 1 && self.initial_color.distance(&color) <= 10 {
                self.update_avg_row(count);
                return ScrollResult::Success;
            }
        }

        ScrollResult::TimeLimitExceeded
    }

    pub fn scroll_rows(&mut self, count: i32) -> ScrollResult {
        if cfg!(not(target_os = "macos")) && self.scrolled_rows >= 5 {
            let length = self.estimate_scroll_length(count);

            debug!(
                "Alread scrolled {} rows, estimated scroll length: {}",
                self.scrolled_rows, length
            );

            self.mouse_scroll(length, false);

            utils::sleep(400);

            self.align_row();
            return ScrollResult::Skip;
        }

        for _ in 0..count {
            match self.scroll_one_row() {
                ScrollResult::Success | ScrollResult::Skip => continue,
                v => {
                    info!("Scrolling failed: {:?}", v);
                    return v;
                },
            }
        }

        ScrollResult::Success
    }

    pub fn wait_until_switched(&mut self) -> bool {
        if self.game_info.is_cloud {
            utils::sleep(self.config.cloud_wait_switch_item);
            return true;
        }

        let now = SystemTime::now();

        let mut consecutive_time = 0;
        let mut diff_flag = false;
        while now.elapsed().unwrap().as_millis() < self.config.max_wait_switch_item as u128 {
            let im: RgbImage = Rect::from(&self.scan_info.pool_pos)
                .capture_relative(&self.scan_info.origin)
                .unwrap();

            let pool = calc_pool(im.as_raw()) as f64;

            if (pool - self.pool).abs() > 0.000001 {
                self.pool = pool;
                diff_flag = true;
                consecutive_time = 0;
            } else if diff_flag {
                consecutive_time += 1;
                if consecutive_time == 1 {
                    self.avg_switch_time = (self.avg_switch_time * self.scanned_count as f64
                        + now.elapsed().unwrap().as_millis() as f64)
                        / (self.scanned_count as f64 + 1.0);
                    self.scanned_count += 1;
                    return true;
                }
            }
        }

        false
    }

    #[inline(always)]
    pub fn mouse_scroll(&mut self, length: i32, try_find: bool) {
        #[cfg(windows)]
        self.enigo.mouse_scroll_y(-length);

        #[cfg(target_os = "linux")]
        self.enigo.mouse_scroll_y(length);

        #[cfg(target_os = "macos")]
        {
            match self.game_info.ui {
                crate::common::UI::Desktop => {
                    self.enigo.mouse_scroll_y(-length);
                    utils::sleep(20);
                },
                crate::common::UI::Mobile => {
                    if try_find {
                        mac_scroll_fast(&mut self.enigo, length)
                    } else {
                        mac_scroll_slow(&mut self.enigo, length)
                    }
                },
            }
        }
    }

    #[inline(always)]
    fn update_avg_row(&mut self, count: i32) {
        let current = self.avg_scroll_one_row * self.scrolled_rows as f64 + count as f64;
        self.scrolled_rows += 1;
        self.avg_scroll_one_row = current / self.scrolled_rows as f64;

        debug!(
            "avg scroll one row: {} ({})",
            self.avg_scroll_one_row, self.scrolled_rows
        );
    }

    #[inline(always)]
    fn estimate_scroll_length(&self, count: i32) -> i32 {
        ((self.avg_scroll_one_row * count as f64 - 2.0).round() as i32).max(0)
    }
}
