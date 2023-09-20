use yas::arguments_builder::arguments_builder::{ArgumentsModifier, ArgumentsBuilder};
use clap::{Arg, Command, FromArgMatches};

use crate::{export::export_format::GenshinArtifactExportFormat, scanner_controller::repository_layout::config::GenshinRepositoryScannerLogicConfig};

#[derive(Default, Clone)]
pub struct GenshinArtifactScannerConfig {
    /// Items with stars less than this will be ignored
    pub min_star: i32,

    /// Items with level less than this will be ignored
    pub min_level: i32,

    /// Ignore duplicated items
    pub ignore_dup: bool,

    pub verbose: bool,

    pub number: i32,

    pub genshin_repo_scan_logic_config: GenshinRepositoryScannerLogicConfig,
}

impl ArgumentsModifier for GenshinArtifactScannerConfig {
    fn modify_arguments(builder: &mut ArgumentsBuilder) {
        // todo use custom command builder
        // todo add more configs
        builder
            .arg(Arg::new("verbose").long("verbose").help("显示详细信息"))
            .arg(Arg::new("min-star").long("min-star").help("最小星级").value_name("MIN_STAR"))
            .arg(Arg::new("min-level").long("min-level").help("最小等级").value_name("MIN_LEVEL"));

        <GenshinRepositoryScannerLogicConfig as ArgumentsModifier>::modify_arguments(builder);
    }
}

impl FromArgMatches for GenshinArtifactScannerConfig {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let scanner_controller_config = GenshinRepositoryScannerLogicConfig::from_arg_matches(matches)?;

        // todo
        let result = GenshinArtifactScannerConfig {
            min_star: *matches.get_one::<i32>("min-star").unwrap(),
            min_level: *matches.get_one::<i32>("min-level").unwrap(),
            number: *matches.get_one::<i32>("number").unwrap(),
            ignore_dup: true,
            verbose: *matches.get_one::<bool>("verbose").unwrap(),

            genshin_repo_scan_logic_config: scanner_controller_config
        };

        Ok(result)
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        unimplemented!()
    }
}