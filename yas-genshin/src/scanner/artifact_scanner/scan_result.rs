#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct GenshinArtifactScanResult {
    pub name: String,
    pub main_stat_name: String,
    pub main_stat_value: String,
    pub sub_stat: [String; 4],
    pub equip: String,
    pub level: i32,
    pub star: i32,
}