#[macro_use]
extern crate log;

use std::time::SystemTime;

use anyhow::Result;
use yas::core::starrail::StarrailRelic;

const MODEL: &[u8] = include_bytes!("../models/model_training.onnx");
const CONTENT: &str = include_str!("../models/index_2_word.json");

fn main() -> Result<()> {
    yas::init_env(yas::Game::StarRail)?;

    let mut scanner = yas::get_scanner(MODEL, CONTENT)?;

    let now = SystemTime::now();
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到崩坏：星穹铁道窗口，Yas 将在 5s 后开始扫描");
        yas::common::utils::sleep(5000);
    }

    let results = scanner.scan()?;
    info!("扫描耗时: {:?}", now.elapsed());

    let relics = yas::map_results_to::<StarrailRelic>(&results);

    yas::export::starrail::export(&relics);

    info!("Yas 识别结束，共识别到 {} 件遗器。", relics.len());

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
