use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StarRailRelicExportFormat {
    March7th,
}

impl Default for StarRailRelicExportFormat {
    fn default() -> Self {
        Self::March7th
    }
}