use anyhow::anyhow;
use clap::{ArgMatches, Args, command};
use log::info;
use yas::export::ExportAssets;
use yas::game_info::{GameInfo, GameInfoBuilder};
use yas::window_info::{load_window_info_repo, WindowInfoRepository};
use crate::scanner::{WWEchoScanner, WWEchoScannerConfig};
use crate::scanner_controller::WWRepositoryLayoutConfig;
use anyhow::Result;

pub struct WWEchoScannerApplication {
    arg_matches: ArgMatches
}

impl WWEchoScannerApplication {
    pub fn new(args: ArgMatches) -> Self {
        Self {
            arg_matches: args
        }
    }

    pub fn build_command() -> clap::Command {
        let mut cmd = command!();
        cmd = <WWEchoScannerConfig as Args>::augment_args_for_update(cmd);
        cmd = <WWRepositoryLayoutConfig as Args>::augment_args_for_update(cmd);
        // cmd = <ExportRelicConfig as Args>::augment_args_for_update(cmd);
        cmd
    }

    fn get_window_info_repository() -> WindowInfoRepository {
        load_window_info_repo!(
            "../../window_info/windows2560x1440.json"
        )
    }

    fn get_game_info() -> anyhow::Result<GameInfo> {
        let game_info = GameInfoBuilder::new()
            .add_local_window_name("鸣潮")
            .add_local_window_name("Wuthering Waves")
            // .add_cloud_window_name("云·星穹铁道")
            .build();
        game_info
    }
}

impl WWEchoScannerApplication {
    pub fn run(&self) -> Result<()> {
        // Self::init();
        let arg_matches = &self.arg_matches;
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

        let mut scanner = WWEchoScanner::from_arg_matches(
            &window_info_repository,
            &arg_matches,
            game_info.clone()
        )?;

        let results = scanner.scan()?;

        for item in results.iter() {
            println!("{:?}", item);
        }


        // let starrail_relics = results.iter()
        //     .map(|x| StarRailRelic::try_from(x))
        //     .filter(|x| x.is_ok())
        //     .map(|x| x.unwrap())
        //     .collect::<Vec<_>>();
        // let exporter = StarRailRelicExporter::new(&arg_matches, &starrail_relics)?;
        // let mut export_assets = ExportAssets::new();
        // exporter.emit(&mut export_assets);
        //
        // let stats = export_assets.save();
        // info!("保存结果：");
        // let table = format!("{}", stats);
        // // print multiline
        // for line in table.lines() {
        //     info!("{}", line);
        // }
        // info!("Yas 识别结束，共识别到 {} 件圣遗物。", results.len());

        Ok(())
    }
}
