#[macro_use]
extern crate log;

use std::time::SystemTime;

use anyhow::Result;
use yas_scanner as yas;

const GENSHIN_MODEL: &[u8] = include_bytes!("../../yas-genshin/models/model_training.onnx");
const GENSHIN_CONTENT: &str = include_str!("../../yas-genshin/models/index_2_word.json");

const STARRAIL_MODEL: &[u8] = include_bytes!("../../yas-starrail/models/model_training.onnx");
const STARRAIL_CONTENT: &str = include_str!("../../yas-starrail/models/index_2_word.json");

fn main() -> Result<()> {
    yas::init_env(yas::CONFIG.game)?;

    let (model, content) = match yas::CONFIG.game {
        yas::Game::Genshin => (GENSHIN_MODEL, GENSHIN_CONTENT),
        yas::Game::StarRail => (STARRAIL_MODEL, STARRAIL_CONTENT),
    };

    let mut scanner = yas::get_scanner(model, content)?;

    let now = SystemTime::now();
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到对应窗口，Yas 将在 5s 后开始扫描");
        yas::common::utils::sleep(5000);
    }

    let results = scanner.scan()?;
    info!("Time: {:?}", now.elapsed());

    match yas::CONFIG.game {
        yas::Game::Genshin => {
            let artifacts = yas::map_results_to::<yas::core::genshin::GenshinArtifact>(&results);
            yas::export::genshin::export(&artifacts);
            println!("{:#?}", artifacts);
        },
        yas::Game::StarRail => {
            let relics = yas::map_results_to::<yas::core::starrail::StarrailRelic>(&results);
            yas::export::starrail::export(&relics);
            println!("{:#?}", relics);
        },
    }

    info!("Yas 识别结束");

    Ok(())
}
