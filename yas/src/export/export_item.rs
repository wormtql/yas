use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ExportItem {
    // bytes
    pub contents: Vec<u8>,
    pub filename: PathBuf,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct StatisticItem {
    pub size_in_bytes: usize,
    pub filename: PathBuf,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl StatisticItem {
    pub fn from_export_item(export_item: &ExportItem) -> Self {
        StatisticItem {
            size_in_bytes: export_item.contents.len(),
            filename: export_item.filename.clone(),
            name: export_item.name.clone(),
            description: export_item.description.clone()
        }
    }
}
