use crate::core::scanner::*;
use crate::core::*;
use anyhow::Result;

pub struct YasStarRailScanner(pub ScannerCore);

impl Deref for YasStarRailScanner {
    type Target = ScannerCore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ItemScanner for YasStarRailScanner {
    fn scan(&mut self) -> Result<Vec<ScanResult>> {
        todo!()
    }
}
