//! Configuration module for styling and formatting PDF output.
//!
//! This module handles loading and parsing of styling configuration from TOML files.
//! It provides functionality to customize text styles, colors, margins and other formatting
//! options for different Markdown elements in the generated PDF.
//!
//! The configuration uses TOML format with sections for different element types. The margin
//! section controls document margins. Heading styles can be customized per level (1-3).
//! Base text style defines the default text appearance. Emphasis sections handle italic text.
//! Strong emphasis controls bold text styling. Code sections format code blocks and inline code.
//! Block quote sections style quoted text. List item sections format list entries. Link and
//! image sections control how those elements appear. A horizontal rule section styles divider lines.
//!
//! Each style section supports properties for customizing the appearance. Font size is specified
//! in points. Text and background colors use RGB values. Vertical spacing after elements can be
//! set. Text alignment options include left, center, right and justify. Font family specifies
//! the typeface. Boolean flags control bold, italic, underline and strikethrough decorations.
//!
//! The configuration can be defined in a TOML file. Basic configuration might customize heading
//! styles with specific sizes, colors and alignment. Text styles control the base paragraph
//! appearance. Custom emphasis styles can modify how italic and bold text renders. Link styles
//! determine how hyperlinks are displayed.
//!
//! The configuration processing follows a pipeline where the TOML file is parsed into style
//! objects that control the PDF generation. The parser extracts style properties and creates
//! corresponding style objects used during rendering.
//!
//! A complete example configuration file can be found in markdown2pdfrc.example.toml which
//! demonstrates all available styling options.

use crate::styling::{BasicTextStyle, Margins, MdPdfFont, StyleMatch, TextAlignment};
use std::fs;
use std::path::Path;
use toml::Value;

/// Parses an RGB color from a TOML configuration value.
///
/// The value parameter provides an optional TOML value containing a color object.
/// The field parameter specifies which color field to parse from the configuration.
/// Returns the RGB color values as a tuple if parsing succeeds, or None if the color
/// value is missing or invalid.
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
/// Takes an optional TOML value containing the alignment string specification.
/// Returns the corresponding TextAlignment enum value if parsing succeeds,
/// or None if the alignment value is missing from the configuration.
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
/// Takes the name of the font family as input and attempts to find a matching
/// font file. Returns the path to the font file if found, or None if the
/// specified font is not available in the system.
fn map_font_family(font: &str) -> Option<&'static str> {
    Some(MdPdfFont::find_match(Some(font)).file())
}

/// Parses a complete text style configuration from TOML.
///
/// Processes all style properties including size, spacing, colors, alignment,
/// font family and text decorations. Takes an optional TOML value containing
/// the style configuration and a default style to use for missing properties.
/// Returns a complete BasicTextStyle with all properties set to either parsed
/// or default values.
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
/// Attempts to read styling configuration from the markdown2pdfrc.toml file.
/// First checks the user's home directory for the config file, then falls back
/// to the current directory. If no config file is found or if parsing fails,
/// returns default styles. The function processes all style sections and
/// returns a complete StyleMatch object containing the full configuration.
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
