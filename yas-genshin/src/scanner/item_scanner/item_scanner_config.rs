use clap::{Arg, ArgAction, FromArgMatches};
use yas::arguments_builder::arguments_builder::ArgumentsModifier;

use crate::scanner_controller::repository_layout::config::GenshinRepositoryScannerLogicConfig;

pub struct ItemScannerConfig {
    pub verbose: bool,

    pub genshin_repo_scan_logic_config: GenshinRepositoryScannerLogicConfig,
}

impl Default for ItemScannerConfig {
    fn default() -> Self {
        ItemScannerConfig {
            verbose: false,
            genshin_repo_scan_logic_config: Default::default()
        }
    }
}

impl ArgumentsModifier for ItemScannerConfig {
    fn modify_arguments(builder: &mut yas::arguments_builder::arguments_builder::ArgumentsBuilder) {
        builder
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .help("显示详细信息")
                    .num_args(0)
                    .action(ArgAction::SetTrue)
            );
        
        <GenshinRepositoryScannerLogicConfig as ArgumentsModifier>::modify_arguments(builder);
    }
}

impl FromArgMatches for ItemScannerConfig {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let scanner_controller_config = GenshinRepositoryScannerLogicConfig::from_arg_matches(matches)?;

        let result = ItemScannerConfig {
            genshin_repo_scan_logic_config: scanner_controller_config,
            verbose: matches.get_flag("verbose"),
        };

        Ok(result)
    }

    fn update_from_arg_matches(&mut self, _matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        unimplemented!()
    }
}
