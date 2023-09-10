use yas::arguments_builder::arguments_builder::ArgumentsBuilder;
use clap::{Arg, Command, FromArgMatches};

use crate::export::export_format::ExportFormat;

#[derive(Default)]
pub struct GenshinArtifactScannerConfig {
    /// Items with stars less than this will be ignored
    pub min_star: u8,

    /// Items with level less than this will be ignored
    pub min_level: u8,

    /// Output directory
    pub output_dir: String,

    /// Ignore duplicated items
    pub ignore_dup: bool,

    /// Export format of item info
    pub export_format: ExportFormat,
}

impl ArgumentsBuilder for GenshinArtifactScannerConfig {
    fn modify_arguments(&self, cmd: &mut Command) {
        // todo add more configs
        cmd
            .arg(Arg::new("min-star").long("min-star").help("最小星级").value_name("MIN_STAR"))
            .arg(Arg::new("min-level").long("min-level").help("最小等级").value_name("MIN_LEVEL"));
    }
}

impl FromArgMatches for GenshinArtifactScannerConfig {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let result = GenshinArtifactScannerConfig {
            min_star: matches.get_one<u8>("min-star")?,
            min_level: matches.get_one<u8>("min-level")?,
            output_dir: String::from("."),
            ignore_dup: true,
            export_format: ExportFormat::Mona
        };

        Ok(result)
    }
}