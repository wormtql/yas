#[macro_use]
extern crate log;

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

    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到对应窗口，Yas 将在 5s 后开始扫描");
        yas::common::utils::sleep(5000);
    }

    let results = scanner.scan()?;

    match yas::CONFIG.game {
        yas::Game::Genshin => {
            let artifacts = yas::map_results_to::<yas::core::genshin::GenshinArtifact>(&results);
            yas::export::genshin::export(&artifacts);
        },
        yas::Game::StarRail => {
            let relics = yas::map_results_to::<yas::core::starrail::StarrailRelic>(&results);
            yas::export::starrail::export(&relics);
        },
    }

    info!("Yas 识别结束，共识别到 {} 件物品。", results.len());

    Ok(())
}
