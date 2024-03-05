use std::path::PathBuf;

use clap::{Arg, FromArgMatches};
use yas::export::{YasExporter, ExportAssets};
use anyhow::Result;

use crate::artifact::GenshinArtifact;
use crate::export::artifact::{ExportArtifactConfig, GenshinArtifactExportFormat};

use super::good::GOODFormat;
use super::mingyu_lab::MingyuLabFormat;
use super::mona_uranai::MonaFormat;

pub struct GenshinArtifactExporter<'a> {
    pub format: GenshinArtifactExportFormat,
    pub results: Option<&'a [GenshinArtifact]>,
    pub output_dir: PathBuf,
}

impl <'a> GenshinArtifactExporter<'a> {
    pub fn new(arg_matches: &clap::ArgMatches, results: &'a [GenshinArtifact]) -> Result<Self> {
        let config = ExportArtifactConfig::from_arg_matches(arg_matches)?;
        Ok(Self {
            format: config.format,
            results: Some(results),
            output_dir: PathBuf::from(&config.output_dir)
        })
    }
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
