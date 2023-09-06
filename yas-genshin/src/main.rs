use anyhow::Result;

fn main() -> Result<()> {
    yas::init_env(yas::Game::Genshin);

    let model = include_bytes!("../models/model_training.onnx");
    let content = String::from(include_str!("../models/index_2_word.json"));
    let mut scanner = yas::get_scanner(model, content);

    let results = scanner.scan()?;

    Ok(())
}
