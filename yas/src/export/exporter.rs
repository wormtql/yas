use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use log::error;

use crate::export::{ExportItem, ExportStatistics, StatisticItem};

pub struct ExportAssets {
    pub assets: Vec<ExportItem>
}

impl ExportAssets {
    pub fn new() -> Self {
        ExportAssets { assets: Vec::new() }
    }

    pub fn add_asset(&mut self, name: Option<String>, filename: PathBuf, contents: Vec<u8>, description: Option<String>) {
        self.assets.push(ExportItem {
            contents,
            filename,
            name,
            description,
        })
    }

    pub fn save(&self) -> ExportStatistics {
        let mut stat = ExportStatistics::new();

        for item in self.assets.iter() {
            let mut file = match File::create(&item.filename) {
                Err(why) => {
                    stat.failed_items.push(StatisticItem::from_export_item(item));
                    error!("无法创建文件 {:?}: {}", &item.filename, why);
                    continue;
                },
                Ok(file) => {
                    file
                },
            };

            match file.write_all(&item.contents) {
                Err(why) => {
                    stat.failed_items.push(StatisticItem::from_export_item(item));
                    error!("无法写入文件 {:?}: {}", &item.filename, why);
                    continue;
                },
                Ok(_) => (),
            }

            stat.exported_assets.push(StatisticItem::from_export_item(item));
        }

        stat
    }
}
