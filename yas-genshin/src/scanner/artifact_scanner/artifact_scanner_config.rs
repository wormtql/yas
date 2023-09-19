use yas::arguments_builder::arguments_builder::ArgumentsBuilder;
use clap::{Arg, Command, FromArgMatches};

use crate::{export::export_format::GenshinArtifactExportFormat, scanner_controller::repository_layout::config::GenshinRepositoryScannerLogicConfig};

#[derive(Default, Clone)]
pub struct GenshinArtifactScannerConfig {
    /// Items with stars less than this will be ignored
    pub min_star: i32,

    /// Items with level less than this will be ignored
    pub min_level: i32,

    /// Output directory
    pub output_dir: String,

    /// Ignore duplicated items
    pub ignore_dup: bool,

    /// Export format of item info
    pub export_format: GenshinArtifactExportFormat,

    pub number: i32,

    pub genshin_repo_scan_logic_config: GenshinRepositoryScannerLogicConfig,
}

impl ArgumentsBuilder for GenshinArtifactScannerConfig {
    fn modify_arguments(cmd: Command) -> Command {
        // todo use custom command builder
        // todo add more configs
        let cmd = cmd
            .arg(Arg::new("min-star").long("min-star").help("最小星级").value_name("MIN_STAR"))
            .arg(Arg::new("min-level").long("min-level").help("最小等级").value_name("MIN_LEVEL"));

        <GenshinRepositoryScannerLogicConfig as ArgumentsBuilder>::modify_arguments(cmd)
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
            output_dir: String::from("."),
            ignore_dup: true,
            export_format: GenshinArtifactExportFormat::Mona,

            genshin_repo_scan_logic_config: scanner_controller_config
        };

        Ok(result)
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        unimplemented!()
    }
}