use crate::export::echo::export_format::WWEchoExportFormat;

#[derive(clap::Args)]
pub struct WWExportEchoConfig {
    #[arg(id = "format", long = "format", short = 'f', default_value_t = WWEchoExportFormat::Hsi, help = "输出格式")]
    #[arg(value_enum)]
    pub format: WWEchoExportFormat,

    #[arg(id = "output-dir", long = "output-dir", short, default_value_t = String::from("."), help = "输出目录")]
    pub output_dir: String,
}
