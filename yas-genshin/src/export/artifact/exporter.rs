use std::path::PathBuf;

use anyhow::Result;
use clap::FromArgMatches;

use yas::export::{AssetEmitter, ExportAssets};

use crate::artifact::GenshinArtifact;
use crate::export::artifact::{ExportArtifactConfig, GenshinArtifactExportFormat};
use crate::export::artifact::csv::GenshinArtifactCSVFormat;

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

impl<'a> AssetEmitter for GenshinArtifactExporter<'a> {
    fn emit(&self, export_assets: &mut ExportAssets) {
        if self.results.is_none() {
            return;
        }

        let results = self.results.unwrap();

        match self.format {
            GenshinArtifactExportFormat::Mona => {
                let path = self.output_dir.join("mona.json");
                let value = MonaFormat::new(results);
                let contents = serde_json::to_string(&value).unwrap();

                export_assets.add_asset(
                    Some(String::from("artifacts")),
                    path,
                    contents.into_bytes(),
                    Some(String::from("莫娜圣遗物格式")));
            },
            GenshinArtifactExportFormat::MingyuLab => {
                let path = self.output_dir.join("mingyulab.json");
                let value = MingyuLabFormat::new(results);
                let contents = serde_json::to_string(&value).unwrap();

                export_assets.add_asset(
                    Some(String::from("artifacts")),
                    path,
                    contents.into_bytes(),
                    Some(String::from("原魔计算器圣遗物格式")));
            },
            GenshinArtifactExportFormat::Good => {
                let path = self.output_dir.join("good.json");
                let value = GOODFormat::new(results);
                let contents = serde_json::to_string(&value).unwrap();

                export_assets.add_asset(
                    Some(String::from("artifacts")),
                    path,
                    contents.into_bytes(),
                    Some(String::from("GOOD圣遗物格式")));
            },
            GenshinArtifactExportFormat::CSV => {
                let path = self.output_dir.join("artifacts.csv");
                let value = GenshinArtifactCSVFormat::new(results);
                let contents = value.to_csv_string();
                export_assets.add_asset(
                    Some(String::from("artifacts csv format")),
                    path,
                    contents.into_bytes(),
                    Some(String::from("CSV格式圣遗物"))
                );
            },
            GenshinArtifactExportFormat::All => {
                // mona
                {
                    let path = self.output_dir.join("mona.json");
                    let value = MonaFormat::new(results);
                    let contents = serde_json::to_string(&value).unwrap();

                    export_assets.add_asset(
                        Some(String::from("mona")),
                        path,
                        contents.into_bytes(),
                        Some(String::from("莫娜圣遗物格式")));
                }
                // mingyulab
                {
                    let path = self.output_dir.join("mingyulab.json");
                    let value = MingyuLabFormat::new(results);
                    let contents = serde_json::to_string(&value).unwrap();

                    export_assets.add_asset(
                        Some(String::from("mingyulab")),
                        path,
                        contents.into_bytes(),
                        Some(String::from("原魔计算器圣遗物格式")));
                }
                // good
                {
                    let path = self.output_dir.join("good.json");
                    let value = GOODFormat::new(results);
                    let contents = serde_json::to_string(&value).unwrap();

                    export_assets.add_asset(
                        Some(String::from("GOOD")),
                        path,
                        contents.into_bytes(),
                        Some(String::from("GOOD圣遗物格式")));
                }
                // csv
                {
                    let path = self.output_dir.join("artifacts.csv");
                    let value = GenshinArtifactCSVFormat::new(results);
                    let contents = value.to_csv_string();
                    export_assets.add_asset(
                        Some(String::from("csv")),
                        path,
                        contents.into_bytes(),
                        Some(String::from("CSV格式圣遗物"))
                    );
                }
            }
        };
    }
}
