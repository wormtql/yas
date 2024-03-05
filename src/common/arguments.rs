#[derive(Debug, clap::Parser)]
pub struct Arguments {
    /// 最大扫描行数
    #[arg(long)]
    #[arg(default_value_t = 1000)]
    pub max_row: u32,
    /// 输出模型预测结果、二值化图像和灰度图像，debug专用
    #[arg(long = "dump")]
    pub dump_mode: bool,
    /// 只保存截图，不进行扫描，debug专用
    #[arg(long)]
    pub capture_only: bool,
    /// 最小星级
    #[arg(long)]
    #[arg(default_value_t = 4)]
    pub min_star: u32,
    /// 最小等级
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub min_level: u32,
    /// 输出目录
    #[arg(long)]
    #[arg(short)]
    #[arg(default_value_t = String::from("."))]
    pub output_dir: String,
    /// 翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项，默认为80）
    #[arg(long)]
    #[arg(default_value_t = 80)]
    pub scroll_stop: u32,
    /// 指定圣遗物、遗器数量（在自动识别数量不准确时使用）
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub number: u32,
    /// 显示详细信息
    #[arg(long)]
    pub verbose: bool,
    /// 人为指定横坐标偏移（截图有偏移时可用该选项校正）
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub offset_x: i32,
    /// 人为指定纵坐标偏移（截图有偏移时可用该选项校正）
    #[arg(long)]
    #[arg(default_value_t = 0)]
    pub offset_y: i32,
}
