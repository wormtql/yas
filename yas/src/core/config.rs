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

    /// TODO
    #[arg(long, default_value_t = 80)]
    pub scroll_delay: u32,

    /// TODO
    #[arg(long, default_value_t = 0)]
    pub number: usize,

    /// Show verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Dump the captured image
    #[arg(id = "dump", long, default_value_t = false)]
    pub dump_mode: bool,

    /// The time to wait for switching to the next item
    #[arg(long, default_value_t = 800)]
    pub max_wait_switch_item: u32,

    /// TODO
    #[arg(long, default_value_t = 300)]
    pub cloud_wait_switch_item: u32,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    pub output_dir: String,

    /// Only draw config for captured image
    #[arg(long, default_value_t = false)]
    pub draw_config_only: bool,

    /// TODO
    #[arg(short, long, value_enum, default_value_t = ExportFormat::None)]
    pub export_format: ExportFormat,

    /// Game to scan
    #[arg(short, long, value_enum, default_value_t = Game::Genshin)]
    pub game: Game,
}

// App::new("YAS - 原神圣遗物导出器")
// // .version(utils::VERSION)
// .author("wormtql <584130248@qq.com>")
// .about("Genshin Impact Artifact Exporter")
// .arg(
//     Arg::with_name("max-row")
//         .long("max-row")
//         .takes_value(true)
//         .help("最大扫描行数"),
// )
// .arg(
//     Arg::with_name("dump")
//         .long("dump")
//         .required(false)
//         .takes_value(false)
//         .help("输出模型预测结果、二值化图像和灰度图像，debug专用"),
// )
// .arg(
//     Arg::with_name("capture-only")
//         .long("capture-only")
//         .required(false)
//         .takes_value(false)
//         .help("只保存截图，不进行扫描，debug专用"),
// )
// .arg(
//     Arg::with_name("min-star")
//         .long("min-star")
//         .takes_value(true)
//         .help("最小星级"),
// )
// .arg(
//     Arg::with_name("min-level")
//         .long("min-level")
//         .takes_value(true)
//         .help("最小等级"),
// )
// .arg(
//     Arg::with_name("output-dir")
//         .long("output-dir")
//         .short("o")
//         .takes_value(true)
//         .help("输出目录")
//         .default_value("."),
// )
// .arg(
//     Arg::with_name("scroll-stop")
//         .long("scroll-stop")
//         .takes_value(true)
//         .help("翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项，默认为80）"),
// )
// .arg(
//     Arg::with_name("verbose")
//         .long("verbose")
//         .help("显示详细信息"),
// )
// .arg(
//     Arg::with_name("offset-x")
//         .long("offset-x")
//         .takes_value(true)
//         .help("人为指定横坐标偏移（截图有偏移时可用该选项校正）"),
// )
// .arg(
//     Arg::with_name("offset-y")
//         .long("offset-y")
//         .takes_value(true)
//         .help("人为指定纵坐标偏移（截图有偏移时可用该选项校正）"),
// )
