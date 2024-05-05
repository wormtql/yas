use clap::{arg};

// todo add all the cmd arguments
#[derive(Clone, clap::Args)]
pub struct StarRailRepositoryScannerLogicConfig {
    /// Max rows to scan
    #[arg(id = "max-row", long = "max-row", help = "最大扫描行数", default_value_t = -1)]
    pub max_row: i32,

    /// The time to wait for scrolling. Consider increasing this value if the scrolling is not correct
    #[arg(id = "scroll-delay", long = "scroll-delay", help = "翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项）", default_value_t = 80)]
    pub scroll_delay: i32,

    /// Dump the captured image
    // pub dump_mode: bool,

    /// The maximum time to wait for switching to the next item
    #[arg(id = "max-wait-switch-item", long = "max-wait-switch-item", help = "切换物品最大等待时间（ms）", default_value_t = 800)]
    pub max_wait_switch_item: i32,

    /// The time to wait for switching to the next item in cloud game
    #[arg(id = "cloud-wait-switch-item", long = "cloud-wait-switch-item", help = "云游戏切换物品等待时间（ms）", default_value_t = 300)]
    pub cloud_wait_switch_item: i32,
}

impl Default for StarRailRepositoryScannerLogicConfig {
    fn default() -> Self {
        StarRailRepositoryScannerLogicConfig {
            max_row: -1,
            scroll_delay: 80,
            max_wait_switch_item: 800,
            cloud_wait_switch_item: 300,
        }
    }
}
