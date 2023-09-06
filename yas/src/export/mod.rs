use clap::ValueEnum;

pub mod good;
pub mod march7th;
pub mod mingyu_lab;
pub mod mona_uranai;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ExportFormat {
    None,
    Mona,
    MingyuLab,
    Good,
    March7th,
}
