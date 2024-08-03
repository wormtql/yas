#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct WWEchoScanResult {
    pub name: String,
    pub main_stat1_name: String,
    pub main_stat1_value: String,
    pub main_stat2_name: String,
    pub main_stat2_value: String,
    pub sub_stat_names: [String; 5],
    pub sub_stat_values: [String; 5],
    // pub equip: String,
    pub level: usize,
    pub star: usize,
}
