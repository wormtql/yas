use std::io::Read;
use log::error;

use yas_scanner_genshin::application::ArtifactScannerApplication;

pub fn main() {
    let application = ArtifactScannerApplication::new();
    match application.run() {
        Err(e) => {
            error!("error: {}", e);
            // press any key to continue
            let _ = std::io::stdin().read(&mut [0u8]).unwrap();
        },
        _ => {}
    }
}
