use yas::utils::press_any_key_to_continue;
use yas_genshin::application::ArtifactScannerApplication;
use log::error;

pub fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let command = ArtifactScannerApplication::build_command();
    let matches = command.get_matches();

    let application = ArtifactScannerApplication::new(matches);
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