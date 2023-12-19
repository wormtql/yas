use std::path::PathBuf;

use clap::{Arg, FromArgMatches};
use yas::arguments_builder::arguments_builder::{ArgumentsModifier, ArgumentsBuilder};
use yas::export::{YasExporter, ExportAssets};

use crate::artifact::GenshinArtifact;

use crate::export::export_format::GenshinArtifactExportFormat;

use super::good::GOODFormat;
use super::mingyu_lab::MingyuLabFormat;
use super::mona_uranai::MonaFormat;

pub struct GenshinArtifactExporter<'a> {
    pub format: GenshinArtifactExportFormat,
    pub results: Option<&'a [GenshinArtifact]>,
    pub output_dir: PathBuf,
}

impl<'a> YasExporter for GenshinArtifactExporter<'a> {
    fn export(&self, export_assets: &mut ExportAssets) {
        if self.results.is_none() {
            return;
        }

        let results = self.results.unwrap();

        match self.format {
            GenshinArtifactExportFormat::Mona => {
                let path = self.output_dir.join("mona.json");
                let value = MonaFormat::new(results);
                let contents = serde_json::to_string(&value).unwrap();
                
                export_assets.add_asset(path, contents.into_bytes());
            },
            GenshinArtifactExportFormat::MingyuLab => {
                let path = self.output_dir.join("mingyulab.json");
                let value = MingyuLabFormat::new(results);
                let contents = serde_json::to_string(&value).unwrap();

                export_assets.add_asset(path, contents.into_bytes());
            },
            GenshinArtifactExportFormat::Good => {
                let path = self.output_dir.join("good.json");
                let value = GOODFormat::new(results);
                let contents = serde_json::to_string(&value).unwrap();
                
                export_assets.add_asset(path, contents.into_bytes());
            },
        };
    }
}

impl<'a> ArgumentsModifier for GenshinArtifactExporter<'a> {
    fn modify_arguments(builder: &mut ArgumentsBuilder) {
        builder.arg(
            Arg::new("output-format")
                .long("output-format")
                .short('f')
                .help("输出格式")
                .value_parser(clap::builder::EnumValueParser::<GenshinArtifactExportFormat>::new())
                .default_value("mona")
                // .default_value("Mona")
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

impl<'a> FromArgMatches for GenshinArtifactExporter<'a> {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let output_dir = matches.get_one::<String>("output-dir").unwrap();

        // todo error propogation
        let path = PathBuf::try_from(output_dir).unwrap();

        let value = GenshinArtifactExporter {
            format: *matches.get_one("output-format").unwrap(),
            results: None,
            output_dir: path
        };

        Ok(value)
    }

    fn update_from_arg_matches(&mut self, _matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        // todo
        unimplemented!()
    }
}
