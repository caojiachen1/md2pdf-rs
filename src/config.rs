/// config.rs — constants / defaults  (mirrors config.js)

// ─────────────────────────────────────────────
//  Font / spacing presets
// ─────────────────────────────────────────────

/// Font-size presets → concrete pixel values  (FONT_SIZE_CONFIG)
pub fn font_size_px(s: &str) -> &str {
    match s {
        "small"  => "12px",
        "medium" => "14px",
        "large"  => "16px",
        "xlarge" => "18px",
        other    => other,
    }
}

/// Font-weight presets → numeric values  (FONT_WEIGHT_CONFIG)
pub fn font_weight_value(s: &str) -> &str {
    match s {
        "light"   => "300",
        "normal"  => "400",
        "medium"  => "500",
        "semibold"=> "600",
        "bold"    => "700",
        "black"   => "900",
        other     => other,
    }
}

/// Chinese-font presets → CSS font-family  (CHINESE_FONT_CONFIG)
pub fn chinese_font_family(s: &str) -> &str {
    match s {
        "simsun"   => r#"SimSun, "宋体", serif"#,
        "simhei"   => r#"SimHei, "黑体", sans-serif"#,
        "simkai"   => r#"KaiTi, "楷体", "STKaiti", serif"#,
        "fangsong" => r#"FangSong, "仿宋", "STFangsong", serif"#,
        "yahei"    => r#""Microsoft YaHei", "微软雅黑", sans-serif"#,
        _ /* auto */ => r#"-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Microsoft YaHei", "微软雅黑", "SimSun", "宋体", sans-serif"#,
    }
}

/// Line-spacing presets → CSS line-height  (LINE_SPACING_CONFIG)
pub fn line_spacing_value(s: &str) -> &str {
    match s {
        "tight"   => "1.2",
        "normal"  => "1.6",
        "loose"   => "2.0",
        "relaxed" => "2.4",
        other     => other,
    }
}

/// Paragraph-spacing presets → CSS margin value  (PARAGRAPH_SPACING_CONFIG)
pub fn paragraph_spacing_value(s: &str) -> &str {
    match s {
        "tight"   => "0.5em",
        "normal"  => "1em",
        "loose"   => "1.5em",
        "relaxed" => "2em",
        other     => other,
    }
}

/// Math-spacing presets → CSS margin value  (MATH_SPACING_CONFIG)
pub fn math_spacing_value(s: &str) -> &str {
    match s {
        "tight"   => "10px",
        "normal"  => "20px",
        "loose"   => "30px",
        "relaxed" => "40px",
        other     => other,
    }
}

/// Normalize numeric-only values by appending a unit  (normalizeNumericOptions in cli.js)
pub fn normalize_with_unit(value: &str, unit: &str) -> String {
    let trimmed = value.trim();
    if trimmed.parse::<f64>().is_ok() {
        format!("{}{}", trimmed, unit)
    } else {
        trimmed.to_string()
    }
}

/// Parse a CSS margin string (e.g. "20mm", "1in", "0.5cm") to inches.
pub fn margin_to_inches(s: &str) -> f64 {
    let s = s.trim();
    if let Some(v) = s.strip_suffix("mm") {
        v.trim().parse::<f64>().unwrap_or(20.0) / 25.4
    } else if let Some(v) = s.strip_suffix("cm") {
        v.trim().parse::<f64>().unwrap_or(2.0) / 2.54
    } else if let Some(v) = s.strip_suffix("in") {
        v.trim().parse::<f64>().unwrap_or(0.787)
    } else if let Some(v) = s.strip_suffix("px") {
        v.trim().parse::<f64>().unwrap_or(0.0) / 96.0
    } else {
        s.parse::<f64>().unwrap_or(20.0) / 25.4
    }
}

// ─────────────────────────────────────────────
//  StyleOptions
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StyleOptions {
    pub font_size: String,
    pub chinese_font: String,
    pub font_weight: String,
    pub line_spacing: String,
    pub paragraph_spacing: String,
    pub math_spacing: String,
}

// ─────────────────────────────────────────────
//  PdfOptions
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PdfOptions {
    pub margin_inches: f64,
    pub landscape: bool,
}

impl Default for PdfOptions {
    fn default() -> Self {
        Self {
            margin_inches: 0.787, // 20mm ≈ 0.787 inches  (PDF_CONFIG default)
            landscape: false,
        }
    }
}

// ─────────────────────────────────────────────
//  Assets directory resolution
// ─────────────────────────────────────────────

/// Resolve the assets directory: next to the executable if present, otherwise CWD/assets.
pub fn resolve_assets_dir() -> std::path::PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));
    exe_dir
        .as_deref()
        .map(|d| d.join("assets"))
        .filter(|p| p.exists())
        .unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_default()
                .join("assets")
        })
}
