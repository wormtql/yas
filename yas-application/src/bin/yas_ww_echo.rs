use yas::utils::press_any_key_to_continue;
use log::error;
use yas_wutheringwaves::application::WWEchoScannerApplication;

pub fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    let matches = WWEchoScannerApplication::build_command().get_matches();

    let application = WWEchoScannerApplication::new(matches);
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