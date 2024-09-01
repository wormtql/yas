use clap::ValueEnum;

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WWEchoExportFormat {
    Hsi
}

impl Default for WWEchoExportFormat {
    fn default() -> Self {
        Self::Hsi
    }
}
