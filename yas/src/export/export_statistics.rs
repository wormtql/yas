use std::fmt;
use prettytable::{row, Row, Table};
use crate::export::{StatisticItem};
use bytesize::ByteSize;

pub struct ExportStatistics {
    pub exported_assets: Vec<StatisticItem>,
    pub failed_items: Vec<StatisticItem>,
}

impl ExportStatistics {
    pub fn get_table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(row!["Name", "Description", "File", "Size"]);
        for item in self.exported_assets.iter() {
            table.add_row(Row::new(vec![
                prettytable::Cell::new(item.name.as_ref().unwrap_or(&Default::default())),
                prettytable::Cell::new(item.description.as_ref().unwrap_or(&Default::default())),
                prettytable::Cell::new(&format!("{:?}", item.filename)),
                prettytable::Cell::new(&format!("{}", ByteSize(item.size_in_bytes as u64))),
            ]));
        }

        table
    }
}

impl fmt::Display for ExportStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = self.get_table();
        write!(f, "{}", table)
    }
}

impl ExportStatistics {
    pub fn new() -> Self {
        ExportStatistics {
            exported_assets: Vec::new(),
            failed_items: Vec::new()
        }
    }
}