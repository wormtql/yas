pub use relic_scanner::{StarRailRelicScanner};
pub use relic_scanner_config::StarRailRelicScannerConfig;
pub use scan_result::StarRailRelicScanResult;
// pub use relic_scanner_window_info::RelicScannerWindowInfo;

mod relic_scanner;
mod relic_scanner_config;
mod relic_scanner_window_info;
mod scan_result;
mod relic_scanner_worker;
mod message_items;
mod match_colors;
