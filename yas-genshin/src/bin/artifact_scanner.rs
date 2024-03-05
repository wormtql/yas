use log::error;
use yas_scanner_genshin::application::ArtifactScannerApplication;

pub fn main() {
    let application = ArtifactScannerApplication::new();
    match application.run() {
        Err(e) => error!("error occrued: {}", e),
        _ => {}
    }
}
