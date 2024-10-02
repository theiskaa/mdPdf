pub mod markdown;
pub mod pdf;

use markdown::*;
use pdf::Pdf;

pub fn parse(markdown: String, path: &str) {
    let mut lexer = Lexer::new(markdown);
    let tokens = lexer.parse().expect("Failed to parse your markdown");
    let parser = Pdf::new(tokens);
    let document = parser.create_document();
    Pdf::render(document, path);
}
