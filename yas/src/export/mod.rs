use clap::ValueEnum;

pub mod genshin;
pub mod starrail;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ExportFormat {
    None,
    Mona,
    MingyuLab,
    Good,
    March7th,
}
