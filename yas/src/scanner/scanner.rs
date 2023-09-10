use anyhow::Result;

/// for scan jobs like artifacts/relics
pub trait StreamingScanner<OutputType> {
    fn scan_next(&mut self) -> Option<OutputType>;
}

pub trait Scanner<OutputType> {
    fn scan(&mut self) -> Result<OutputType>;
}