#[derive(Clone, clap::Args)]
pub struct GenshinArtifactScannerConfig {
    /// Items with stars less than this will be ignored
    #[arg(id = "min-star", long = "min-star", help = "最小星级", value_name = "MIN_STAR", default_value_t = 4)]
    pub min_star: i32,

    /// Items with level less than this will be ignored
    #[arg(id = "min-level", long = "min-level", help = "最小等级", value_name = "MIN_LEVEL", default_value_t = 0)]
    pub min_level: i32,

    /// Ignore duplicated items
    #[arg(id = "ignore-dup", long = "ignore-dup", help = "忽略重复物品")]
    pub ignore_dup: bool,

    /// it will output very verbose messages
    #[arg(id = "verbose", long, help = "显示详细信息")]
    pub verbose: bool,

    /// the exact amount to scan
    #[arg(id = "number", long, help = "指定圣遗物数量", value_name = "NUMBER", default_value_t = -1)]
    pub number: i32,
}
