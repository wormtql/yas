use anyhow::Result;
use crate::core::*;
use crate::core::scanner::*;

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
