/// katex_assets.rs — load KaTeX CSS (with inlined fonts), JS, and auto-render JS.
///                    Mirrors katex-assets.js.

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use regex::Regex;
use std::fs;
use std::path::Path;

// ─────────────────────────────────────────────
//  Internal helpers
// ─────────────────────────────────────────────

/// MIME type for a KaTeX font file.
fn font_mime_type(filename: &str) -> &'static str {
    if filename.ends_with(".woff2") {
        "font/woff2"
    } else if filename.ends_with(".woff") {
        "font/woff"
    } else {
        "font/ttf"
    }
}

// ─────────────────────────────────────────────
//  Public API
// ─────────────────────────────────────────────

/// Read katex.min.css and replace every `url(fonts/X)` reference with an
/// inline base64 data-URL.  Mirrors `getLocalKatexCssWithInlineFonts`.
/// Returns an empty string on any error (template then omits the KaTeX block).
pub fn get_local_katex_css_with_inline_fonts(assets_dir: &Path) -> String {
    let css_path = assets_dir.join("katex").join("katex.min.css");
    let mut css = match fs::read_to_string(&css_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Warning: could not read katex CSS ({}): {}",
                css_path.display(),
                e
            );
            return String::new();
        }
    };

    let fonts_dir = assets_dir.join("katex").join("fonts");
    let re = Regex::new(r"url\(fonts/([^)]+)\)").expect("valid regex");

    let replacements: Vec<(String, String)> = re
        .captures_iter(&css.clone())
        .filter_map(|cap| {
            let font_file = cap[1].to_string();
            let font_path = fonts_dir.join(&font_file);
            match fs::read(&font_path) {
                Ok(bytes) => {
                    let b64 = B64.encode(&bytes);
                    let mime = font_mime_type(&font_file);
                    let data_url = format!("data:{};base64,{}", mime, b64);
                    let original = format!("url(fonts/{})", font_file);
                    let replacement = format!("url({})", data_url);
                    Some((original, replacement))
                }
                Err(_) => None,
            }
        })
        .collect();

    for (orig, repl) in replacements {
        css = css.replacen(&orig, &repl, 1);
    }

    css
}

/// Load local `katex.min.js`.
pub fn get_local_katex_js(assets_dir: &Path) -> String {
    let js_path = assets_dir.join("katex").join("katex.min.js");
    match fs::read_to_string(&js_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Warning: could not read local katex JS ({}): {}",
                js_path.display(),
                e
            );
            String::new()
        }
    }
}

/// Load local KaTeX auto-render extension (`contrib/auto-render.min.js`).
pub fn get_local_katex_auto_render_js(assets_dir: &Path) -> String {
    let js_path = assets_dir
        .join("katex")
        .join("contrib")
        .join("auto-render.min.js");
    match fs::read_to_string(&js_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Warning: could not read local katex auto-render JS ({}): {}",
                js_path.display(),
                e
            );
            String::new()
        }
    }
}
