use yas::arguments_builder::arguments_builder::{ArgumentsModifier, ArgumentsBuilder};
use clap::{Arg, arg, Command, FromArgMatches};

// todo add all the cmd arguments
#[derive(Clone)]
pub struct GenshinRepositoryScannerLogicConfig {
    /// Max rows to scan
    pub max_row: i32,

    /// Will the scanner capture only?
    pub capture_only: bool,

    /// The time to wait for scrolling. Consider increasing this value if the scrolling is not correct
    pub scroll_delay: u32,

    /// Max number of items to scan
    pub number: i32,

    /// Dump the captured image
    pub dump_mode: bool,

    /// The maximum time to wait for switching to the next item
    pub max_wait_switch_item: u32,

    /// The time to wait for switching to the next item in cloud game
    pub cloud_wait_switch_item: u32,
}

impl Default for GenshinRepositoryScannerLogicConfig {
    fn default() -> Self {
        GenshinRepositoryScannerLogicConfig {
            max_row: -1,
            capture_only: false,
            scroll_delay: 80,
            number: -1,
            dump_mode: false,
            max_wait_switch_item: 80,
            cloud_wait_switch_item: 100,
        }
    }
}

impl FromArgMatches for GenshinRepositoryScannerLogicConfig {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        // todo
        // let result = GenshinRepositoryScannerLogicConfig {
        //     max_row: matches.get_one<usize>("max-row"),
        //     capture_only: false,

        // }
            
        let result = GenshinRepositoryScannerLogicConfig::default();
        Ok(result)
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        Ok(())
    }
}

impl ArgumentsModifier for GenshinRepositoryScannerLogicConfig {
    fn modify_arguments(builder: &mut ArgumentsBuilder) {
        builder
            .arg(Arg::new("max-row").long("max-row").help("最大扫描行数"))
            .arg(Arg::new("scroll-delay").long("scroll-delay").help("翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项，默认为80）"))
            .arg(Arg::new("offset-x").long("offset-x").help("人为指定横坐标偏移（截图有偏移时可用该选项校正）"))
            .arg(Arg::new("offset-y").long("offset-y").help("人为指定纵坐标偏移（截图有偏移时可用该选项校正）"))
            .arg(Arg::new("max-wait-switch-artifact").long("max-wait-switch-artifact").help("切换物品最大等待时间(ms)"))
            .arg(Arg::new("cloud-wait-switch-artifact").long("cloud-wait-switch-artifact").help("指定云游戏切换物品等待时间(ms)"));
    }
}

