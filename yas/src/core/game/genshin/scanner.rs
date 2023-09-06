use anyhow::Result;
use crate::core::*;
use crate::core::scanner::*;

pub struct YasGenshinScanner(pub ScannerCore);

impl ItemScanner for YasGenshinScanner {
    fn scan(&mut self) -> Result<Vec<ScanResult>> {
        todo!()
    }
}
