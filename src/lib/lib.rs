//! Core library module for markdown-to-pdf conversion with customizable styling.
//!
//! This library provides functionality to convert Markdown content into styled PDF documents.
//! It handles parsing Markdown text into tokens, applying configurable styling, and generating
//! the final PDF output.
//!
//! # Features
//! - Parse Markdown files or strings into PDF documents
//! - Customizable styling via TOML configuration files
//! - Support for common Markdown elements like headings, emphasis, lists, etc.
//! - Configurable margins, fonts, colors, and text properties
//!
//! # Configuration
//! Styling can be customized through a TOML configuration file (`mdprc.toml`).
//! The configuration supports:
//! - Margins and page layout
//! - Text styles for different Markdown elements (size, color, alignment, etc.)
//! - Font families and text decorations
//!
//! See the `config` module for detailed configuration options.
//!
//! # Example
//! ```rust
//! use mdp;
//!
//! // Convert Markdown string to PDF
//! let markdown = "# Hello World\nThis is a test.".to_string();
//! if let Err(e) = mdp::parse(markdown, "output.pdf") {
//!     eprintln!("Failed to generate PDF: {}", e);
//! }
//! ```
//!
//! # Library Structure
//! - `config`: Handles styling configuration and TOML parsing
//! - `markdown`: Implements the Markdown lexer and token parsing
//! - `pdf`: Manages PDF document generation
//! - `styling`: Defines styling types and defaults

pub mod config;
pub mod markdown;
pub mod pdf;
pub mod styling;

use markdown::*;
use pdf::Pdf;
use std::error::Error;
use std::fmt;

/// Custom error type for markdown-to-pdf conversion errors
#[derive(Debug)]
pub enum MdpError {
    /// Error during Markdown parsing
    ParseError(String),
    /// Error during PDF generation
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

/// Converts Markdown content to a styled PDF document.
///
/// This function handles the complete conversion process:
/// 1. Parses the Markdown content into tokens
/// 2. Loads styling configuration from `mdprc.toml` if available
/// 3. Generates a PDF document with the configured styling
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
/// // Convert a Markdown file to PDF with custom styling
/// let markdown = std::fs::read_to_string("input.md").unwrap();
/// let result = mdp::parse(markdown, "output.pdf");
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
    let style = config::load_config();
    let document = parser.create_document(style);

    // Render PDF to file
    Pdf::render(document, path);
    Ok(())
}
