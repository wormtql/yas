#[macro_use]
extern crate log;

use anyhow::Result;
use clap::{Command, FromArgMatches};
use yas::{arguments_builder::arguments_builder::{ArgumentsModifier, ArgumentsBuilder}, window_info::{require_window_info::RequireWindowInfo, window_info_builder::WindowInfoBuilder, window_info_prototypes::WindowInfoPrototypes}, load_window_info, game_info::GameInfoBuilder, export::ExportAssets};
use yas_scanner_starrail::{scanner::relic_scanner::{StarRailRelicScanner, StarRailRelicScannerConfig}, export::relic::StarRailRelicExporter, relic::StarRailRelic};
use yas::export::YasExporter;
use yas::window_info::window_info::WindowInfo;
use yas::utils;

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut builder = ArgumentsBuilder::new();
    // setup arguments
    <StarRailRelicScannerConfig as ArgumentsModifier>::modify_arguments(&mut builder);
    <StarRailRelicExporter as ArgumentsModifier>::modify_arguments(&mut builder);
    let cmd = Command::new("yas-starrail-relic")
        .version("0.1.14") // todo
        .author("wormtql <584130248@qq.com>")
        .about("Honkai: Star Rail Relic Scanner");
    let cmd = builder.build(cmd);
    let matches = cmd.get_matches();

    // get game info
    let game_info = GameInfoBuilder::new()
        .add_local_window_name("崩坏：星穹铁道")
        .add_local_window_name("Honkai: Star Rail")
        .add_cloud_window_name("云·星穹铁道")
        .build();
    let game_info = match game_info {
        Err(e) => {
            error!("{}", e);
            utils::quit()
        },
        Ok(v) => v
    };
    info!("{:?}", game_info);

    // setup window info
    let window_info = {
        let mut window_info_builder = WindowInfoBuilder::new();
        <StarRailRelicScanner as RequireWindowInfo>::require_window_info(&mut window_info_builder);

        let mut window_info_prototypes = WindowInfoPrototypes::new();
        window_info_prototypes.insert(load_window_info!("../window_info/windows16x9.json"));

        let resolution = game_info.window.size();
        let mut wi = window_info_builder.build(&window_info_prototypes, resolution).unwrap();
        wi.add_pos("window_origin", game_info.window.origin());

        wi
    };

    // setup config
    let config = StarRailRelicScannerConfig::from_arg_matches(&matches).unwrap();

    // setup scanner
    let mut scanner = StarRailRelicScanner::new(config, &window_info, game_info.clone());

    // run
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到崩坏：星穹铁道窗口，Yas 将在 5s 后开始扫描");
        utils::sleep(5000);
    }
    let results = scanner.scan()?;

    // export
    let mut exporter = StarRailRelicExporter::from_arg_matches(&matches)?;
    let starrail_relics = results.iter()
        .map(|x| StarRailRelic::try_from(x))
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    exporter.results = Some(&starrail_relics);
    let mut export_assets = ExportAssets::new();
    exporter.export(&mut export_assets);
    let export_stats = export_assets.save();
    info!("{}", export_stats);

    info!("Yas 识别结束，共识别到 {} 件遗器。", results.len());

    Ok(())
}