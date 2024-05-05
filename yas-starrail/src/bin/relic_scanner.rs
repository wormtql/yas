use log::error;
use yas::utils::press_any_key_to_continue;
use yas_scanner_starrail::application::RelicScannerApplication;

pub fn main() {
    let application = RelicScannerApplication::new();
    match application.run() {
        Err(e) => {
            error!("error: {}", e);
            press_any_key_to_continue();
        },
        _ => {}
    }
}
