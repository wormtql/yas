use anyhow::Result;
use crate::core::*;
use crate::core::scanner::*;

pub struct YasStarRailScanner(pub ScannerCore);

impl ItemScanner for YasStarRailScanner {
    fn scan(&mut self) -> Result<Vec<ScanResult>> {
        todo!()
    }
}
