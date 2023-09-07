use clap::{command, Parser};

use crate::{export::ExportFormat, Game};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref CONFIG: YasScannerConfig = YasScannerConfig::parse();
}

/// Yas Scanner Config
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct YasScannerConfig {
    /// Max rows to scan
    #[arg(long, default_value_t = 1000)]
    pub max_row: usize,

    /// Will the scanner capture only?
    #[arg(long, default_value_t = false)]
    pub capture_only: bool,

    /// Items with stars less than this will be ignored
    #[arg(long, default_value_t = 4)]
    pub min_star: u8,

    /// Items with level less than this will be ignored
    #[arg(long, default_value_t = 0)]
    pub min_level: u8,

    /// The time to wait for scrolling. Consider increasing this value if the scrolling is not correct
    #[arg(long, default_value_t = 80)]
    pub scroll_delay: u32,

    /// Max number of items to scan
    #[arg(long, default_value_t = 0)]
    pub number: usize,

    /// Show verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Dump the captured image
    #[arg(id = "dump", long, default_value_t = false)]
    pub dump_mode: bool,

    /// The maximum time to wait for switching to the next item
    #[arg(long, default_value_t = 800)]
    pub max_wait_switch_item: u32,

    /// The time to wait for switching to the next item in cloud game
    #[arg(long, default_value_t = 300)]
    pub cloud_wait_switch_item: u32,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    pub output_dir: String,

    /// Only draw config for captured image
    #[arg(long, default_value_t = false)]
    pub draw_config_only: bool,

    /// Export format of item info
    #[arg(short, long, value_enum, default_value_t = ExportFormat::None)]
    pub export_format: ExportFormat,

    /// Game to scan
    #[arg(short, long, value_enum, default_value_t = Game::Genshin)]
    pub game: Game,
}
