use std::fmt::Display;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use log::info;

pub struct ExportItem {
    // bytes
    pub contents: Vec<u8>,
    pub filename: PathBuf
}

pub struct ExportAssets {
    pub assets: Vec<ExportItem>
}

pub struct ExportStatistics {
    pub total: usize,
    pub saved: usize,
}

impl Display for ExportStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "export results: {}/{}", self.saved, self.total)
    }
}

impl ExportStatistics {
    pub fn new() -> Self {
        ExportStatistics {
            total: 0,
            saved: 0,
        }
    }
}

impl ExportAssets {
    pub fn new() -> Self {
        ExportAssets { assets: Vec::new() }
    }

    pub fn add_asset(&mut self, filename: PathBuf, contents: Vec<u8>) {
        self.assets.push(ExportItem {
            contents,
            filename: filename
        })
    }

    pub fn save(&self) -> ExportStatistics {
        let mut stat = ExportStatistics::new();
        stat.total = self.assets.len();
        for item in self.assets.iter() {
            let mut file = match File::create(&item.filename) {
                // todo
                Err(why) => {
                    crate::error_and_quit!("无法创建文件 {:?}: {}", &item.filename, why)
                },
                Ok(file) => {
                    file
                },
            };

            match file.write_all(&item.contents) {
                Err(why) => {
                    crate::error_and_quit!("无法写入文件 {:?}: {}", &item.filename, why)
                },
                Ok(_) => info!("结果已保存至 {:?}", &item.filename),
            }

            stat.saved += 1;
        }

        stat
    }
}

pub trait YasExporter {
    fn export(&self, export_assets: &mut ExportAssets);
}