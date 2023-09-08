pub mod good;
pub mod mingyu_lab;
pub mod mona_uranai;

use crate::*;
use std::path::Path;

use super::*;

pub fn export(results: &[GenshinArtifact]) {
    let output_dir = Path::new(&CONFIG.output_dir);

    match &CONFIG.export_format {
        ExportFormat::Mona => {
            let path = output_dir.join("mona.json");
            let value = mona_uranai::MonaFormat::new(results);
            save(&value, path);
        },
        ExportFormat::MingyuLab => {
            let path = output_dir.join("mingyulab.json");
            let value = mingyu_lab::MingyuLabFormat::new(results);
            save(&value, path);
        },
        ExportFormat::Good => {
            let path = output_dir.join("good.json");
            let value = good::GOODFormat::new(results);
            save(&value, path);
        },
        _ => {},
    };
}
