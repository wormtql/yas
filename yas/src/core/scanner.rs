use super::*;
use crate::common::color::Color;
use crate::core::inference::inference::CRNNModel;
use std::sync::Arc;
use enigo::Enigo;
use anyhow::Result;

pub struct ScannerCore {
    model: Arc<CRNNModel>,
    enigo: Enigo,

    scan_info: ScanInfo,
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

pub trait ItemScanner {
    fn scan(&mut self) -> Result<Vec<ScanResult>>;
}

impl ScannerCore {
    pub fn new(
        scan_info: ScanInfo,
        config: YasScannerConfig,
        game_info: GameInfo,
        model: &[u8],
        content: String,
    ) -> Self {
        let model = CRNNModel::new(model, content).expect("Failed to load model");
        let row = scan_info.item_row;
        let col = scan_info.item_col;

        Self {
            model: Arc::new(model),
            enigo: Enigo::new(),

            scan_info,
            config,

            row,
            col,

            pool: 0.0,

            initial_color: Color::new(0, 0, 0),

            scrolled_rows: 0,
            avg_scroll_one_row: 0.0,

            avg_switch_time: 0.0,
            scanned_count: 0,

            is_cloud: game_info.is_cloud,
        }
    }
}
