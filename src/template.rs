/// template.rs — Build the full HTML document.  Mirrors template.js.

use crate::config::{
    chinese_font_family, font_size_px, font_weight_value, line_spacing_value,
    math_spacing_value, paragraph_spacing_value, StyleOptions,
};

// ─────────────────────────────────────────────
//  CSS generation
// ─────────────────────────────────────────────

/// Build the CSS block.  Mirrors `getCssStyles()` in template.js.
pub fn get_css_styles(opts: &StyleOptions) -> String {
    let font_size      = font_size_px(&opts.font_size);
    let font_family    = chinese_font_family(&opts.chinese_font);
    let font_weight_val = font_weight_value(&opts.font_weight);
    let line_spacing_val = line_spacing_value(&opts.line_spacing);
    let para_spacing_val = paragraph_spacing_value(&opts.paragraph_spacing);
    let math_spacing_val = math_spacing_value(&opts.math_spacing);

    // px → pt for print media (1px = 0.75pt)
    let px_num: f64 = font_size
        .trim_end_matches("px")
        .parse()
        .unwrap_or(14.0);
    let pt_size = format!("{}pt", px_num * 0.75);

    format!(
        r#"
        /* 基础样式 */
        body {{
            font-family: {font_family};
            font-weight: {font_weight_val};
            line-height: {line_spacing_val};
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            color: #333;
            background-color: #fff;
            font-size: {font_size};
        }}

        /* 段落间距 */
        p {{
            margin-top: 0;
            margin-bottom: {para_spacing_val};
        }}

        /* 列表项间距 */
        li {{
            margin-bottom: calc({para_spacing_val} * 0.5);
        }}

        /* 数学公式样式 */
        .math-block {{
            margin: {math_spacing_val} 0;
            text-align: center;
            overflow-x: auto;
        }}

        .math-inline {{
            display: inline;
        }}

        /* 代码样式 */
        pre {{
            background-color: #f6f8fa;
            border: 1px solid #e1e4e8;
            border-radius: 6px;
            padding: 16px;
            overflow-x: auto;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
            font-size: 14px;
            line-height: 1.45;
        }}

        code {{
            background-color: rgba(175, 184, 193, 0.2);
            border-radius: 6px;
            padding: 2px 4px;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
            font-size: 85%;
        }}

        pre code {{
            background-color: transparent;
            border-radius: 0;
            padding: 0;
            font-size: 100%;
        }}

        /* 表格样式 */
        table {{
            border-collapse: collapse;
            margin: 25px 0;
            font-size: 0.9em;
            min-width: 400px;
            border-radius: 5px 5px 0 0;
            overflow: hidden;
            box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);
        }}

        table thead tr {{
            background-color: #009879;
            color: #ffffff;
            text-align: left;
        }}

        table th,
        table td {{
            padding: 12px 15px;
            border: 1px solid #dddddd;
        }}

        table tbody tr {{
            border-bottom: 1px solid #dddddd;
        }}

        table tbody tr:nth-of-type(even) {{
            background-color: #f3f3f3;
        }}

        /* 引用样式 */
        blockquote {{
            border-left: 4px solid #dfe2e5;
            padding: 0 16px;
            color: #6a737d;
            background-color: #f6f8fa;
            margin: {para_spacing_val} 0;
            line-height: {line_spacing_val};
        }}

        /* 标题样式 */
        h1, h2, h3, h4, h5, h6 {{
            margin-top: calc({para_spacing_val} * 1.5);
            margin-bottom: {para_spacing_val};
            font-weight: 600;
            line-height: {line_spacing_val};
        }}

        h1 {{
            font-size: 2em;
            border-bottom: 1px solid #eaecef;
            padding-bottom: 0.3em;
        }}

        h2 {{
            font-size: 1.5em;
            border-bottom: 1px solid #eaecef;
            padding-bottom: 0.3em;
        }}

        /* 链接样式 */
        a {{
            color: #0366d6;
            text-decoration: none;
        }}

        a:hover {{
            text-decoration: underline;
        }}

        /* 打印样式 */
        @media print {{
            body {{
                max-width: none;
                margin: 0;
                padding: 15mm;
                font-size: {pt_size};
            }}

            .math-block {{
                page-break-inside: avoid;
            }}

            pre {{
                page-break-inside: avoid;
                white-space: pre-wrap;
            }}

            table {{
                page-break-inside: avoid;
            }}

            h1, h2, h3, h4, h5, h6 {{
                page-break-after: avoid;
            }}
        }}
"#
    )
}

// ─────────────────────────────────────────────
//  HTML document assembly
// ─────────────────────────────────────────────

/// Build the full HTML document.  Mirrors `generateHtmlDocument()` in template.js.
pub fn generate_html_document(
    content: &str,
    title: &str,
    katex_css: &str,
    katex_js: &str,
    katex_auto_render_js: &str,
    style_opts: &StyleOptions,
) -> String {
    let css = get_css_styles(style_opts);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        {katex_css}
        {css}
    </style>
    <!-- Local KaTeX JavaScript Library (inlined) -->
    <script>
    {katex_js}
    </script>
    <!-- Local KaTeX Auto-render Extension (inlined) -->
    <script>
    {katex_auto_render_js}
    </script>
</head>
<body>
        {content}
    <script>
    // KaTeX auto-render — applied after DOM is ready
    document.addEventListener("DOMContentLoaded", function() {{
        try {{
            if (typeof renderMathInElement !== 'undefined') {{
                renderMathInElement(document.body, {{
                    delimiters: [
                        {{left: '$$', right: '$$', display: true}},
                        {{left: '$', right: '$', display: false}},
                        {{left: '\\\\(', right: '\\\\)', display: false}},
                        {{left: '\\\\[', right: '\\\\]', display: true}}
                    ],
                    throwOnError: false
                }});
            }}
        }} finally {{
            // 写入哨兵元素，通知 Rust 端公式渲染已结束
            var done = document.createElement("div");
            done.id = "render-complete";
            done.style.display = "none";
            document.body.appendChild(done);
        }}
    }});
    </script>
</body>
</html>"#
    )
}
