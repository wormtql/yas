use std::path::{Path, PathBuf};

use clap::{Arg, FromArgMatches};
use yas::arguments_builder::arguments_builder::{ArgumentsModifier, ArgumentsBuilder};
use yas::export::{YasExporter, ExportAssets};

use crate::relic::StarRailRelic;

use crate::export::export_format::StarRailRelicExportFormat;

use super::march7th::March7thFormat;

pub struct StarRailRelicExporter<'a> {
    pub format: StarRailRelicExportFormat,
    pub results: Option<&'a [StarRailRelic]>,
    pub output_dir: PathBuf,
}

impl<'a> YasExporter for StarRailRelicExporter<'a> {
    fn export(&self, export_assets: &mut ExportAssets) {
        if self.results.is_none() {
            return;
        }

        let results = self.results.unwrap();

        match self.format {
            StarRailRelicExportFormat::March7th => {
                let path = self.output_dir.join("march7th.json");
                let value = March7thFormat::new(results);
                let contents = serde_json::to_string(&value).unwrap();

                export_assets.add_asset(path, contents.into_bytes());
            },
        };
    }
}

impl<'a> ArgumentsModifier for StarRailRelicExporter<'a> {
    fn modify_arguments(builder: &mut ArgumentsBuilder) {
        builder.arg(
            Arg::new("output-format")
                .long("output-format")
                .short('f')
                .help("输出格式")
                .value_parser(clap::builder::EnumValueParser::<StarRailRelicExportFormat>::new())
                .default_value("march7th")
        )
        .arg(
            Arg::new("output-dir")
                .long("output-dir")
                .short('o')
                .help("输出目录")
                .default_value(".")
        );
    }
}

impl<'a> FromArgMatches for StarRailRelicExporter<'a> {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let output_dir = matches.get_one::<String>("output-dir").unwrap();

        // todo error propogation
        let path = PathBuf::try_from(output_dir).unwrap();

        let value = StarRailRelicExporter {
            format: *matches.get_one("output-format").unwrap(),
            results: None,
            output_dir: path
        };

        Ok(value)
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        unimplemented!()
    }
}