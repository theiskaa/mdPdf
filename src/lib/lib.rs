pub mod markdown_lexer;

use markdown_lexer::*;

pub fn parse(markdown: String) {
    let mut lexer = Lexer::new(markdown);
    match lexer.parse() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("Error while parsing: {:?}", e);
        }
    }
}


