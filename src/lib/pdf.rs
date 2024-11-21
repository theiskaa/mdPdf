//! PDF generation module for markdown-to-pdf conversion.
//!
//! This module handles the complete process of converting parsed markdown content into professionally formatted PDF documents.
//! It provides robust support for generating PDFs with proper typography, layout, and styling while maintaining the semantic
//! structure of the original markdown.
//!
//! The PDF generation process preserves the hierarchical document structure through careful handling of block-level and inline
//! elements. Block elements like headings, paragraphs, lists and code blocks are rendered with appropriate spacing and indentation.
//! Inline formatting such as emphasis, links and inline code maintain proper nesting and style inheritance.
//!
//! The styling system offers extensive customization options through a flexible configuration model. This includes control over:
//! fonts, text sizes, colors, margins, spacing, and special styling for different content types. The module automatically handles
//! font loading, page layout, and proper rendering of all markdown elements while respecting the configured styles.
//!
//! Error handling is built in throughout the generation process to provide meaningful feedback if issues occur during PDF creation.
//! The module is designed to be both robust for production use and flexible enough to accommodate various document structures
//! and styling needs.

use crate::styling::{MdPdfFont, StyleMatch};
use crate::Token;
use genpdfi::elements::{Paragraph, UnorderedList};
use genpdfi::fonts::{FontData, FontFamily};
use genpdfi::style::{Color, Style};
use genpdfi::Margins;

/// A block-level element representing a distinct section of content in the document structure.
/// Block elements form the fundamental building blocks of the document layout, each handling
/// a specific type of content with its own semantic meaning and visual presentation rules.
#[derive(Debug)]
#[allow(dead_code)]
enum Block {
    /// A document heading with associated content and heading level
    Heading(Vec<Token>, usize),
    /// A standard text paragraph
    Paragraph(Vec<Token>),
    /// An unordered list of content items
    List(Vec<Vec<Token>>),
    /// An indented block quote section
    BlockQuote(Vec<Token>),
    /// A formatted block of code content
    CodeBlock(String, String),
    /// A horizontal dividing line
    HorizontalRule,
    /// A vertical spacing element
    EmptyLine,
}

/// The main PDF document generator that orchestrates the conversion process from markdown to PDF.
/// This struct serves as the central coordinator for document generation, managing the overall
/// structure, styling application, and proper sequencing of content elements.
pub struct Pdf {
    input: Vec<Token>,
}

impl Pdf {
    /// Creates a new PDF generator instance prepared to process the provided markdown tokens.
    /// The generator will maintain the document structure while applying appropriate styling
    /// and layout rules during conversion.
    pub fn new(input: Vec<Token>) -> Self {
        Self { input }
    }

    /// Finalizes and outputs the processed document to a PDF file at the specified path.
    /// Provides comprehensive error handling to catch and report any issues during the
    /// final rendering phase.
    pub fn render(document: genpdfi::Document, path: &str) -> Option<String> {
        match document.render_to_file(path) {
            Ok(_) => None,
            Err(err) => Some(err.to_string()),
        }
    }

    /// Orchestrates the complete document creation process by coordinating font loading,
    /// document initialization, content processing, and styled rendering of all elements.
    pub fn create_document(self, style_match: StyleMatch) -> genpdfi::Document {
        // Initialize document and fonts
        let font_family = MdPdfFont::load_font_family(style_match.text.font_family)
            .expect("Failed to load font family");
        let code_font_family = MdPdfFont::load_font_family(style_match.code.font_family)
            .expect("Failed to load code font family");

        let mut doc = self.init_document(&font_family, &style_match);

        // Process tokens into blocks and render them
        let blocks = self.group_tokens(self.input.clone());
        self.render_blocks(
            &mut doc,
            blocks,
            &font_family,
            &code_font_family,
            &style_match,
        );

        doc
    }

    /// Establishes the foundational document configuration including page dimensions,
    /// margins, and base typographic settings that will be used throughout the document.
    fn init_document(
        &self,
        font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) -> genpdfi::Document {
        let mut doc = genpdfi::Document::new(font_family.clone());

        let mut decorator = genpdfi::SimplePageDecorator::new();
        decorator.set_margins(Margins::trbl(
            style_match.margins.top,
            style_match.margins.right,
            style_match.margins.bottom,
            style_match.margins.left,
        ));

        doc.set_page_decorator(decorator);
        doc.set_font_size(style_match.text.size);
        doc
    }

    /// Manages the rendering of all block-level elements while maintaining proper
    /// document flow and applying the appropriate styling to each element type.
    fn render_blocks(
        &self,
        doc: &mut genpdfi::Document,
        blocks: Vec<Block>,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) {
        for block in blocks {
            match block {
                Block::Heading(content, level) => {
                    self.render_heading(
                        doc,
                        content,
                        level,
                        font_family,
                        code_font_family,
                        style_match,
                    );
                }
                Block::Paragraph(content) => {
                    self.render_paragraph(doc, content, font_family, code_font_family, style_match);
                }
                Block::List(items) => {
                    self.render_list(doc, items, font_family, code_font_family, style_match);
                }
                Block::BlockQuote(content) => {
                    self.render_blockquote(
                        doc,
                        content,
                        font_family,
                        code_font_family,
                        style_match,
                    );
                }
                Block::CodeBlock(_, content) => {
                    self.render_codeblock(doc, content, style_match);
                }
                Block::HorizontalRule => {
                    doc.push(genpdfi::elements::Break::new(
                        style_match.horizontal_rule.after_spacing,
                    ));
                }
                Block::EmptyLine => {
                    doc.push(genpdfi::elements::Break::new(1.0));
                }
            }
        }
    }

    /// Handles the specialized rendering of headings with appropriate level-based styling,
    /// spacing, and visual hierarchy within the document structure.
    fn render_heading(
        &self,
        doc: &mut genpdfi::Document,
        content: Vec<Token>,
        level: usize,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) {
        let heading_style = match level {
            1 => &style_match.heading_1,
            2 => &style_match.heading_2,
            3 => &style_match.heading_3,
            _ => &style_match.text,
        };

        let mut style = Style::new().with_font_size(heading_style.size);
        if heading_style.bold {
            style = style.bold();
        }
        if heading_style.italic {
            style = style.italic();
        }
        if let Some(color) = heading_style.text_color {
            style = style.with_color(Color::Rgb(color.0, color.1, color.2));
        }

        let paragraph =
            self.process_inline_tokens(content, style, font_family, code_font_family, style_match);
        doc.push(paragraph);
        doc.push(genpdfi::elements::Break::new(heading_style.after_spacing));
    }

    /// Manages the rendering of standard paragraphs while handling inline formatting
    /// and maintaining proper text flow and spacing.
    fn render_paragraph(
        &self,
        doc: &mut genpdfi::Document,
        content: Vec<Token>,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) {
        let mut style = Style::new().with_font_size(style_match.text.size);
        if let Some(color) = style_match.text.text_color {
            style = style.with_color(Color::Rgb(color.0, color.1, color.2));
        }

        let paragraph =
            self.process_inline_tokens(content, style, font_family, code_font_family, style_match);
        doc.push(paragraph);
        doc.push(genpdfi::elements::Break::new(
            style_match.text.after_spacing,
        ));
    }

    /// Handles the specialized rendering of unordered lists including proper indentation,
    /// bullet points, and spacing between list items.
    fn render_list(
        &self,
        doc: &mut genpdfi::Document,
        items: Vec<Vec<Token>>,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) {
        let mut list = UnorderedList::new();
        for item_tokens in items {
            let mut style = Style::new().with_font_size(style_match.list_item.size);
            if let Some(color) = style_match.list_item.text_color {
                style = style.with_color(Color::Rgb(color.0, color.1, color.2));
            }
            let item_paragraph = self.process_inline_tokens(
                item_tokens,
                style,
                font_family,
                code_font_family,
                style_match,
            );
            list.push(item_paragraph);
        }
        doc.push(list);
        doc.push(genpdfi::elements::Break::new(
            style_match.list_item.after_spacing,
        ));
    }

    /// Manages the rendering of block quotes with appropriate indentation, styling,
    /// and visual distinction from regular text content.
    fn render_blockquote(
        &self,
        doc: &mut genpdfi::Document,
        content: Vec<Token>,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) {
        let mut style = Style::new().with_font_size(style_match.block_quote.size);
        if style_match.block_quote.italic {
            style = style.italic();
        }
        if let Some(color) = style_match.block_quote.text_color {
            style = style.with_color(Color::Rgb(color.0, color.1, color.2));
        }

        let paragraph =
            self.process_inline_tokens(content, style, font_family, code_font_family, style_match);
        doc.push(paragraph);
        doc.push(genpdfi::elements::Break::new(
            style_match.block_quote.after_spacing,
        ));
    }

    /// Handles the specialized rendering of code blocks with monospace fonts,
    /// proper formatting, and optional syntax highlighting.
    fn render_codeblock(
        &self,
        doc: &mut genpdfi::Document,
        content: String,
        style_match: &StyleMatch,
    ) {
        let mut code_style = Style::new().with_font_size(style_match.code.size);
        if let Some(color) = style_match.code.text_color {
            code_style = code_style.with_color(Color::Rgb(color.0, color.1, color.2));
        }

        for line in content.split('\n') {
            let mut para = Paragraph::default();
            para.push_styled(line.to_string(), code_style.clone());
            doc.push(para);
        }
        doc.push(genpdfi::elements::Break::new(
            style_match.code.after_spacing,
        ));
    }

    /// Analyzes and organizes markdown tokens into logical block-level elements
    /// while maintaining proper document structure and content relationships.
    fn group_tokens(&self, tokens: Vec<Token>) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut idx = 0;
        let len = tokens.len();
        let mut current_inline_content = Vec::new();
        let mut newline_count = 0;

        while idx < len {
            match &tokens[idx] {
                Token::Heading(content, level) => {
                    if !current_inline_content.is_empty() {
                        blocks.push(Block::Paragraph(current_inline_content.clone()));
                        current_inline_content.clear();
                    }
                    blocks.push(Block::Heading(content.clone(), *level));
                    idx += 1;
                    newline_count = 0;
                }
                Token::ListItem(content) => {
                    if !current_inline_content.is_empty() {
                        blocks.push(Block::Paragraph(current_inline_content.clone()));
                        current_inline_content.clear();
                    }
                    // Start a new list
                    let mut items = Vec::new();
                    items.push(content.clone());
                    idx += 1;

                    // Collect subsequent list items
                    while idx < len {
                        if let Token::ListItem(content) = &tokens[idx] {
                            items.push(content.clone());
                            idx += 1;
                        } else {
                            break;
                        }
                    }

                    blocks.push(Block::List(items));
                    newline_count = 0;
                }
                Token::BlockQuote(content) => {
                    if !current_inline_content.is_empty() {
                        blocks.push(Block::Paragraph(current_inline_content.clone()));
                        current_inline_content.clear();
                    }
                    let content_tokens = vec![Token::Text(content.clone())];
                    blocks.push(Block::BlockQuote(content_tokens));
                    idx += 1;
                    newline_count = 0;
                }
                Token::Code(lang, content) if content.contains('\n') => {
                    if !current_inline_content.is_empty() {
                        blocks.push(Block::Paragraph(current_inline_content.clone()));
                        current_inline_content.clear();
                    }
                    blocks.push(Block::CodeBlock(lang.clone(), content.clone()));
                    idx += 1;
                    newline_count = 0;
                }
                Token::HorizontalRule => {
                    if !current_inline_content.is_empty() {
                        blocks.push(Block::Paragraph(current_inline_content.clone()));
                        current_inline_content.clear();
                    }
                    blocks.push(Block::HorizontalRule);
                    idx += 1;
                    newline_count = 0;
                }
                Token::Newline => {
                    idx += 1;
                    newline_count += 1;

                    if newline_count >= 2 {
                        if !current_inline_content.is_empty() {
                            blocks.push(Block::Paragraph(current_inline_content.clone()));
                            current_inline_content.clear();
                        }
                        blocks.push(Block::EmptyLine);
                        newline_count = 0;
                    } else {
                        current_inline_content.push(Token::Newline);
                    }
                }
                Token::Text(_)
                | Token::Emphasis { .. }
                | Token::StrongEmphasis(_)
                | Token::Code(_, _)
                | Token::Link(_, _) => {
                    current_inline_content.push(tokens[idx].clone());
                    idx += 1;
                    newline_count = 0;
                }
                _ => {
                    idx += 1;
                    newline_count = 0;
                }
            }
        }

        if !current_inline_content.is_empty() {
            blocks.push(Block::Paragraph(current_inline_content));
        }

        blocks
    }

    /// Processes inline tokens to create properly styled paragraph elements while
    /// maintaining formatting and proper nesting of inline elements.
    fn process_inline_tokens(
        &self,
        tokens: Vec<Token>,
        style: Style,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) -> Paragraph {
        let mut paragraph = Paragraph::default();
        self.render_inline_tokens(
            &mut paragraph,
            tokens,
            style,
            font_family,
            code_font_family,
            style_match,
        );
        paragraph
    }

    /// Handles the detailed rendering of inline formatting elements while maintaining
    /// proper style inheritance and nesting of text decorations.
    fn render_inline_tokens(
        &self,
        paragraph: &mut Paragraph,
        tokens: Vec<Token>,
        style: Style,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
        style_match: &StyleMatch,
    ) {
        for token in tokens {
            match token {
                Token::Text(content) => {
                    paragraph.push_styled(content, style.clone());
                }
                Token::Emphasis { level, content } => {
                    let mut nested_style = style.clone();
                    match level {
                        1 => nested_style = nested_style.italic(),
                        2 => nested_style = nested_style.bold(),
                        _ => nested_style = nested_style.bold().italic(),
                    }
                    self.render_inline_tokens(
                        paragraph,
                        content,
                        nested_style,
                        font_family,
                        code_font_family,
                        style_match,
                    );
                }
                Token::StrongEmphasis(content) => {
                    let nested_style = style.clone().bold();
                    self.render_inline_tokens(
                        paragraph,
                        content,
                        nested_style,
                        font_family,
                        code_font_family,
                        style_match,
                    );
                }
                Token::Link(text, url) => {
                    let mut link_style = style.clone();
                    if let Some(color) = style_match.link.text_color {
                        link_style = link_style.with_color(Color::Rgb(color.0, color.1, color.2));
                    }
                    paragraph.push_link(format!("{} ", text), url, link_style);
                }
                Token::Code(_language, content) => {
                    let mut code_style = style.clone();
                    if let Some(color) = style_match.code.text_color {
                        code_style = code_style.with_color(Color::Rgb(color.0, color.1, color.2));
                    }
                    paragraph.push_styled(content, code_style);
                }
                Token::Newline => {}
                _ => {}
            }
        }
    }
}
