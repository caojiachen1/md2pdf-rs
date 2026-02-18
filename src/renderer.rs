/// renderer.rs — Markdown + LaTeX math rendering pipeline.
///               Mirrors renderer.js.

use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

// ─────────────────────────────────────────────
//  Math expression extraction
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum MathKind {
    Block,
    Inline,
}

#[derive(Debug, Clone)]
pub struct MathExpr {
    pub kind: MathKind,
    pub content: String,
    pub placeholder: String,
}

/// Extract all math expressions and replace them with HTML-comment placeholders.
/// Processing order matches renderer.js:
///   1. Block: `$$...$$` and `\[...\]`  (must come before inline)
///   2. Inline: `$...$` and `\(...\)`
///
/// An inline expression containing a newline is promoted to block.
pub fn process_math_expressions(content: &str) -> (String, Vec<MathExpr>) {
    let mut exprs: Vec<MathExpr> = Vec::new();
    let mut text = content.to_string();

    // ── Block: $$...$$ ────────────────────────────────────────────────────────
    {
        let re = Regex::new(r"(?s)\$\$([\s\S]*?)\$\$").unwrap();
        let mut result = String::new();
        let mut last = 0usize;
        for cap in re.captures_iter(&text.clone()) {
            let m = cap.get(0).unwrap();
            let math_content = cap[1].trim().to_string();
            let idx = exprs.len();
            let placeholder = format!("<!--MATH_BLOCK_{}-->", idx);
            exprs.push(MathExpr {
                kind: MathKind::Block,
                content: math_content,
                placeholder: placeholder.clone(),
            });
            result.push_str(&text[last..m.start()]);
            result.push_str(&placeholder);
            last = m.end();
        }
        result.push_str(&text[last..]);
        text = result;
    }

    // ── Block: \[...\] ───────────────────────────────────────────────────────
    {
        let re = Regex::new(r"(?s)\\\[([\s\S]*?)\\\]").unwrap();
        let mut result = String::new();
        let mut last = 0usize;
        for cap in re.captures_iter(&text.clone()) {
            let m = cap.get(0).unwrap();
            let math_content = cap[1].trim().to_string();
            let idx = exprs.len();
            let placeholder = format!("<!--MATH_BLOCK_{}-->", idx);
            exprs.push(MathExpr {
                kind: MathKind::Block,
                content: math_content,
                placeholder: placeholder.clone(),
            });
            result.push_str(&text[last..m.start()]);
            result.push_str(&placeholder);
            last = m.end();
        }
        result.push_str(&text[last..]);
        text = result;
    }

    // ── Inline: $...$ (not $$, not escaped \$) ───────────────────────────────
    // Rust's `regex` crate lacks look-around, so we scan byte-by-byte.
    {
        let bytes = text.as_bytes();
        let len = bytes.len();
        let mut result = String::new();
        let mut i = 0usize;

        while i < len {
            if bytes[i] == b'$' {
                let prev_dollar  = i > 0 && bytes[i - 1] == b'$';
                let prev_escape  = i > 0 && bytes[i - 1] == b'\\';
                let next_dollar  = i + 1 < len && bytes[i + 1] == b'$';

                if !prev_dollar && !prev_escape && !next_dollar {
                    let start = i;
                    let mut j = i + 1;
                    let mut found_close = false;
                    while j < len {
                        if bytes[j] == b'$'
                            && bytes[j - 1] != b'\\'
                            && !(j + 1 < len && bytes[j + 1] == b'$')
                            && bytes[j - 1] != b'$'
                        {
                            found_close = true;
                            let math_content = &text[start + 1..j];
                            let is_block = math_content.contains('\n');
                            let idx = exprs.len();
                            let placeholder = if is_block {
                                format!("<!--MATH_BLOCK_{}-->", idx)
                            } else {
                                format!("<!--MATH_INLINE_{}-->", idx)
                            };
                            exprs.push(MathExpr {
                                kind: if is_block { MathKind::Block } else { MathKind::Inline },
                                content: math_content.trim().to_string(),
                                placeholder: placeholder.clone(),
                            });
                            result.push_str(&placeholder);
                            i = j + 1;
                            break;
                        }
                        j += 1;
                    }
                    if !found_close {
                        result.push('$');
                        i = start + 1;
                    }
                    continue;
                }
            }
            let ch = text[i..].chars().next().unwrap();
            result.push(ch);
            i += ch.len_utf8();
        }
        text = result;
    }

    // ── Inline: \(...\) ──────────────────────────────────────────────────────
    {
        let re = Regex::new(r"(?s)\\\(([\s\S]*?)\\\)").unwrap();
        let mut result = String::new();
        let mut last = 0usize;
        for cap in re.captures_iter(&text.clone()) {
            let m = cap.get(0).unwrap();
            let math_content = cap[1].trim().to_string();
            let is_block = math_content.contains('\n');
            let idx = exprs.len();
            let placeholder = if is_block {
                format!("<!--MATH_BLOCK_{}-->", idx)
            } else {
                format!("<!--MATH_INLINE_{}-->", idx)
            };
            exprs.push(MathExpr {
                kind: if is_block { MathKind::Block } else { MathKind::Inline },
                content: math_content,
                placeholder: placeholder.clone(),
            });
            result.push_str(&text[last..m.start()]);
            result.push_str(&placeholder);
            last = m.end();
        }
        result.push_str(&text[last..]);
        text = result;
    }

    (text, exprs)
}

// ─────────────────────────────────────────────
//  Math → HTML wrapper
// ─────────────────────────────────────────────

/// Wrap a TeX expression in an HTML container.
/// Actual rendering is performed client-side by KaTeX's auto-render script.
pub fn generate_math_html(tex: &str, is_block: bool) -> String {
    if is_block {
        format!(
            r#"<div class="math-block"><span class="katex-display">$${}$$</span></div>"#,
            tex
        )
    } else {
        format!(r#"<span class="math-inline">${}$</span>"#, tex)
    }
}

// ─────────────────────────────────────────────
//  Markdown rendering
// ─────────────────────────────────────────────

/// Escape HTML special characters.  Mirrors `escapeHtml` in utils.js.
#[allow(dead_code)]
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Render Markdown source (math already replaced by placeholders) to an HTML fragment.
/// Uses pulldown-cmark with strikethrough, tables, footnotes and task-lists.
pub fn render_markdown(content: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, opts);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);

    html_out
        .replace("<p></p>", "")
        .replace("<p>\n</p>", "")
}

// ─────────────────────────────────────────────
//  Full render pipeline
// ─────────────────────────────────────────────

/// Extract math → render markdown → restore math.
/// Mirrors `MarkdownLatexRenderer.render()`.
pub fn render(content: &str) -> String {
    let (processed, math_exprs) = process_math_expressions(content);
    let mut html = render_markdown(&processed);

    for expr in &math_exprs {
        let math_html = generate_math_html(&expr.content, matches!(expr.kind, MathKind::Block));
        html = html.replacen(&expr.placeholder, &math_html, 1);
    }

    html
}
