#[macro_use]
extern crate log;

use std::time::SystemTime;

use anyhow::Result;
use yas::core::genshin::GenshinArtifact;

const MODEL: &[u8] = include_bytes!("../models/model_training.onnx");
const CONTENT: &str = include_str!("../models/index_2_word.json");

fn main() -> Result<()> {
    yas::init_env(yas::Game::Genshin);

    let mut scanner = yas::get_scanner(MODEL, CONTENT);

    let now = SystemTime::now();
    #[cfg(target_os = "macos")]
    {
        info!("初始化完成，请切换到原神窗口，Yas 将在 5s 后开始扫描");
        yas::common::utils::sleep(5000);
    }

    let results = scanner.scan()?;
    info!("Time: {:?}", now.elapsed());

    let artifacts = yas::map_results_to::<GenshinArtifact>(&results);

    yas::export::genshin::export(&artifacts);

    println!("{:#?}", artifacts);

    info!("Yas 识别结束");

    Ok(())
}

#[cfg(test)]
mod tests {
    use yas::core::inference::CRNNModel;

    use super::*;

    #[test]
    fn test() {
        yas::init_env(yas::Game::Genshin);

        CRNNModel::new(MODEL, CONTENT).expect("Failed to load model");
    }
}
