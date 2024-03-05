use clap::{Arg, arg, FromArgMatches};

// todo add all the cmd arguments
#[derive(Clone)]
pub struct StarRailRepositoryScannerLogicConfig {
    /// Max rows to scan
    pub max_row: i32,

    // todo move to another scanner
    /// Will the scanner capture only?
    // pub capture_only: bool,

    /// The time to wait for scrolling. Consider increasing this value if the scrolling is not correct
    pub scroll_delay: i32,

    /// Dump the captured image
    // pub dump_mode: bool,

    /// The maximum time to wait for switching to the next item
    pub max_wait_switch_item: i32,

    /// The time to wait for switching to the next item in cloud game
    pub cloud_wait_switch_item: i32,
}

impl Default for StarRailRepositoryScannerLogicConfig {
    fn default() -> Self {
        StarRailRepositoryScannerLogicConfig {
            max_row: -1,
            // capture_only: false,
            scroll_delay: 80,
            // number: -1,
            // dump_mode: false,
            max_wait_switch_item: 800,
            cloud_wait_switch_item: 300,
        }
    }
}

impl FromArgMatches for StarRailRepositoryScannerLogicConfig {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let mut result = StarRailRepositoryScannerLogicConfig::default();
        if matches.contains_id("max-row") {
            result.max_row = *matches.get_one::<i32>("max-row").unwrap();
        }
        if matches.contains_id("scroll-delay") {
            result.scroll_delay = *matches.get_one::<i32>("scroll-delay").unwrap();
        }
        if matches.contains_id("max-wait-switch-item") {
            result.max_wait_switch_item = *matches.get_one::<i32>("max-wait-switch-item").unwrap();
        }
        if matches.contains_id("cloud-wait-switch-item") {
            result.cloud_wait_switch_item = *matches.get_one::<i32>("cloud-wait-switch-item").unwrap();
        }

        Ok(result)
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        Ok(())
    }
}

impl ArgumentsModifier for StarRailRepositoryScannerLogicConfig {
    fn modify_arguments(builder: &mut ArgumentsBuilder) {
        builder
            .arg(
                Arg::new("max-row")
                    .long("max-row")
                    .help("最大扫描行数")
                    .value_parser(clap::value_parser!(i32).range(1..))
            )
            .arg(
                Arg::new("scroll-delay")
                    .long("scroll-delay")
                    .help("翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项）")
                    .value_parser(clap::value_parser!(i32).range(1..))
            )
            // todo remove offset x and y
            .arg(
                Arg::new("offset-x")
                    .long("offset-x")
                    .help("人为指定横坐标偏移（截图有偏移时可用该选项校正）")
                    .value_parser(clap::value_parser!(i32))
                    .hide(true)
            )
            .arg(
                Arg::new("offset-y")
                    .long("offset-y")
                    .help("人为指定纵坐标偏移（截图有偏移时可用该选项校正）")
                    .value_parser(clap::value_parser!(i32))
                    .hide(true)
            )
            .arg(
                Arg::new("max-wait-switch-item")
                    .long("max-wait-switch-item")
                    .help("切换物品最大等待时间(ms)")
                    .value_parser(clap::value_parser!(i32).range(1..))
            )
            .arg(
                Arg::new("cloud-wait-switch-item")
                    .long("cloud-wait-switch-item")
                    .help("指定云游戏切换物品等待时间(ms)")
                    .value_parser(clap::value_parser!(i32).range(1..))
            );
    }
}