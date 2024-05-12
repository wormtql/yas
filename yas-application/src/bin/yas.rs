use clap::{command, Command};
use yas::utils::press_any_key_to_continue;
use yas_genshin::application::ArtifactScannerApplication;
use yas_starrail::application::RelicScannerApplication;

fn get_genshin_command() -> Command {
    let cmd = ArtifactScannerApplication::build_command();
    cmd.name("genshin")
}

fn get_starrail_command() -> Command {
    let cmd = RelicScannerApplication::build_command();
    cmd.name("starrail")
}

fn init() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
}

pub fn main() {
    init();
    let cmd = command!()
        .subcommand(get_genshin_command())
        .subcommand(get_starrail_command());
    let arg_matches = cmd.get_matches();

    let res = if let Some((subcommand_name, matches)) = arg_matches.subcommand() {
        if subcommand_name == "genshin" {
            let application = ArtifactScannerApplication::new(matches.clone());
            application.run()
        } else if subcommand_name == "starrail" {
            let application = RelicScannerApplication::new(matches.clone());
            application.run()
        } else {
            Ok(())
        }
    } else {
        Ok(())
    };

    match res {
        Ok(_) => {
            press_any_key_to_continue();
        },
        Err(e) => {
            log::error!("error: {}", e);
            press_any_key_to_continue();
        }
    }
}