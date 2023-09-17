#[macro_use]
extern crate log;

use anyhow::Result;
use clap::{Command, command, FromArgMatches};
use yas::{core::genshin::GenshinArtifact, arguments_builder::arguments_builder::ArgumentsBuilder, window_info::{require_window_info::RequireWindowInfo, window_info_builder::WindowInfoBuilder, window_info_prototypes::WindowInfoPrototypes, self}, load_window_info, game_info::game_info::Resolution};
use yas_scanner_genshin::scanner::{GenshinArtifactScanner, GenshinArtifactScannerConfig};

fn main() -> Result<()> {
    let mut cmd = Command::new("yas-genshin-artifact");

    // setup arguments
    <GenshinArtifactScanner as ArgumentsBuilder>::modify_arguments(&mut cmd);

    // setup window info
    let window_info = {
        let mut window_info_builder = WindowInfoBuilder::new();
        <GenshinArtifactScanner as RequireWindowInfo>::require_window_info(&mut window_info_builder);

        let mut window_info_prototypes = WindowInfoPrototypes::new();
        window_info_prototypes.insert(load_window_info!("../window_info/windows16x9.json"));

        let resolution = Resolution::Windows16x9;
        window_info_builder.build(&window_info_prototypes, resolution).unwrap()
    };

    // setup config
    let matches = cmd.get_matches();
    let config = GenshinArtifactScannerConfig::from_arg_matches(&matches).unwrap();

    // setup scanner
    let scanner = GenshinArtifactScanner::new(config, &window_info);

    // run
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到原神窗口，Yas 将在 5s 后开始扫描");
        yas::common::utils::sleep(5000);
    }
    let results = scanner.scan()?;



    let artifacts = yas::map_results_to::<GenshinArtifact>(&results);

    yas::export::genshin::export(&artifacts);

    info!("Yas 识别结束，共识别到 {} 件圣遗物。", artifacts.len());

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use yas::core::inference::CRNNModel;

//     use super::*;

//     #[test]
//     fn test() {
//         yas::init_env(yas::Game::Genshin).unwrap();

//         CRNNModel::new(MODEL, CONTENT).expect("Failed to load model");
//     }
// }
