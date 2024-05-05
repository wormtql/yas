use std::path::{PathBuf};

use clap::{FromArgMatches};

use crate::relic::StarRailRelic;

use crate::export::relic::{ExportRelicConfig, StarRailRelicExportFormat};
use anyhow::Result;
use yas::export::{AssetEmitter, ExportAssets};

use super::march7th::March7thFormat;

pub struct StarRailRelicExporter<'a> {
    pub format: StarRailRelicExportFormat,
    pub results: Option<&'a [StarRailRelic]>,
    pub output_dir: PathBuf,
}

impl<'a> StarRailRelicExporter<'a> {
    pub fn new(arg_matches: &clap::ArgMatches, results: &'a [StarRailRelic]) -> Result<Self> {
        let config = ExportRelicConfig::from_arg_matches(arg_matches)?;
        Ok(Self {
            format: config.format,
            results: Some(results),
            output_dir: PathBuf::from(&config.output_dir)
        })
    }
}

impl<'a> AssetEmitter for StarRailRelicExporter<'a> {
    fn emit(&self, asset_bundle: &mut ExportAssets) {
        if self.results.is_none() {
            return;
        }

        let results = self.results.unwrap();

        match self.format {
            StarRailRelicExportFormat::March7th => {
                let path = self.output_dir.join("march7th.json");
                let format = March7thFormat::new(results);
                let contents = serde_json::to_string(&format).unwrap();

                asset_bundle.add_asset(
                    Some(String::from("relics")),
                    path,
                    contents.into_bytes(),
                    Some(String::from("三月七遗器格式"))
                );
            }
        }
    }
}
