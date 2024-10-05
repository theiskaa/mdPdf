pub mod markdown;
pub mod pdf;
pub mod styling;

use markdown::*;
use pdf::Pdf;
use styling::StyleMatch;

pub fn parse(markdown: String, path: &str) {
    let mut lexer = Lexer::new(markdown);
    let tokens = lexer.parse().expect("Failed to parse your markdown");
    let parser = Pdf::new(tokens);
    let document = parser.create_document(StyleMatch::default());
    Pdf::render(document, path);
}
