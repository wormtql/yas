use crate::export::artifact::GenshinArtifactExportFormat;

#[derive(clap::Args)]
pub struct ExportArtifactConfig {
    #[arg(id = "format", long = "format", short = 'f', default_value_t = GenshinArtifactExportFormat::Mona, help = "输出格式")]
    #[arg(value_enum)]
    pub format: GenshinArtifactExportFormat,

    #[arg(id = "output-dir", long = "output-dir", short, default_value_t = String::from("."), help = "输出目录")]
    pub output_dir: String,
}
