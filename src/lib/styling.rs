//! Styling module for markdown-to-pdf conversion.
//!
//! This module provides styling configuration for converting markdown elements to PDF,
//! including fonts, text styles, margins and alignments.
//!
//! # Examples
//! ```rust
//! use mdp::styling::StyleMatch;
//!
//! // Create default styling
//! let styles = StyleMatch::default();
//!
//! // Access specific element styles
//! let heading_size = styles.heading_1.size; // 14pt
//! let text_color = styles.text.text_color; // RGB(0,0,0)
//! let link_underline = styles.link.underline; // true
//! ```
//!
//! ```rust
//! use mdp::styling::{BasicTextStyle, TextAlignment};
//!
//! // Create custom text style
//! let custom_style = BasicTextStyle::new(
//!     12, // size
//!     Some((0, 0, 255)), // blue text
//!     Some(1.0), // spacing after
//!     Some(TextAlignment::Center),
//!     None, // default font
//!     true, // bold
//!     false, // not italic
//!     false, // not underlined
//!     false, // no strikethrough
//!     None, // no background
//! );
//! ```
//!
//! # Styling Configuration
//! The styling system supports customization through a TOML configuration file.
//! Each element type can have the following properties:
//!
//! ```toml
//! [element_name]
//! size = 12 # Font size in points
//! textcolor = { r = 0, g = 0, b = 0 } # RGB text color
//! backgroundcolor = { r = 255, g = 255, b = 255 } # RGB background
//! afterspacing = 0.5 # Vertical spacing after element
//! alignment = "left" # Text alignment (left|center|right|justify)
//! fontfamily = "roboto" # Font family name
//! bold = false # Bold text
//! italic = false # Italic text
//! underline = false # Underlined text
//! strikethrough = false # Strikethrough text
//! ```
//!
//! Supported element types:
//! - `heading.1`, `heading.2`, `heading.3` - Headings
//! - `text` - Regular text
//! - `emphasis` - Italic text
//! - `strong_emphasis` - Bold text
//! - `code` - Code blocks
//! - `block_quote` - Block quotes
//! - `list_item` - List items
//! - `link` - Links
//! - `image` - Image captions
//! - `horizontal_rule` - Horizontal rules
//!
//! # Processing Pipeline
//! ```text
//! +----------------+     +------------------+     +------------------+
//! | Markdown       | --> | Style            | --> | PDF Element      |
//! | Element        |     | Configuration    |     | with Styling     |
//! +----------------+     +------------------+     +------------------+
//!       |                       |                        |
//!       |                       |                        |
//!       v                       v                        v
//! (e.g. # Heading)    (size: 14, bold: true)     (Formatted heading in PDF document)
//! ```
//!
//! Font Processing:
//! ```text
//! +--------------+     +---------------+     +------------------+
//! | Font Family  | --> | Font Loading  | --> | PDF Font        |
//! | Selection    |     | from Assets   |     | Registration    |
//! +--------------+     +---------------+     +------------------+
//! (e.g. "roboto")     (Load TTF files)      (Ready for use in document)
//! ```

use genpdf::{
    error::Error,
    fonts::{FontData, FontFamily},
};

/// Available font families that can be used in the PDF document.
/// Currently only supports Roboto font.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MdPdfFont {
    Roboto,
}

impl MdPdfFont {
    /// Returns the directory name where the font files are stored.
    pub fn dir(&self) -> &'static str {
        match self {
            MdPdfFont::Roboto => "roboto",
        }
    }

    /// Returns the base filename of the font files without extension.
    pub fn file(&self) -> &'static str {
        match self {
            MdPdfFont::Roboto => "Roboto",
        }
    }

    /// Finds a matching font family based on the provided name.
    /// Currently defaults to Roboto for all inputs.
    ///
    /// # Arguments
    /// * `family` - Optional font family name to match
    pub fn find_match(family: Option<&str>) -> MdPdfFont {
        match family.unwrap_or("roboto") {
            _ => MdPdfFont::Roboto,
        }
    }

    /// Loads a font family from files in the assets directory.
    ///
    /// # Arguments
    /// * `family` - Optional font family name to load
    ///
    /// # Returns
    /// Result containing the loaded FontFamily or an Error
    pub fn load_font_family(family: Option<&str>) -> Result<FontFamily<FontData>, Error> {
        let found_match = MdPdfFont::find_match(family);
        let path = format!("assets/fonts/{}", found_match.dir());
        genpdf::fonts::from_files(path.as_str(), found_match.file(), None)
    }
}

/// Text alignment options for PDF elements.
#[derive(Clone, Copy)]
pub enum TextAlignment {
    /// Align text to the left margin
    Left,
    /// Center text between margins
    Center,
    /// Align text to the right margin
    Right,
    /// Spread text evenly between margins
    Justify,
}

/// Document margins configuration in points.
#[derive(Clone, Copy)]
pub struct Margins {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

/// Basic text styling properties that can be applied to any text element.
#[derive(Clone, Copy)]
pub struct BasicTextStyle {
    /// Font size in points
    pub size: u8,
    /// Text color in RGB format
    pub text_color: Option<(u8, u8, u8)>,
    /// Space after element in points
    pub after_spacing: f32,
    /// Text alignment within container
    pub alignment: Option<TextAlignment>,
    /// Font family name
    pub font_family: Option<&'static str>,
    /// Whether text should be bold
    pub bold: bool,
    /// Whether text should be italic
    pub italic: bool,
    /// Whether text should be underlined
    pub underline: bool,
    /// Whether text should have strikethrough
    pub strikethrough: bool,
    /// Background color in RGB format
    pub background_color: Option<(u8, u8, u8)>,
}

impl BasicTextStyle {
    /// Creates a new BasicTextStyle with the specified properties.
    ///
    /// # Arguments
    /// * `size` - Font size in points
    /// * `text_color` - Optional RGB color tuple for text
    /// * `after_spacing` - Optional space after element in points
    /// * `alignment` - Optional text alignment
    /// * `font_family` - Optional font family name
    /// * `bold` - Whether text should be bold
    /// * `italic` - Whether text should be italic
    /// * `underline` - Whether text should be underlined
    /// * `strikethrough` - Whether text should have strikethrough
    /// * `background_color` - Optional RGB color tuple for background
    pub fn new(
        size: u8,
        text_color: Option<(u8, u8, u8)>,
        after_spacing: Option<f32>,
        alignment: Option<TextAlignment>,
        font_family: Option<&'static str>,
        bold: bool,
        italic: bool,
        underline: bool,
        strikethrough: bool,
        background_color: Option<(u8, u8, u8)>,
    ) -> Self {
        Self {
            size,
            text_color,
            after_spacing: after_spacing.unwrap_or(0.0),
            alignment,
            font_family,
            bold,
            italic,
            underline,
            strikethrough,
            background_color,
        }
    }
}

/// Main style configuration for mapping markdown elements to PDF styles.
///
/// This struct contains style definitions for each markdown element type
/// that can appear in the document. It is used by the PDF renderer to
/// determine how to format each element.
pub struct StyleMatch {
    /// Document margins
    pub margins: Margins,
    /// Style for level 1 headings (#)
    pub heading_1: BasicTextStyle,
    /// Style for level 2 headings (##)
    pub heading_2: BasicTextStyle,
    /// Style for level 3 headings (###)
    pub heading_3: BasicTextStyle,
    /// Style for emphasized text (*text* or _text_)
    pub emphasis: BasicTextStyle,
    /// Style for strongly emphasized text (**text** or __text__)
    pub strong_emphasis: BasicTextStyle,
    /// Style for inline code (`code`)
    pub code: BasicTextStyle,
    /// Style for block quotes (> quote)
    pub block_quote: BasicTextStyle,
    /// Style for list items (- item or * item)
    pub list_item: BasicTextStyle,
    /// Style for links ([text](url))
    pub link: BasicTextStyle,
    /// Style for images (![alt](url))
    pub image: BasicTextStyle,
    /// Style for regular text
    pub text: BasicTextStyle,
    /// Style for horizontal rules (---)
    pub horizontal_rule: BasicTextStyle,
}

impl StyleMatch {
    /// Creates a StyleMatch with default styling settings.
    ///
    /// The default style provides a clean, readable layout with:
    /// - Hierarchical heading sizes (14pt, 12pt, 10pt)
    /// - 8pt base font size for body text
    /// - Appropriate styling for code blocks and quotes
    /// - Consistent spacing and margins
    ///
    /// # Returns
    /// A new StyleMatch instance with default settings
    pub fn default() -> Self {
        Self {
            margins: Margins {
                top: 8.0,
                right: 8.0,
                bottom: 8.0,
                left: 8.0,
            },
            heading_1: BasicTextStyle::new(
                14,
                Some((0, 0, 0)),
                Some(0.5),
                Some(TextAlignment::Center),
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            heading_2: BasicTextStyle::new(
                12,
                Some((0, 0, 0)),
                Some(0.5),
                Some(TextAlignment::Left),
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            heading_3: BasicTextStyle::new(
                10,
                Some((0, 0, 0)),
                Some(0.5),
                Some(TextAlignment::Left),
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            emphasis: BasicTextStyle::new(
                8,
                Some((0, 0, 0)),
                None,
                None,
                None,
                false,
                true,
                false,
                false,
                None,
            ),
            strong_emphasis: BasicTextStyle::new(
                8,
                Some((0, 0, 0)),
                None,
                None,
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            code: BasicTextStyle::new(
                8,
                Some((128, 128, 128)),
                None,
                None,
                Some("Roboto"),
                false,
                false,
                false,
                false,
                Some((230, 230, 230)),
            ),
            block_quote: BasicTextStyle::new(
                8,
                Some((128, 128, 128)),
                None,
                None,
                None,
                false,
                true,
                false,
                false,
                Some((245, 245, 245)),
            ),
            list_item: BasicTextStyle::new(
                8,
                Some((0, 0, 0)),
                Some(0.5),
                None,
                None,
                false,
                false,
                false,
                false,
                None,
            ),
            link: BasicTextStyle::new(
                8,
                Some((128, 128, 128)),
                None,
                None,
                None,
                false,
                false,
                true,
                false,
                None,
            ),
            image: BasicTextStyle::new(
                8,
                Some((0, 0, 0)),
                None,
                Some(TextAlignment::Center),
                None,
                false,
                false,
                false,
                false,
                None,
            ),
            text: BasicTextStyle::new(
                8,
                Some((0, 0, 0)),
                None,
                None,
                None,
                false,
                false,
                false,
                false,
                None,
            ),
            horizontal_rule: BasicTextStyle::new(
                8,
                Some((0, 0, 0)),
                Some(0.5),
                None,
                None,
                false,
                false,
                false,
                false,
                None,
            ),
        }
    }
}
