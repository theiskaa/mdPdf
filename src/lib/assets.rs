//! Asset management module for embedded fonts and resources.
//!
//! This module provides functionality for loading and managing embedded assets like fonts that are compiled
//! into the binary. It handles checking for embedded fonts and retrieving their binary data for use in
//! PDF generation.
//!
//! The module uses a static mapping of font names to their embedded file paths, allowing efficient lookups
//! of embedded font data. Font names are case-insensitive for convenience.

use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::collections::HashMap;

/// Container for embedded resource files like fonts. Uses the assets/ directory
/// as the root for embedded files.
#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

/// Static mapping that associates font names with their corresponding embedded file paths.
/// Font names are stored in lowercase for case-insensitive lookups.
static EMBEDDED_FONTS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("roboto", "fonts/roboto");
    m
});

/// Retrieves the binary data for an embedded font file. Takes a font path relative to the assets
/// folder and returns the raw font data if found. Returns None if the specified font file does
/// not exist in the embedded assets.
pub fn get_font_data(font_path: &str) -> Option<Vec<u8>> {
    Assets::get(font_path).map(|f| f.data.to_vec())
}

/// Checks if a given font name corresponds to an embedded font. The check is case-insensitive,
/// so font names can be provided in any case. Returns true if the font is embedded in the binary,
/// false otherwise.
pub fn is_embedded_font(name: &str) -> bool {
    EMBEDDED_FONTS.contains_key(name.to_lowercase().as_str())
}

/// Retrieves the asset path for an embedded font by name. The lookup is case-insensitive.
/// Returns the font's asset path if the font name is recognized, or None if the font is not
/// found in the embedded assets.
pub fn get_embedded_font_path(name: &str) -> Option<&'static str> {
    EMBEDDED_FONTS.get(name.to_lowercase().as_str()).copied()
}
