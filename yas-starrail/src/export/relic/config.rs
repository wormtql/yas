use crate::export::relic::export_format::StarRailRelicExportFormat;

#[derive(clap::Args)]
pub struct ExportRelicConfig {
    #[arg(id = "format", long = "format", short = 'f', default_value_t = StarRailRelicExportFormat::March7th, help = "输出格式")]
    #[arg(value_enum)]
    pub format: StarRailRelicExportFormat,

    #[arg(id = "output-dir", long = "output-dir", short, default_value_t = String::from("."), help = "输出目录")]
    pub output_dir: String,
}
