#[macro_use]
extern crate log;

use anyhow::Result;
use yas::core::genshin::GenshinArtifact;

const MODEL: &[u8] = include_bytes!("../models/model_training.onnx");
const CONTENT: &str = include_str!("../models/index_2_word.json");

fn main() -> Result<()> {
    yas::init_env(yas::Game::Genshin)?;

    let mut scanner = yas::get_scanner(MODEL, CONTENT)?;

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

#[cfg(test)]
mod tests {
    use yas::core::inference::CRNNModel;

    use super::*;

    #[test]
    fn test() {
        yas::init_env(yas::Game::Genshin).unwrap();

        CRNNModel::new(MODEL, CONTENT).expect("Failed to load model");
    }
}
