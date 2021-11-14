use regex::Regex;
use std::hash::{Hash, Hasher};
use edit_distance;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ItemName {
    SlimeConcentrate,
    SlimeSecretions,
    SlimeCondensate,
    //need more
}
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct GeneshinItem {
    pub name: ItemName,
    pub number: u32,
}