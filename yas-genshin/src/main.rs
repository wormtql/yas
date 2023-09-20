#[macro_use]
extern crate log;

use anyhow::Result;
use clap::{Command, command, FromArgMatches};
use yas::{arguments_builder::arguments_builder::{ArgumentsModifier, ArgumentsBuilder}, window_info::{require_window_info::RequireWindowInfo, window_info_builder::WindowInfoBuilder, window_info_prototypes::WindowInfoPrototypes, self}, load_window_info, game_info::{Resolution, GameInfoBuilder}, common::positioning::Pos, export::ExportAssets};
use yas_scanner_genshin::{scanner::artifact_scanner::{GenshinArtifactScanner, GenshinArtifactScannerConfig}, export::artifact::GenshinArtifactExporter, artifact::GenshinArtifact};
use yas::export::YasExporter;
use yas::window_info::window_info::WindowInfo;
use yas::utils;

fn main() -> Result<()> {
    let mut builder = ArgumentsBuilder::new();
    // setup arguments
    <GenshinArtifactScannerConfig as ArgumentsModifier>::modify_arguments(&mut builder);
    <GenshinArtifactExporter as ArgumentsModifier>::modify_arguments(&mut builder);
    let cmd = Command::new("yas-genshin-artifact")
        .version("0.1.14") // todo
        .author("wormtql <584130248@qq.com>")
        .about("Genshin Impact Artifact Scanner");
    let cmd = builder.build(cmd);
    let matches = cmd.get_matches();

    // get game info
    let game_info = GameInfoBuilder::new()
        .add_local_window_name("原神")
        .add_local_window_name("Genshin Impact")
        .add_cloud_window_name("云·原神")
        .build();
    let game_info = match game_info {
        Err(e) => {
            error!("{}", e);
            utils::quit()
        },
        Ok(v) => v
    };

    // setup window info
    let window_info = {
        let mut window_info_builder = WindowInfoBuilder::new();
        <GenshinArtifactScanner as RequireWindowInfo>::require_window_info(&mut window_info_builder);

        let mut window_info_prototypes = WindowInfoPrototypes::new();
        window_info_prototypes.insert(load_window_info!("../window_info/windows16x9.json"));

        let resolution = game_info.window.size();
        let mut wi = window_info_builder.build(&window_info_prototypes, resolution).unwrap();
        wi.add_pos("window_origin", game_info.window.origin());

        wi
    };    

    // setup config
    let config = GenshinArtifactScannerConfig::from_arg_matches(&matches).unwrap();

    // setup scanner
    let mut scanner = GenshinArtifactScanner::new(config, &window_info, game_info.clone());

    // run
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到原神窗口，Yas 将在 5s 后开始扫描");
        utils::sleep(5000);
    }
    let results = scanner.scan()?;

    // export
    let mut exporter = GenshinArtifactExporter::from_arg_matches(&matches)?;
    let genshin_artifacts = results.iter()
        .map(|x| GenshinArtifact::try_from(x))
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    exporter.results = Some(&genshin_artifacts);
    let mut export_assets = ExportAssets::new();
    exporter.export(&mut export_assets);
    let export_stats = export_assets.save();
    info!("{}", export_stats);

    info!("Yas 识别结束，共识别到 {} 件圣遗物。", results.len());

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
