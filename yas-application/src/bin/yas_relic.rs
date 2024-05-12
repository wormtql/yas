use yas::utils::press_any_key_to_continue;
use yas_starrail::application::RelicScannerApplication;
use log::error;

pub fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    let matches = RelicScannerApplication::build_command().get_matches();

    let application = RelicScannerApplication::new(matches);
    match application.run() {
        Err(e) => {
            error!("error: {}", e);
            press_any_key_to_continue();
        },
        _ => {
            press_any_key_to_continue();
        }
    }
}