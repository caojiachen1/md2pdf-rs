/// converter.rs — Launch headless Chrome and print the HTML to PDF.
///                Mirrors converter.js → MarkdownToPdfConverter.generatePdf().

use crate::config::PdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use std::fs;
use std::path::Path;
use thiserror::Error;

// ─────────────────────────────────────────────
//  Error type
// ─────────────────────────────────────────────

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Browser error: {0}")]
    Browser(String),
    #[error("PDF error: {0}")]
    Pdf(String),
}

// ─────────────────────────────────────────────
//  PDF generation
// ─────────────────────────────────────────────

/// Write the HTML to a sibling temp file, load it in headless Chrome, print to PDF.
pub fn generate_pdf(
    html: &str,
    output_path: &Path,
    pdf_opts: &PdfOptions,
    chrome_path: Option<&Path>,
) -> Result<(), AppError> {
    let html_path = output_path.with_extension("html");
    fs::write(&html_path, html)?;

    let path_str = html_path.to_string_lossy().replace('\\', "/");
    let file_url = if path_str.starts_with('/') {
        format!("file://{}", path_str)
    } else {
        format!("file:///{}", path_str)
    };

    println!("[1/5] 正在启动浏览器 (Headless Chrome)...");

    let mut builder = LaunchOptions::default_builder();
    builder
        .headless(true)
        .sandbox(false)
        .idle_browser_timeout(std::time::Duration::from_secs(3600 * 24 * 365 * 100))
        .args(vec![
            std::ffi::OsStr::new("--no-sandbox"),
            std::ffi::OsStr::new("--disable-setuid-sandbox"),
            std::ffi::OsStr::new("--disable-dev-shm-usage"),
            std::ffi::OsStr::new("--disable-extensions"),
            std::ffi::OsStr::new("--disable-gpu"),
            std::ffi::OsStr::new("--disable-background-timer-throttling"),
            std::ffi::OsStr::new("--disable-renderer-backgrounding"),
            std::ffi::OsStr::new("--disable-backgrounding-occluded-windows"),
            std::ffi::OsStr::new("--disable-hang-monitor"),
        ]);

    if let Some(p) = chrome_path {
        builder.path(Some(p.to_path_buf()));
    }

    let launch_opts = builder
        .build()
        .map_err(|e| AppError::Browser(format!("launch options error: {}", e)))?;

    let browser = Browser::new(launch_opts)
        .map_err(|e| AppError::Browser(format!("cannot start browser: {}", e)))?;

    println!("[2/5] 正在创建新标签页...");
    let tab = browser
        .new_tab()
        .map_err(|e| AppError::Browser(format!("new tab failed: {}", e)))?;

    tab.set_default_timeout(std::time::Duration::from_secs(3600 * 24 * 365 * 100));

    println!("[3/5] 正在加载页面: {} ...", file_url);
    tab.navigate_to(&file_url)
        .map_err(|e| AppError::Browser(format!("navigation failed: {}", e)))?;
    tab.wait_until_navigated()
        .map_err(|e| AppError::Browser(format!("wait navigated failed: {}", e)))?;

    println!("[4/5] 正在等待数学公式动态渲染完成...");
    tab.wait_for_element("#render-complete")
        .map_err(|e| AppError::Browser(format!("wait for rendering complete failed: {}", e)))?;

    println!("[5/5] 正在生成 PDF...");
    let pdf_print_opts = headless_chrome::types::PrintToPdfOptions {
        print_background: Some(true),
        paper_width:  Some(8.27),
        paper_height: Some(11.69),
        margin_top:    Some(pdf_opts.margin_inches),
        margin_right:  Some(pdf_opts.margin_inches),
        margin_bottom: Some(pdf_opts.margin_inches),
        margin_left:   Some(pdf_opts.margin_inches),
        landscape: Some(pdf_opts.landscape),
        ..Default::default()
    };

    let pdf_data = tab
        .print_to_pdf(Some(pdf_print_opts))
        .map_err(|e| AppError::Pdf(format!("print to pdf failed: {}", e)))?;

    fs::write(output_path, pdf_data)?;

    // Clean up temp HTML
    let _ = fs::remove_file(&html_path);

    Ok(())
}
