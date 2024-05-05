#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct StarRailRelicScanResult {
    pub name: String,
    pub main_stat_name: String,
    pub main_stat_value: String,
    pub sub_stat_name: [String; 4],
    pub sub_stat_value: [String; 4],
    pub equip: String,
    pub level: i32,
    pub star: i32,
    pub lock: bool,
    pub discard: bool,
}
