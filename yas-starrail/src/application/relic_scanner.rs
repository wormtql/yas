use clap::{Args, command};
use yas::game_info::{GameInfo, GameInfoBuilder};
use yas::window_info::{load_window_info_repo, WindowInfoRepository};
use crate::export::{ExportRelicConfig, StarRailRelicExporter};
use crate::scanner::relic_scanner::{StarRailRelicScanner, StarRailRelicScannerConfig};
use crate::scanner_controller::repository_layout::StarRailRepositoryScannerLogicConfig;
use anyhow::{anyhow, Result};
use log::info;
use yas::export::{AssetEmitter, ExportAssets};
use crate::relic::StarRailRelic;

pub struct RelicScannerApplication;

impl RelicScannerApplication {
    pub fn new() -> Self {
        RelicScannerApplication
    }

    fn build_command() -> clap::Command {
        let mut cmd = command!();
        cmd = <StarRailRelicScannerConfig as Args>::augment_args_for_update(cmd);
        cmd = <StarRailRepositoryScannerLogicConfig as Args>::augment_args_for_update(cmd);
        cmd = <ExportRelicConfig as Args>::augment_args_for_update(cmd);
        cmd
    }

    fn get_window_info_repository() -> WindowInfoRepository {
        load_window_info_repo!(
            "../../window_info/windows1920x1080.json"
        )
    }

    fn init() {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    fn get_game_info() -> Result<GameInfo> {
        let game_info = GameInfoBuilder::new()
            .add_local_window_name("崩坏：星穹铁道")
            .add_local_window_name("Honkai: Star Rail")
            .add_cloud_window_name("云·星穹铁道")
            .build();
        game_info
    }
}

impl RelicScannerApplication {
    pub fn run(&self) -> Result<()> {
        Self::init();
        let arg_matches = Self::build_command().get_matches();
        let window_info_repository = Self::get_window_info_repository();
        let game_info = Self::get_game_info()?;

        info!("window: {:?}", game_info.window);
        info!("ui: {:?}", game_info.ui);
        info!("cloud: {}", game_info.is_cloud);
        info!("resolution family: {:?}", game_info.resolution_family);

        #[cfg(target_os = "windows")]
        {
            // assure admin
            if !yas::utils::is_admin() {
                return Err(anyhow!("请使用管理员运行"));
            }
        }

        let mut scanner = StarRailRelicScanner::from_arg_matches(
            &window_info_repository,
            &arg_matches,
            game_info.clone()
        )?;

        let results = scanner.scan()?;
        let starrail_relics = results.iter()
            .map(|x| StarRailRelic::try_from(x))
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        let exporter = StarRailRelicExporter::new(&arg_matches, &starrail_relics)?;
        let mut export_assets = ExportAssets::new();
        exporter.emit(&mut export_assets);

        let stats = export_assets.save();
        info!("保存结果：");
        let table = format!("{}", stats);
        // print multiline
        for line in table.lines() {
            info!("{}", line);
        }
        info!("Yas 识别结束，共识别到 {} 件圣遗物。", results.len());

        Ok(())
    }
}
