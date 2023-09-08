pub mod march7th;

use crate::*;
use std::path::Path;

use super::*;

pub fn export(results: &[StarrailRelic]) {
    let output_dir = Path::new(&CONFIG.output_dir);

    match CONFIG.export_format {
        ExportFormat::March7th => {
            let path = output_dir.join("march7th.json");
            let value = march7th::March7thFormat::new(results);
            save(&value, path);
        },
        _ => {},
    }
}
