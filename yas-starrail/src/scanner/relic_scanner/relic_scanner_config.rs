use log::info;
use yas::arguments_builder::arguments_builder::{ArgumentsModifier, ArgumentsBuilder};
use clap::{Arg, Command, FromArgMatches, ArgAction};

use crate::{export::export_format::StarRailRelicExportFormat, scanner_controller::repository_layout::config::StarRailRepositoryScannerLogicConfig};

#[derive(Default, Clone)]
pub struct StarRailRelicScannerConfig {
    /// Items with stars less than this will be ignored
    pub min_star: i32,

    /// Items with level less than this will be ignored
    pub min_level: i32,

    /// Ignore duplicated items
    pub ignore_dup: bool,

    pub verbose: bool,

    pub number: i32,

    pub starrail_repo_scan_logic_config: StarRailRepositoryScannerLogicConfig,
}

impl ArgumentsModifier for StarRailRelicScannerConfig {
    fn modify_arguments(builder: &mut ArgumentsBuilder) {
        // todo use custom command builder
        // todo add more configs
        builder
            .arg(
                Arg::new("ignore-dup")
                    .long("ignore-dup")
                    .help("忽略重复物品")
                    // .num_args(0)
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .help("显示详细信息")
                    // .num_args(0)
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("min-star")
                    .long("min-star")
                    .help("最小星级")
                    .value_name("MIN_STAR")
                    .default_value("4")
                    .value_parser(clap::value_parser!(i32))
            )
            .arg(
                Arg::new("min-level")
                    .long("min-level")
                    .help("最小等级")
                    .value_name("MIN_LEVEL")
                    .default_value("0")
                    .value_parser(clap::value_parser!(i32))
            ).arg(
                Arg::new("number")
                    .long("number")
                    .help("指定遗器数量")
                    .value_name("NUMBER")
                    .value_parser(clap::value_parser!(i32))
            );

        <StarRailRepositoryScannerLogicConfig as ArgumentsModifier>::modify_arguments(builder);
    }
}

impl FromArgMatches for StarRailRelicScannerConfig {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let scanner_controller_config = StarRailRepositoryScannerLogicConfig::from_arg_matches(matches)?;

        // todo
        let result = StarRailRelicScannerConfig {
            min_star: *matches.get_one::<i32>("min-star").unwrap(),
            min_level: *matches.get_one::<i32>("min-level").unwrap(),
            number: if matches.contains_id("number") {
                *matches.get_one::<i32>("number").unwrap()
            } else {
                -1
            },
            ignore_dup: matches.get_flag("ignore-dup"),
            verbose: matches.get_flag("verbose"),

            starrail_repo_scan_logic_config: scanner_controller_config
        };

        Ok(result)
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        unimplemented!()
    }
}