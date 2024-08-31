pub mod markdown;

use markdown::*;

pub fn parse(markdown: String) {
    let mut lexer = Lexer::new(markdown);
    let tokens = lexer.parse();
    match tokens {
        Ok(v) => {
            for token in v.iter() {
                println!("{:?}", token);
            }
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}


