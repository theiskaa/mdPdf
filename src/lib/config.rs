//! Configuration module for styling and formatting PDF output.
//!
//! This module handles loading and parsing of styling configuration from TOML files.
//! It provides functionality to customize text styles, colors, margins and other formatting
//! options for different Markdown elements in the generated PDF.
//!
//! # Configuration Format
//! The configuration uses TOML format with sections for different element types:
//!
//! - `[margin]` - Document margins
//! - `[heading.N]` - Heading styles (N=1,2,3)
//! - `[text]` - Base text style
//! - `[emphasis]` - Italic text style
//! - `[strong_emphasis]` - Bold text style
//! - `[code]` - Code block/inline code style
//! - `[block_quote]` - Blockquote style
//! - `[list_item]` - List item style
//! - `[link]` - Link text style
//! - `[image]` - Image caption style
//! - `[horizontal_rule]` - Horizontal rule style
//!
//! Each style section supports these properties:
//!
//! ```toml
//! size = 12 # Font size in points
//! textcolor = { r = 0, g = 0, b = 0 } # RGB text color
//! backgroundcolor = { r = 255, g = 255, b = 255 } # RGB background color
//! afterspacing = 0.5 # Vertical spacing after element
//! alignment = "left" # Text alignment (left|center|right|justify)
//! fontfamily = "roboto" # Font family name
//! bold = false # Bold text
//! italic = false # Italic text
//! underline = false # Underlined text
//! strikethrough = false # Strikethrough text
//! ```
//!
//! # Examples
//!
//! Basic configuration with custom heading styles:
//! ```toml
//! [margin]
//! top = 8.0
//! right = 8.0
//! bottom = 8.0
//! left = 8.0
//!
//! [heading.1]
//! size = 14
//! textcolor = { r = 0, g = 0, b = 0 }
//! afterspacing = 0.5
//! alignment = "center"
//! fontfamily = "roboto"
//! bold = true
//!
//! [text]
//! size = 8
//! textcolor = { r = 0, g = 0, b = 0 }
//! afterspacing = 0.0
//! alignment = "left"
//! fontfamily = "roboto"
//! ```
//!
//! Custom styles for emphasis and links:
//! ```toml
//! [emphasis]
//! size = 10
//! textcolor = { r = 100, g = 100, b = 100 }
//! italic = true
//!
//! [link]
//! textcolor = { r = 0, g = 0, b = 255 }
//! underline = true
//! ```
//!
//! # Processing Pipeline
//! ```text
//! +---------------------+     +----------------+     +----------------+
//! |                     |     |                |     |                |
//! | markdown2pdfrc.toml | --> | TOML Parser    | --> | Style Objects  |
//! | Configuration       |     | (parse_style)  |     | (StyleMatch)   |
//! |                     |     |                |     |                |
//! +---------------------+     +----------------+     +----------------+
//! ```
//!
//! # Example Configuration
//! See `@markdown2pdfrc.example.toml` for a complete example configuration file.

use crate::styling::{BasicTextStyle, Margins, MdPdfFont, StyleMatch, TextAlignment};
use std::fs;
use std::path::Path;
use toml::Value;

/// Parses an RGB color from a TOML configuration value.
///
/// # Arguments
/// * `value` - Optional TOML value containing a color object
/// * `field` - Name of the color field to parse
///
/// # Returns
/// * `Some((r,g,b))` - RGB color values if parsing succeeds
/// * `None` - If the color value is missing or invalid
fn parse_color(value: Option<&Value>, field: &str) -> Option<(u8, u8, u8)> {
    value.and_then(|c| {
        let color = c.get(field)?;
        let r = color.get("r")?.as_integer()? as u8;
        let g = color.get("g")?.as_integer()? as u8;
        let b = color.get("b")?.as_integer()? as u8;
        Some((r, g, b))
    })
}

/// Parses text alignment from TOML configuration.
///
/// # Arguments
/// * `value` - Optional TOML value containing alignment string
///
/// # Returns
/// * `Some(TextAlignment)` - Parsed alignment value
/// * `None` - If alignment value is missing
fn parse_alignment(value: Option<&Value>) -> Option<TextAlignment> {
    value.and_then(|v| v.as_str()).map(|s| match s {
        "left" => TextAlignment::Left,
        "center" => TextAlignment::Center,
        "right" => TextAlignment::Right,
        "justify" => TextAlignment::Justify,
        _ => TextAlignment::Left,
    })
}

/// Maps a font family name to its corresponding font file path.
///
/// # Arguments
/// * `font` - Name of the font family
///
/// # Returns
/// * `Some(&str)` - Path to the font file if found
/// * `None` - If font is not available
fn map_font_family(font: &str) -> Option<&'static str> {
    Some(MdPdfFont::find_match(Some(font)).file())
}

/// Parses a complete text style configuration from TOML.
///
/// Handles all style properties including size, spacing, colors,
/// alignment, font family and text decorations.
///
/// # Arguments
/// * `value` - Optional TOML value containing style configuration
/// * `default` - Default style to use for missing properties
///
/// # Returns
/// Complete `BasicTextStyle` with parsed or default values
fn parse_style(value: Option<&Value>, default: BasicTextStyle) -> BasicTextStyle {
    let mut style = default.clone();
    if let Some(style_config) = value {
        if let Some(size) = style_config.get("size").and_then(|v| v.as_integer()) {
            style.size = size as u8;
        }
        if let Some(spacing) = style_config.get("afterspacing").and_then(|v| v.as_float()) {
            style.after_spacing = spacing as f32;
        }

        if let Some(color) = parse_color(Some(style_config), "textcolor") {
            style.text_color = Some(color);
        }
        if let Some(bg_color) = parse_color(Some(style_config), "backgroundcolor") {
            style.background_color = Some(bg_color);
        }

        // Parse text properties
        if let Some(alignment) = parse_alignment(style_config.get("alignment")) {
            style.alignment = Some(alignment);
        }

        if let Some(font) = style_config.get("fontfamily").and_then(|v| v.as_str()) {
            style.font_family = map_font_family(font);
        }

        // Parse boolean flags
        if let Some(bold) = style_config.get("bold").and_then(|v| v.as_bool()) {
            style.bold = bold;
        }
        if let Some(italic) = style_config.get("italic").and_then(|v| v.as_bool()) {
            style.italic = italic;
        }
        if let Some(underline) = style_config.get("underline").and_then(|v| v.as_bool()) {
            style.underline = underline;
        }
        if let Some(strikethrough) = style_config.get("strikethrough").and_then(|v| v.as_bool()) {
            style.strikethrough = strikethrough;
        }
    }
    style
}

/// Loads and parses the complete styling configuration.
///
/// Attempts to read styling configuration from markdown2pdfrc.toml file.
/// Falls back to default styles if the file is missing or invalid.
///
/// # Returns
/// Complete `StyleMatch` containing all style configurations
///
/// # Example
/// ```rust
/// use markdown2pdf::config;
///
/// let styles = config::load_config();
/// // Use styles for PDF generation
/// ```
pub fn load_config() -> StyleMatch {
    // Try to read config from home directory first, fall back to current directory
    let config_path = dirs::home_dir()
        .map(|mut path| {
            path.push("markdown2pdfrc.toml");
            path
        })
        .unwrap_or_else(|| Path::new("markdown2pdfrc.toml").to_path_buf());

    let config_str = match fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(_) => return StyleMatch::default(),
    };

    let config: Value = match toml::from_str(&config_str) {
        Ok(v) => v,
        Err(_) => return StyleMatch::default(),
    };

    // Get default style to use for missing values
    let default_style = StyleMatch::default();

    // Parse margins
    let margins = if let Some(margins) = config.get("margin") {
        Margins {
            top: margins.get("top").and_then(|v| v.as_float()).unwrap_or(8.0) as f32,
            right: margins
                .get("right")
                .and_then(|v| v.as_float())
                .unwrap_or(8.0) as f32,
            bottom: margins
                .get("bottom")
                .and_then(|v| v.as_float())
                .unwrap_or(8.0) as f32,
            left: margins
                .get("left")
                .and_then(|v| v.as_float())
                .unwrap_or(8.0) as f32,
        }
    } else {
        default_style.margins
    };

    StyleMatch {
        margins,
        heading_1: parse_style(
            config.get("heading").and_then(|h| h.get("1")),
            default_style.heading_1,
        ),
        heading_2: parse_style(
            config.get("heading").and_then(|h| h.get("2")),
            default_style.heading_2,
        ),
        heading_3: parse_style(
            config.get("heading").and_then(|h| h.get("3")),
            default_style.heading_3,
        ),
        emphasis: parse_style(config.get("emphasis"), default_style.emphasis),
        strong_emphasis: parse_style(config.get("strong_emphasis"), default_style.strong_emphasis),
        code: parse_style(config.get("code"), default_style.code),
        block_quote: parse_style(config.get("block_quote"), default_style.block_quote),
        list_item: parse_style(config.get("list_item"), default_style.list_item),
        link: parse_style(config.get("link"), default_style.link),
        image: parse_style(config.get("image"), default_style.image),
        text: parse_style(config.get("text"), default_style.text),
        horizontal_rule: parse_style(config.get("horizontal_rule"), default_style.horizontal_rule),
    }
}
