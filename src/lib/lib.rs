//! Core library module for markdown-to-pdf conversion.
//!
//! This module provides the main functionality for converting Markdown content into PDF documents.
//! It handles parsing Markdown text into tokens, applying styling, and generating the final PDF output.
//!
//! # Example
//! ```rust
//! use mdp;
//!
//! let markdown = "# Hello World\nThis is a test.".to_string();
//! if let Err(e) = mdp::parse(markdown, "output.pdf") {
//!     eprintln!("Failed to generate PDF: {}", e);
//! }
//! ```

pub mod markdown;
pub mod pdf;
pub mod styling;

use std::error::Error;
use std::fmt;

use markdown::*;
use pdf::Pdf;
use styling::StyleMatch;

/// Custom error type for markdown-to-pdf conversion errors
#[derive(Debug)]
pub enum MdpError {
    /// Error during Markdown parsing
    ParseError(String),
    /// Error during PDF generation
    PdfError(String),
}

impl fmt::Display for MdpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MdpError::ParseError(msg) => write!(f, "[lexer] markdown parsing error: {}", msg),
            MdpError::PdfError(msg) => write!(f, "[pdf] PDF generation error: {}", msg),
        }
    }
}

impl Error for MdpError {}

/// Converts Markdown content to a PDF document.
///
/// # Arguments
/// * `markdown` - The Markdown content as a string
/// * `path` - Output path for the generated PDF file
///
/// # Returns
/// * `Ok(())` if conversion succeeds
/// * `Err(MdpError)` if parsing or PDF generation fails
///
/// # Example
/// ```rust
/// let result = mdp::parse("# Hello".to_string(), "output.pdf");
/// if let Err(e) = result {
///     eprintln!("Conversion failed: {}", e);
/// }
/// ```
pub fn parse(markdown: String, path: &str) -> Result<(), MdpError> {
    // Create lexer and parse markdown tokens
    let mut lexer = Lexer::new(markdown);
    let tokens = lexer
        .parse()
        .map_err(|e| MdpError::ParseError(format!("Failed to parse markdown: {:?}", e)))?;

    // Create PDF parser with default styling
    let parser = Pdf::new(tokens);
    let document = parser.create_document(StyleMatch::default());

    // Render PDF to file
    Pdf::render(document, path);
    Ok(())
}
