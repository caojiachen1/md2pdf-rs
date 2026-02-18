mod cli;
mod config;
mod converter;
mod katex_assets;
mod renderer;
mod template;

use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;

use config::{margin_to_inches, normalize_with_unit, resolve_assets_dir, PdfOptions, StyleOptions};
use converter::generate_pdf;
use katex_assets::{
    get_local_katex_auto_render_js, get_local_katex_css_with_inline_fonts, get_local_katex_js,
};
use renderer::render;
use template::generate_html_document;

// 
//  Entry point
// 

fn print_title() {
    println!();
    println!("");
    println!("  Markdown LaTeX  PDF 转换器     ");
    println!("  支持数学公式 | 美观排版          ");
    println!("");
    println!();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_title();

    let args = cli::Args::parse();

    //  Validate input 
    if !args.input.exists() {
        eprintln!("错误: 输入文件不存在: {}", args.input.display());
        std::process::exit(1);
    }

    //  Normalize numeric options 
    let margin            = normalize_with_unit(&args.margin, "mm");
    let font_size         = normalize_with_unit(&args.font_size, "px");
    let paragraph_spacing = normalize_with_unit(&args.paragraph_spacing, "em");
    let math_spacing      = normalize_with_unit(&args.math_spacing, "px");

    //  Determine output path 
    let output_path: PathBuf = args.output.unwrap_or_else(|| {
        let ext = if args.format == "html" { "html" } else { "pdf" };
        args.input.with_extension(ext)
    });
    let output_path = if output_path.is_absolute() {
        output_path
    } else {
        std::env::current_dir()?.join(output_path)
    };

    //  Print settings 
    println!("开始转换...");
    println!("  输入:     {}", args.input.display());
    println!("  输出:     {}", output_path.display());
    println!("  格式:     {}", args.format.to_uppercase());
    println!("  字体大小: {}", font_size);
    println!("  页边距:   {}", margin);
    println!("  中文字体: {}", args.chinese_font);
    println!("  文字厚度: {}", args.font_weight);
    println!("  行间距:   {}", args.line_spacing);
    println!("  段落间距: {}", paragraph_spacing);
    println!("  公式间距: {}", math_spacing);
    if args.landscape {
        println!("  页面方向: 横向");
    }
    println!();

    let style_opts = StyleOptions {
        font_size,
        chinese_font:       args.chinese_font.clone(),
        font_weight:        args.font_weight.clone(),
        line_spacing:       args.line_spacing.clone(),
        paragraph_spacing,
        math_spacing,
    };

    //  Locate assets directory 
    let assets_dir = resolve_assets_dir();

    let start = std::time::Instant::now();

    //  Phase 1: read markdown 
    println!("读取 Markdown 文件...");
    let markdown = fs::read_to_string(&args.input)?;

    //  Phase 2: load KaTeX assets 
    println!("加载 KaTeX 本地资源 (CSS, JS, 字体)...");
    let katex_css            = get_local_katex_css_with_inline_fonts(&assets_dir);
    let katex_js             = get_local_katex_js(&assets_dir);
    let katex_auto_render_js = get_local_katex_auto_render_js(&assets_dir);

    //  Phase 3: render markdown + math  HTML fragment 
    println!("渲染 HTML 内容...");
    let html_body = render(&markdown);

    //  Phase 4: wrap in full HTML document 
    let title = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Markdown to PDF")
        .to_string();
    let full_html = generate_html_document(
        &html_body,
        &title,
        &katex_css,
        &katex_js,
        &katex_auto_render_js,
        &style_opts,
    );

    //  Phase 5: output 
    match args.format.as_str() {
        "html" => {
            println!("保存 HTML 文件...");
            fs::write(&output_path, &full_html)?;
            println!("\n转换完成! (耗时: {:.1}秒)", start.elapsed().as_secs_f32());
            println!("文件已生成: {}", output_path.display());
        }
        "pdf" => {
            let pdf_opts = PdfOptions {
                margin_inches: margin_to_inches(&margin),
                landscape: args.landscape,
            };
            let output_path_display = output_path.display().to_string();
            tokio::task::spawn_blocking(move || {
                generate_pdf(&full_html, &output_path, &pdf_opts, args.chrome.as_deref())
            })
            .await??;

            println!("\n转换完成! (耗时: {:.1}秒)", start.elapsed().as_secs_f32());
            println!("文件已生成: {}", output_path_display);
        }
        other => {
            eprintln!("不支持的格式: {}", other);
            std::process::exit(1);
        }
    }

    Ok(())
}
