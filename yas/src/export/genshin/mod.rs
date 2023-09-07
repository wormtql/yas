pub mod good;
pub mod mingyu_lab;
pub mod mona_uranai;

use crate::*;
use std::path::Path;

use super::ExportFormat;

pub fn export(results: &[GenshinArtifact]) {
    let output_dir = Path::new(&CONFIG.output_dir);

    match &CONFIG.export_format {
        ExportFormat::Mona => {
            let output_filename = output_dir.join("mona.json");
            let mona = mona_uranai::MonaFormat::new(results);
            mona.save(String::from(output_filename.to_str().unwrap()));
        },
        ExportFormat::MingyuLab => {
            let output_filename = output_dir.join("mingyulab.json");
            let mingyulab = mingyu_lab::MingyuLabFormat::new(results);
            mingyulab.save(String::from(output_filename.to_str().unwrap()));
        },
        ExportFormat::Good => {
            let output_filename = output_dir.join("good.json");
            let good = good::GOODFormat::new(results);
            good.save(String::from(output_filename.to_str().unwrap()));
        },
        _ => {},
    }
}
