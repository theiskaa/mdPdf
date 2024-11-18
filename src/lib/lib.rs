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
//! # Examples
//!
//! Basic markdown-to-pdf conversion:
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
//! Converting a file with custom styling:
//! ```rust
//! use markdown2pdf;
//! use std::fs;
//!
//! // Read markdown file
//! let markdown = fs::read_to_string("input.md").unwrap();
//!
//! // Create custom styling config (markdown2pdfrc.toml):
//! // [heading.1]
//! // size = 24
//! // textcolor = { r = 0, g = 0, b = 255 }
//! // bold = true
//!
//! // Convert with custom styling
//! markdown2pdf::parse(markdown, "styled-output.pdf").unwrap();
//! ```
//!
//! Processing markdown with images and links:
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
//! # Configuration
//! Styling can be customized through a TOML configuration file (`markdown2pdfrc.toml`).
//!
//! ## Page Layout
//! ```toml
//! [page]
//! margins = { top = 72, right = 72, bottom = 72, left = 72 }
//! size = "a4"
//! orientation = "portrait"
//! ```
//!
//! ## Element Styling
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
//! # Processing Pipeline
//! ```text
//! Input                  Processing                   Output
//! -------------          ----------------             ----------------
//! Markdown Text    -->   Lexical Analysis       -->   Token Stream
//!                        (markdown::Lexer)            (Token enum)
//!
//! Token Stream    -->    Style Application      -->   PDF Elements
//!                        (styling::StyleMatch)        (genpdfi)
//!
//! PDF Elements    -->    PDF Generation         -->   Final PDF
//!                        (pdf::Pdf)                   Document
//! ```
//!
//! ## Token Processing Flow
//! ```text
//! +-------------+     +----------------+     +----------------+
//! |  Markdown   |     |  Tokens        |     |  PDF Elements  |
//! |  Input      | --> |  # -> Heading  | --> |  - Styled      |
//! |  # Title    |     |  * -> List     |     |    Heading     |
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
//!
//! # Library Structure
//! - `config`: Handles styling configuration and TOML parsing
//! - `markdown`: Implements the Markdown lexer and token parsing
//! - `pdf`: Manages PDF document generation
//! - `styling`: Defines styling types and defaults

pub mod assets;
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
/// 2. Loads styling configuration from `markdown2pdfrc.toml` if available
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

    let parser = Pdf::new(tokens);
    let style = config::load_config();
    let document = parser.create_document(style);

    if let Some(err) = Pdf::render(document, path) {
        return Err(MdpError::PdfError(err));
    }

    Ok(())
}
