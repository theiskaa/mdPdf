use crate::Token;
use genpdf::elements::Paragraph;
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

#[allow(dead_code)]
pub struct Pdf {
    input: Vec<Token>,
}

impl Pdf {
    pub fn new(input: Vec<Token>) -> Self {
        Self { input }
    }

    pub fn create_document(self) -> genpdf::Document {
        let font_family = genpdf::fonts::from_files(
            "assets/fonts",
            MdPdfFont::ITCAvantGardeGothicStdMedium.name(),
            None,
        )
        .expect("Failed to load font family");

        let mut doc = genpdf::Document::new(font_family);
        doc.set_title("Generated Markdown PDF");

        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);

        // Add elements from tokens
        for token in &self.input {
            self.render_token(&mut doc, token);
        }

        return doc;
    }

    fn render_token(&self, doc: &mut genpdf::Document, token: &Token) {
        match token {
            Token::Heading(content, level) => {
                let size = match level {
                    1 => 20,
                    2 => 18,
                    _ => 16,
                };
                let style = Style::new().bold().with_font_size(size);
                let paragraph = Paragraph::default();
                self.render_nested_content(paragraph, content, style, doc);
            }
            Token::Emphasis { level, content } => {
                let style = match level {
                    1 => Style::new().italic(),
                    2 => Style::new().bold().italic(),
                    _ => Style::new(),
                };
                let paragraph = Paragraph::default();
                self.render_nested_content(paragraph, content, style, doc);
            }
            Token::StrongEmphasis(content) => {
                let style = Style::new().bold();
                let paragraph = Paragraph::default();
                self.render_nested_content(paragraph, content, style, doc);
            }
            Token::Code(content) => {
                let style = Style::new().italic().with_font_size(12);
                let mut paragraph = Paragraph::new("");
                paragraph.push_styled(content, style);
                doc.push(paragraph);
            }
            Token::BlockQuote(content) => {
                let style = Style::new().italic().with_color(Color::Rgb(128, 128, 128));
                let mut paragraph = Paragraph::new("");
                paragraph.push_styled(content, style);
                doc.push(paragraph);
            }
            Token::ListItem(content) => {
                let style = Style::new();
                let paragraph = Paragraph::default();
                self.render_nested_content(paragraph, content, style, doc);
            }
            Token::Link(text, url) => {
                let style = Style::new().with_color(Color::Rgb(0, 0, 255));
                let content = format!("{} ({})", text, url);
                let mut paragraph = Paragraph::new("");
                paragraph.push_styled(content, style);
                doc.push(paragraph);
            }
            Token::Image(alt_text, url) => {
                let style = Style::new().italic();
                let content = format!("Image: {} [{}]", alt_text, url);
                let mut paragraph = Paragraph::new("");
                paragraph.push_styled(content, style);
                doc.push(paragraph);
            }
            Token::Text(content) => {
                let paragraph = Paragraph::new(content.clone());
                doc.push(paragraph);
            }
            Token::HtmlComment(_) => {
                // Skip rendering HTML comments in the PDF
            }
            Token::Newline => {
                doc.push(Paragraph::new(""));
            }
            Token::HorizontalRule => {
                doc.push(Paragraph::new("----------------------"));
            }
            Token::Unknown(content) => {
                let style = Style::new().italic();
                let mut paragraph = Paragraph::new("");
                paragraph.push_styled(format!("Unknown token: {}", content), style);
                doc.push(paragraph);
            }
        }
    }

    fn render_nested_content(
        &self,
        mut paragraph: Paragraph,
        content: &Vec<Token>,
        style: Style,
        doc: &mut genpdf::Document,
    ) {
        for nested_token in content {
            match nested_token {
                Token::Text(text) => {
                    paragraph.push_styled(text, style.clone());
                }
                _ => {
                    self.render_token(doc, nested_token);
                }
            }
        }
        doc.push(paragraph);
    }

    pub fn render(document: genpdf::Document, file: &str) {
        match document.render_to_file(file) {
            Ok(_) => {
                println!("Successfully saved your pdf to {}", file);
            }
            Err(err) => {
                println!("Failed to save file to {}: {}", file, err);
            }
        }
    }
}
