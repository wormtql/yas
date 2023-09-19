// pub mod starrail;
pub mod exporter;

// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// pub enum ExportFormat {
//     None,
//     Mona,
//     MingyuLab,
//     Good,
//     March7th,
// }

// fn save<V, P>(value: &V, path: P)
// where
//     V: Serialize + ?Sized,
//     P: AsRef<Path> + Debug,
// {
//     let mut file = match File::create(&path) {
//         Err(why) => crate::error_and_quit!("无法创建文件 {:?}: {}", &path, why),
//         Ok(file) => file,
//     };
//     let s = serde_json::to_string(value).unwrap();

//     match file.write_all(s.as_bytes()) {
//         Err(why) => crate::error_and_quit!("无法写入文件 {:?}: {}", &path, why),
//         Ok(_) => info!("结果已保存至 {:?}", &path),
//     }
// }
