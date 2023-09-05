use std::sync::Arc;
use crate::info::*;
use crate::common::color::Color;
use crate::inference::inference::CRNNModel;

use clap::Parser;
use enigo::Enigo;

pub mod genshin;
pub mod starrail;

/// Yas Scanner Config
#[derive(Parser, Debug)]
pub struct YasScannerConfig {
    /// Max rows to scan
    #[arg(short, long, default_value_t = 1000)]
    max_row: u32,

    /// Will the scanner capture only?
    #[arg(short, long, default_value_t = false)]
    capture_only: bool,

    /// Items with stars less than this will be ignored
    #[arg(short, long, default_value_t = 4)]
    min_star: u32,

    /// Items with level less than this will be ignored
    #[arg(short, long, default_value_t = 0)]
    min_level: u32,

    /// The time to wait for switching to the next item
    #[arg(short, long, default_value_t = 800)]
    max_wait_switch_item: u32,

    /// TODO
    #[arg(short, long, default_value_t = 80)]
    scroll_stop: u32,

    /// TODO
    #[arg(short, long, default_value_t = 0)]
    number: u32,

    /// Show verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Dump the captured image
    #[arg(id = "dump", short, long, default_value_t = false)]
    dump_mode: bool,

    /// TODO
    #[arg(short, long, default_value_t = 300)]
    cloud_wait_switch_item: u32,
}

enum ScrollResult {
    TimeLimitExceeded,
    Interrupt,
    Success,
    Skip,
}


#[derive(Debug)]
pub struct ScanResult {
    name: String,
    main_stat_name: String,
    main_stat_value: String,
    sub_stat: [String; 4],
    level: String,
    equip: String,
    star: u32,
}


pub struct YasScanner {
    model: Arc<CRNNModel>,
    enigo: Enigo,

    info: ScanInfo,
    config: YasScannerConfig,

    row: usize,
    col: usize,

    pool: f64,

    initial_color: Color,

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: u32,

    is_cloud: bool,
}

impl YasScanner {
    pub fn new(info: ScanInfo, is_cloud: bool, model: &[u8], content: String) -> Self {
        let model = CRNNModel::new(model, content).expect("Err");

        YasScanner {
            enigo: Enigo::new(),
            model: Arc::new(model),
            info,
            config: YasScannerConfig::parse(),

            row: info.item_row,
            col: info.item_col,

            pool: -1.0,
            initial_color: Color::default(),
            scrolled_rows: 0,
            avg_scroll_one_row: 0.0,

            avg_switch_time: 0.0,
            scanned_count: 0,

            is_cloud,
        }
    }
}
