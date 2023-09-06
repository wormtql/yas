use std::collections::HashSet;
use std::ops::DerefMut;
use std::sync::mpsc;
use std::time::SystemTime;
use std::{fs, thread};

use crate::core::inference::*;
use crate::core::scanner::*;
use crate::core::*;
use anyhow::Result;
use enigo::{MouseButton, MouseControllable};
use image::{GenericImageView, RgbImage};

pub struct YasGenshinScanner(pub ScannerCore);

impl ItemScanner for YasGenshinScanner {
    fn scan(&mut self) -> Result<Vec<ScanResult>> {
        let count = self.get_item_count().unwrap_or(1800) as usize;

        let total_row = (count + self.col - 1) / self.col;
        let last_row_col = if count % self.col == 0 {
            self.col
        } else {
            count % self.col
        };

        info!("Detected count: {}", count);
        info!("Total row: {}", total_row);
        info!("Last column: {}", last_row_col);

        let (tx, rx) = mpsc::channel::<Option<(RgbImage, u32)>>();
        let info = self.scan_info.clone();
        let is_verbose = self.config.verbose;
        let is_dump_mode = self.config.dump_mode;
        let min_level = self.config.min_level;
        let model = self.model.clone();

        let handle = thread::spawn(move || {
            let mut results: Vec<ScanResult> = Vec::new();
            let mut dup_count = 0;
            let mut hash = HashSet::new();
            let mut consecutive_dup_count = 0;
            let panel_origin = Rect::<u32, u32>::from(info.panel_pos).origin;

            let mut cnt = 0;
            if is_dump_mode {
                fs::create_dir("dumps").unwrap();
            }

            for i in rx {
                if i.is_none() {
                    break;
                }

                let (capture, star) = i.unwrap();

                //info!("raw capture image: width = {}, height = {}", capture.width(), capture.height());
                let _now = SystemTime::now();

                let model_inference = |pos: &RectBound<u32>,
                                       name: &str,
                                       captured_img: &RgbImage,
                                       cnt: i32|
                 -> String {
                    let rect = &Rect::<u32, u32>::from(*pos) - &panel_origin;
                    let raw_img = to_gray(captured_img)
                        .view(
                            rect.origin.x,
                            rect.origin.y,
                            rect.size.width,
                            rect.size.height,
                        )
                        .to_image();

                    if is_dump_mode {
                        raw_img
                            .to_common_grayscale()
                            .save(format!("dumps/{}_{}.png", name, cnt))
                            .expect("Err");
                    }

                    let processed_img = match pre_process(raw_img) {
                        Some(im) => im,
                        None => {
                            return String::new();
                        },
                    };
                    if is_dump_mode {
                        processed_img
                            .to_common_grayscale()
                            .save(format!("dumps/p_{}_{}.png", name, cnt))
                            .expect("Err");
                    }

                    let inference_result = model.inference_string(&processed_img).unwrap();
                    if is_dump_mode {
                        fs::write(format!("dumps/{}_{}.txt", name, cnt), &inference_result)
                            .expect("Err");
                    }

                    inference_result
                };

                let str_title = model_inference(&info.title_pos, "title", &capture, cnt);
                let str_main_stat_name =
                    model_inference(&info.main_stat_name_pos, "main_stat_name", &capture, cnt);
                let str_main_stat_value =
                    model_inference(&info.main_stat_value_pos, "main_stat_value", &capture, cnt);

                let genshin_info = &info.inner_genshin();

                let str_sub_stat_1 =
                    model_inference(&genshin_info.sub_stat_pos[0], "sub_stat_1", &capture, cnt);
                let str_sub_stat_2 =
                    model_inference(&genshin_info.sub_stat_pos[1], "sub_stat_2", &capture, cnt);
                let str_sub_stat_3 =
                    model_inference(&genshin_info.sub_stat_pos[2], "sub_stat_3", &capture, cnt);
                let str_sub_stat_4 =
                    model_inference(&genshin_info.sub_stat_pos[3], "sub_stat_4", &capture, cnt);

                let str_level = model_inference(&info.level_pos, "level", &capture, cnt);
                let str_equip = model_inference(&info.item_equip_pos, "equip", &capture, cnt);

                cnt += 1;

                // let predict_time = now.elapsed().unwrap().as_millis();
                // println!("predict time: {}ms", predict_time);

                let result = ScanResult {
                    name: str_title,
                    main_stat_name: str_main_stat_name,
                    main_stat_value: str_main_stat_value,
                    sub_stat: [
                        str_sub_stat_1,
                        str_sub_stat_2,
                        str_sub_stat_3,
                        str_sub_stat_4,
                    ],
                    level: parse_level(&str_level),
                    equip: str_equip,
                    star,
                };

                if is_verbose {
                    info!("{:?}", result);
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
                    error!("检测到连续多个重复圣遗物，可能为翻页错误，或者为非背包顶部开始扫描");
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
        });

        let mut scanned_row = 0;
        let mut scanned_count = 0;
        let mut start_row = 0;
        self.move_to(0, 0);

        #[cfg(target_os = "macos")]
        utils::sleep(20);

        self.enigo.mouse_click(MouseButton::Left);

        utils::sleep(1000);
        // self.wait_until_switched();

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
                    let capture = self.capture_panel().unwrap();
                    let star = self.get_star();
                    if star < self.config.min_star {
                        break 'outer;
                    }
                    tx.send(Some((capture, star))).unwrap();

                    scanned_count += 1;
                } // end 'col

                scanned_row += 1;

                if scanned_row >= self.config.max_row {
                    info!("max row reached, quiting...");
                    break 'outer;
                }
            } // end 'row

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

        tx.send(None)?;

        info!("扫描结束，等待识别线程结束，请勿关闭程序");

        let results: Vec<ScanResult> = handle.join().unwrap();

        info!("count: {}", results.len());

        Ok(results)
    }
}


impl Deref for YasGenshinScanner {
    type Target = ScannerCore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YasGenshinScanner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
