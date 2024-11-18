//! Asset management module for embedded fonts and resources.
//!
//! This module handles loading and managing embedded assets like fonts that are compiled
//! into the binary. It provides functionality to check for and retrieve embedded font data.
//!
//! # Examples
//! ```rust
//! use markdown2pdf::assets;
//!
//! // Check if a font is embedded
//! if assets::is_embedded_font("roboto") {
//!     // Get the font data
//!     if let Some(font_data) = assets::get_font_data("fonts/roboto") {
//!         // Use the font data...
//!     }
//! }
//! ```

use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::collections::HashMap;

/// Asset container for embedded resources
#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

/// Static mapping of font names to their embedded file paths
static EMBEDDED_FONTS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("roboto", "fonts/roboto");
    m
});

/// Retrieves binary data for an embedded font file.
///
/// # Arguments
/// * `font_path` - Path to the font file within the assets folder
///
/// # Returns
/// * `Some(Vec<u8>)` containing the font data if found
/// * `None` if the font file doesn't exist
pub fn get_font_data(font_path: &str) -> Option<Vec<u8>> {
    Assets::get(font_path).map(|f| f.data.to_vec())
}

/// Checks if a font name corresponds to an embedded font.
///
/// # Arguments
/// * `name` - Name of the font to check
///
/// # Returns
/// * `true` if the font is embedded
/// * `false` if the font is not embedded
pub fn is_embedded_font(name: &str) -> bool {
    EMBEDDED_FONTS.contains_key(name.to_lowercase().as_str())
}

/// Gets the asset path for an embedded font.
///
/// # Arguments
/// * `name` - Name of the embedded font
///
/// # Returns
/// * `Some(&str)` containing the font's asset path if found
/// * `None` if the font name is not recognized
pub fn get_embedded_font_path(name: &str) -> Option<&'static str> {
    EMBEDDED_FONTS.get(name.to_lowercase().as_str()).copied()
}
