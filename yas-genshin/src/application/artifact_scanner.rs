use crate::scanner::{GenshinArtifactScanner, GenshinArtifactScannerConfig};
use clap::command;
use clap::Args;
use yas::game_info::{GameInfo, GameInfoBuilder};
use yas::window_info::{load_window_info_repo, WindowInfoRepository};
use crate::export::artifact::{ExportArtifactConfig, GenshinArtifactExporter};
use crate::scanner_controller::repository_layout::GenshinRepositoryScannerLogicConfig;
use anyhow::Result;
use log::info;
use yas::export::{ExportAssets, YasExporter};
use crate::artifact::GenshinArtifact;

pub struct ArtifactScannerApplication;

impl ArtifactScannerApplication {
    pub fn new() -> Self {
        ArtifactScannerApplication
    }

    fn build_command() -> clap::Command {
        let mut cmd = command!();
        cmd = <ExportArtifactConfig as Args>::augment_args_for_update(cmd);
        cmd = <GenshinArtifactScannerConfig as Args>::augment_args_for_update(cmd);
        cmd = <GenshinRepositoryScannerLogicConfig as Args>::augment_args_for_update(cmd);
        cmd
    }

    fn get_window_info_repository() -> WindowInfoRepository {
        load_window_info_repo!("../../window_info/windows16x9.yaml")
    }

    fn init() {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    fn get_game_info() -> Result<GameInfo> {
        let game_info = GameInfoBuilder::new()
            .add_local_window_name("原神")
            .add_local_window_name("Genshin Impact")
            .add_cloud_window_name("云·原神")
            .build();
        game_info
    }
}

impl ArtifactScannerApplication {
    pub fn run(&self) -> Result<()> {
        Self::init();
        let arg_matches = Self::build_command().get_matches();
        let window_info_repository = Self::get_window_info_repository();
        let game_info = Self::get_game_info()?;

        let mut scanner = GenshinArtifactScanner::from_arg_matches(
            &window_info_repository,
            &arg_matches,
            game_info.clone()
        )?;

        let result = scanner.scan()?;
        let artifacts = result
            .iter()
            .map(|x| GenshinArtifact::try_from(x))
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let exporter = GenshinArtifactExporter::new(&arg_matches, &artifacts)?;

        let mut export_assets = ExportAssets::new();
        exporter.export(&mut export_assets);

        let stats = export_assets.save();
        info!("{}", stats);
        info!("Yas 识别结束，共识别到 {} 件圣遗物。", result.len());

        Ok(())
    }
}
