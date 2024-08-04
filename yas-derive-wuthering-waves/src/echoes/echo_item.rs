use serde::{Deserialize};

#[derive(Deserialize)]
pub struct EchoDataItem {
    pub name: String,
    // pub cost: usize,
    pub name_chs: String,
}
