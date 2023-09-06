pub mod march7th;

use crate::{core::starrail::StarrailRelic, YasScannerConfig};
use std::path::Path;

use super::ExportFormat;

pub fn export(config: &YasScannerConfig, results: &[StarrailRelic]) {
    let output_dir = Path::new(&config.output_dir);

    match config.export_format {
        ExportFormat::March7th => {
            let output_filename = output_dir.join("march7th.json");
            let mona = march7th::March7thFormat::new(results);
            mona.save(String::from(output_filename.to_str().unwrap()));
        },
        _ => {},
    }
}
