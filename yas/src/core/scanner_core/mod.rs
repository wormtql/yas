use super::*;
use crate::common::color::Color;
use crate::core::inference::CRNNModel;
use anyhow::Result;
use enigo::Enigo;
use image::RgbImage;
use std::ops::*;
use std::sync::Arc;

mod control;
mod scan;

pub use scan::get_model_inference_func;

pub type ModelInferenceFunc =
    Box<dyn Fn(&RectBound<u32>, &str, &RgbImage, usize) -> Result<String>>;

pub struct ScannerCore {
    pool: f64,

    initial_color: Color,

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: u32,

    is_cloud: bool,

    pub row: usize,
    pub col: usize,

    pub model: Arc<CRNNModel>,
    pub scan_info: Arc<ScanInfo>,
    pub config: YasScannerConfig,

    pub enigo: Enigo,
}

pub struct ItemImage {
    pub image: RgbImage,
    pub star: u8,
}

pub trait ItemScanner {
    fn scan_item_image(
        model_inference: &ModelInferenceFunc,
        info: &Arc<ScanInfo>,
        item: ItemImage,
        cnt: usize,
    ) -> Result<ScanResult>;
}

pub fn calc_pool(row: &Vec<u8>) -> f32 {
    let len = row.len() / 3;
    let mut pool: f32 = 0.0;

    for i in 0..len {
        pool += row[i * 3] as f32;
    }
    pool
}

impl ScannerCore {
    pub fn new(
        scan_info: ScanInfo,
        config: YasScannerConfig,
        game_info: GameInfo,
        model: &[u8],
        content: &str,
    ) -> Self {
        let model = match CRNNModel::new(model, content) {
            Ok(v) => v,
            Err(e) => crate::error_and_quit!("模型加载失败, 错误信息：{}", e),
        };

        let row = scan_info.item_row;
        let col = scan_info.item_col;

        Self {
            model: Arc::new(model),
            enigo: Enigo::new(),

            scan_info: Arc::new(scan_info),
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
