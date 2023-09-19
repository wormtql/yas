use yas::arguments_builder::arguments_builder::ArgumentsModifier;
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
    fn modify_arguments(cmd: Command) -> Command {
        cmd
            .arg(Arg::new("max-row").long("max-row").help("最大扫描行数"))
            // .arg(Arg::new("min-star").long("min-star").help("最小星级"))
            // .arg(Arg::new("min-level").long("min-level").help("最小等级").value_name("MIN-LEVEL"))
            .arg(Arg::new("scroll-delay").long("scroll-delay").help("翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项，默认为80）"))
            // .arg(Arg::new("number").long("number").help("指定圣遗物数量（在自动识别数量不准确时使用）"))
            .arg(Arg::new("verbose").long("verbose").help("显示详细信息"))
            .arg(Arg::new("offset-x").long("offset-x").help("人为指定横坐标偏移（截图有偏移时可用该选项校正）"))
            .arg(Arg::new("offset-y").long("offset-y").help("人为指定纵坐标偏移（截图有偏移时可用该选项校正）"))
            .arg(Arg::new("max-wait-switch-artifact").long("max-wait-switch-artifact").help("切换圣遗物最大等待时间(ms)"))
            // .arg(Arg::new("output-dir").long("output-dir").short('o').help("输出目录").default_value("."))
            // .arg(Arg::new("output-format").long("output-format").short("f").help("输出格式").default_value("mona"))
            .arg(Arg::new("cloud-wait-switch-artifact").long("cloud-wait-switch-artifact").help("指定云·原神切换圣遗物等待时间(ms)"))
    }
}

