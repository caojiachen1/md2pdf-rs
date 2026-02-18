/// cli.rs — Command-line argument definitions.  Mirrors cli.js.

use clap::Parser as ClapParser;
use std::path::PathBuf;

#[derive(ClapParser, Debug)]
#[command(
    name = "md2pdf",
    about = "将Markdown文件(含LaTeX公式)转换为PDF",
    version = "1.0.0"
)]
pub struct Args {
    /// Markdown 输入文件路径
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// PDF/HTML 输出文件路径 (可选，默认同目录同名)
    #[arg(value_name = "OUTPUT")]
    pub output: Option<PathBuf>,

    /// 显示详细信息
    #[arg(short, long)]
    pub verbose: bool,

    /// 输出格式 (pdf|html)
    #[arg(short, long, default_value = "pdf")]
    pub format: String,

    /// 页边距, 例如 20mm (默认: 0mm)
    #[arg(long, default_value = "0mm")]
    pub margin: String,

    /// 横向页面
    #[arg(long)]
    pub landscape: bool,

    /// 字体大小 (small|medium|large|xlarge 或具体数值如 14px)
    #[arg(long, default_value = "medium")]
    pub font_size: String,

    /// 中文字体 (simsun|simhei|simkai|fangsong|yahei|auto)
    #[arg(long, default_value = "simsun")]
    pub chinese_font: String,

    /// 文字厚度 (light|normal|medium|semibold|bold|black 或数值如 400)
    #[arg(long, default_value = "medium")]
    pub font_weight: String,

    /// 行间距 (tight|normal|loose|relaxed 或数值如 1.6)
    #[arg(long, default_value = "normal")]
    pub line_spacing: String,

    /// 段落间距 (tight|normal|loose|relaxed 或数值如 1em)
    #[arg(long, default_value = "tight")]
    pub paragraph_spacing: String,

    /// 数学公式间距 (tight|normal|loose|relaxed 或数值如 20px)
    #[arg(long, default_value = "tight")]
    pub math_spacing: String,

    /// Chrome 可执行文件路径 (可选，留空则自动搜索)
    #[arg(long)]
    pub chrome: Option<PathBuf>,
}
