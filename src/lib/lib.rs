//! The markdown2pdf library enables conversion of Markdown content into professionally styled PDF documents.
//! It provides a complete pipeline for parsing Markdown text, applying configurable styling rules, and
//! generating polished PDF output.
//!
//! The library handles the intricacies of Markdown parsing and PDF generation while giving users control
//! over the visual presentation through styling configuration. Users can customize fonts, colors, spacing,
//! and other visual properties via a TOML configuration file.
//!
//! Basic usage involves passing Markdown content as a string along with an output path:
//! ```rust
//! use markdown2pdf;
//!
//! // Convert Markdown string to PDF
//! let markdown = "# Hello World\nThis is a test.".to_string();
//! if let Err(e) = markdown2pdf::parse(markdown, "output.pdf") {
//!     eprintln!("Failed to generate PDF: {}", e);
//! }
//! ```
//!
//! For more control over the output styling, users can create a configuration file (markdown2pdfrc.toml)
//! to specify custom visual properties:
//! ```rust
//! use markdown2pdf;
//! use std::fs;
//!
//! // Read markdown file
//! let markdown = fs::read_to_string("input.md").unwrap();
//!
//! // Convert with custom styling from markdown2pdfrc.toml
//! markdown2pdf::parse(markdown, "styled-output.pdf").unwrap();
//! ```
//!
//! The library also handles rich content like images and links seamlessly:
//! ```rust
//! let markdown = r#"
//! # Document Title
//!
//! ![Logo](./images/logo.png)
//!
//! See our [website](https://example.com) for more info.
//! "#.to_string();
//!
//! markdown2pdf::parse(markdown, "doc-with-images.pdf").unwrap();
//! ```
//!
//! The styling configuration file supports comprehensive customization of the document appearance.
//! Page layout properties control the overall document structure:
//! ```toml
//! [page]
//! margins = { top = 72, right = 72, bottom = 72, left = 72 }
//! size = "a4"
//! orientation = "portrait"
//! ```
//!
//! Individual elements can be styled with precise control:
//! ```toml
//! [heading.1]
//! size = 24
//! textcolor = { r = 0, g = 0, b = 0 }
//! bold = true
//! afterspacing = 1.0
//!
//! [text]
//! size = 12
//! fontfamily = "roboto"
//! alignment = "left"
//!
//! [code]
//! backgroundcolor = { r = 245, g = 245, b = 245 }
//! fontfamily = "roboto-mono"
//! ```
//!
//! The conversion process follows a carefully structured pipeline. First, the Markdown text undergoes
//! lexical analysis to produce a stream of semantic tokens. These tokens then receive styling rules
//! based on the configuration. Finally, the styled elements are rendered into the PDF document.
//!
//! ## Token Processing Flow
//! ```text
//! +-------------+     +----------------+     +----------------+
//! |  Markdown   |     |  Tokens        |     |  PDF Elements  |
//! |  Input      |     |  # -> Heading  |     |  - Styled      |
//! |  # Title    | --> |  * -> List     | --> |    Heading     |
//! |  * Item     |     |  > -> Quote    |     |  - List with   |
//! |  > Quote    |     |                |     |    bullets     |
//! +-------------+     +----------------+     +----------------+
//!
//! +---------------+     +------------------+     +--------------+
//! | Styling       |     | Font Loading     |     | Output:      |
//! | - Font sizes  | --> | - Font families  | --> | Final        |
//! | - Colors      |     | - Weights        |     | Rendered     |
//! | - Margins     |     | - Styles         |     | PDF Document |
//! +---------------+     +------------------+     +--------------+
//! ```

pub mod assets;
pub mod config;
pub mod markdown;
pub mod pdf;
pub mod styling;

use markdown::*;
use pdf::Pdf;
use std::error::Error;
use std::fmt;

/// Represents errors that can occur during the markdown-to-pdf conversion process.
/// This includes both parsing failures and PDF generation issues.
#[derive(Debug)]
pub enum MdpError {
    /// Indicates an error occurred while parsing the Markdown content
    ParseError(String),
    /// Indicates an error occurred during PDF file generation
    PdfError(String),
}

impl Error for MdpError {}
impl fmt::Display for MdpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MdpError::ParseError(msg) => write!(f, "[lexer] markdown parsing error: {}", msg),
            MdpError::PdfError(msg) => write!(f, "[pdf] PDF generation error: {}", msg),
        }
    }
}

/// Transforms Markdown content into a styled PDF document. The function orchestrates the entire
/// conversion pipeline, from parsing the input text through generating the final PDF file.
///
/// The process begins by parsing the Markdown content into a structured token representation.
/// It then applies styling rules, either from a configuration file if present or using defaults.
/// Finally, it generates the PDF document with the appropriate styling and structure.
///
/// # Arguments
/// * `markdown` - The Markdown content to convert
/// * `path` - Where to save the generated PDF file
///
/// # Returns
/// * `Ok(())` on successful conversion
/// * `Err(MdpError)` if errors occur during parsing or PDF generation
///
/// # Example
/// ```rust
/// // Convert a Markdown file to PDF with custom styling
/// let markdown = std::fs::read_to_string("input.md").unwrap();
/// let result = markdown2pdf::parse(markdown, "output.pdf");
/// if let Err(e) = result {
///     eprintln!("Conversion failed: {}", e);
/// }
/// ```
pub fn parse(markdown: String, path: &str) -> Result<(), MdpError> {
    let mut lexer = Lexer::new(markdown);
    let tokens = lexer
        .parse()
        .map_err(|e| MdpError::ParseError(format!("Failed to parse markdown: {:?}", e)))?;

    let style = config::load_config();
    let pdf = Pdf::new(tokens, style);
    let document = pdf.render_into_document();

    if let Some(err) = Pdf::render(document, path) {
        return Err(MdpError::PdfError(err));
    }

    Ok(())
}
