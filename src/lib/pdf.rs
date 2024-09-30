use crate::Token;
use genpdf::elements::{Paragraph, UnorderedList};
use genpdf::fonts::{FontData, FontFamily};
use genpdf::style::{Color, Style};

#[derive(Debug, PartialEq)]
pub enum MdPdfFont {
    ITCAvantGardeGothicStdMedium,
}

impl MdPdfFont {
    pub fn name(&self) -> &'static str {
        match self {
            MdPdfFont::ITCAvantGardeGothicStdMedium => "ITC-Avant-Garde-Gothic-Std-Medium",
        }
    }
}

#[derive(Debug)]
enum Block {
    Heading(Vec<Token>, usize),
    Paragraph(Vec<Token>),
    List(Vec<Vec<Token>>), // List of list items
    BlockQuote(Vec<Token>),
    HorizontalRule,
    EmptyLine,
}

pub struct Pdf {
    input: Vec<Token>,
}

impl Pdf {
    pub fn new(input: Vec<Token>) -> Self {
        Self { input }
    }

    pub fn render(document: genpdf::Document, file: &str) {
        match document.render_to_file(file) {
            Ok(_) => {
                println!("Successfully saved your PDF to {}", file);
            }
            Err(err) => {
                println!("Failed to save file to {}: {}", file, err);
            }
        }
    }

    pub fn create_document(self) -> genpdf::Document {
        // Load your custom font
        let font_family = genpdf::fonts::from_files(
            "assets/fonts",
            MdPdfFont::ITCAvantGardeGothicStdMedium.name(),
            None,
        )
        .expect("Failed to load font family");

        // TODO: Load a monospace font for code blocks
        let code_font_family = genpdf::fonts::from_files("assets/fonts", "monospace", None)
            .unwrap_or_else(|_| font_family.clone()); // Fallback to main font if not available

        let mut doc = genpdf::Document::new(font_family.clone());
        doc.set_title("Generated Markdown PDF");

        // Set document decorator and margins
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);

        // Set default font size
        doc.set_font_size(10);

        // Process tokens into blocks
        let blocks = self.group_tokens(self.input.clone());

        // Render each block
        for block in blocks {
            match block {
                Block::Heading(content, level) => {
                    let size = match level {
                        1 => 17,
                        2 => 15,
                        3 => 13,
                        _ => 11,
                    };
                    let style = Style::new().bold().with_font_size(size);
                    let paragraph =
                        self.process_inline_tokens(content, style, &font_family, &code_font_family);
                    doc.push(paragraph);
                    // Add spacing after heading
                    doc.push(genpdf::elements::Break::new(0.5));
                }
                Block::Paragraph(content) => {
                    let paragraph = self.process_inline_tokens(
                        content,
                        Style::new(),
                        &font_family,
                        &code_font_family,
                    );
                    doc.push(paragraph);
                    // No extra spacing after paragraph
                }
                Block::List(items) => {
                    let mut list = UnorderedList::new();
                    for item_tokens in items {
                        let item_paragraph = self.process_inline_tokens(
                            item_tokens,
                            Style::new(),
                            &font_family,
                            &code_font_family,
                        );
                        list.push(item_paragraph);
                    }
                    doc.push(list);
                    // Add spacing after list
                    doc.push(genpdf::elements::Break::new(0.5));
                }
                Block::BlockQuote(content) => {
                    let style = Style::new().italic().with_color(Color::Rgb(128, 128, 128));
                    let paragraph =
                        self.process_inline_tokens(content, style, &font_family, &code_font_family);
                    doc.push(paragraph);
                    // Add spacing after blockquote
                    doc.push(genpdf::elements::Break::new(0.5));
                }
                Block::HorizontalRule => {
                    // Implement horizontal rule if needed
                    // For now, add spacing
                    doc.push(genpdf::elements::Break::new(0.5));
                }
                Block::EmptyLine => {
                    // Add spacing equivalent to one line
                    doc.push(genpdf::elements::Break::new(1.0));
                }
            }
        }

        doc
    }

    // Function to group tokens into blocks
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
                    // Process the blockquote content as inline tokens
                    let content_tokens = vec![Token::Text(content.clone())];
                    blocks.push(Block::BlockQuote(content_tokens));
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
                        // Single newline within inline content
                        current_inline_content.push(Token::Newline);
                    }
                }
                Token::Text(_)
                | Token::Emphasis { .. }
                | Token::StrongEmphasis(_)
                | Token::Code(_)
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

        // Flush any remaining inline content as a paragraph
        if !current_inline_content.is_empty() {
            blocks.push(Block::Paragraph(current_inline_content));
        }

        blocks
    }

    // Function to process inline tokens into a paragraph
    fn process_inline_tokens(
        &self,
        tokens: Vec<Token>,
        style: Style,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
    ) -> Paragraph {
        let mut paragraph = Paragraph::default();
        self.render_inline_tokens(&mut paragraph, tokens, style, font_family, code_font_family);
        paragraph
    }

    // Function to render inline tokens within a paragraph
    fn render_inline_tokens(
        &self,
        paragraph: &mut Paragraph,
        tokens: Vec<Token>,
        style: Style,
        font_family: &FontFamily<FontData>,
        code_font_family: &FontFamily<FontData>,
    ) {
        for token in tokens {
            match token {
                Token::Text(content) => {
                    paragraph.push_styled(content, style.clone());
                }
                Token::Emphasis { level, content } => {
                    let mut nested_style = style.clone();
                    match level {
                        1 => {
                            nested_style = nested_style.italic();
                        }
                        2 => {
                            nested_style = nested_style.bold();
                        }
                        3 | _ => {
                            nested_style = nested_style.bold().italic();
                        }
                    }
                    self.render_inline_tokens(
                        paragraph,
                        content,
                        nested_style,
                        font_family,
                        code_font_family,
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
                    );
                }
                Token::Link(text, _url) => {
                    let link_style = style.clone().with_color(Color::Rgb(0, 0, 255));
                    paragraph.push_styled(text, link_style);
                }
                Token::Code(content) => {
                    let code_style = style.clone().with_color(Color::Rgb(0, 0, 255));
                    paragraph.push_styled(content, code_style);
                }
                Token::Newline => {
                    // TODO: handle new line in paragraph.
                    // paragraph.push(genpdf::elements::Break::new());
                }
                _ => {}
            }
        }
    }
}
