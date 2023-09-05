use clap::Parser;

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
