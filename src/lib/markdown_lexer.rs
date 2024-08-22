#[derive(Debug, PartialEq)]
pub enum Token {
    Heading(String, usize),  // (content, level)
    Paragraph(String),
    Emphasis(String),
    StrongEmphasis(String),
    Code(String),
    BlockQuote(String),
    ListItem(String),
    HorizontalRule,
    Link(String, String),   // (text, url)
    Image(String, String),  // (alt text, url)
    Text(String),
    Newline,
    Unknown(String),
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedEndOfInput,
    UnknownToken(String),
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let current_char = self.current_char();

        let token = match current_char {
            '#' => self.parse_heading()?,
            '*' | '_' => self.parse_emphasis_or_strong_emphasis()?,
            '`' => self.parse_code()?,
            '>' => self.parse_blockquote()?,
            '-' | '+' => self.parse_list_item_or_horizontal_rule()?,  // '*' removed from here
            '[' => self.parse_link()?,
            '!' => self.parse_image()?,
            '\n' => self.parse_newline()?,
            _ => self.parse_text()?,
        };

        Ok(Some(token))
    }

    fn parse_heading(&mut self) -> Result<Token, LexerError> {
        let mut level = 0;
        while self.current_char() == '#' {
            level += 1;
            self.advance();
        }
        self.skip_whitespace();
        let content = self.read_until_newline();
        Ok(Token::Heading(content, level))
    }

    fn parse_emphasis_or_strong_emphasis(&mut self) -> Result<Token, LexerError> {
        let delimiter = self.current_char();
        self.advance();
        if self.current_char() == delimiter {
            self.advance();
            let content = self.read_until_char(delimiter);
            self.advance();
            Ok(Token::StrongEmphasis(content))
        } else {
            let content = self.read_until_char(delimiter);
            self.advance();
            Ok(Token::Emphasis(content))
        }
    }

    fn parse_code(&mut self) -> Result<Token, LexerError> {
        self.advance();
        let content = self.read_until_char('`');
        self.advance();
        Ok(Token::Code(content))
    }

    fn parse_blockquote(&mut self) -> Result<Token, LexerError> {
        self.advance();
        self.skip_whitespace();
        let content = self.read_until_newline();
        Ok(Token::BlockQuote(content))
    }

    fn parse_list_item_or_horizontal_rule(&mut self) -> Result<Token, LexerError> {
        self.advance();
        self.skip_whitespace();

        if self.current_char() == '-' && self.peek_char() == '-' {
            self.advance();
            self.advance();
            return Ok(Token::HorizontalRule);
        }

        let content = self.read_until_newline();
        Ok(Token::ListItem(content))
    }

    fn parse_link(&mut self) -> Result<Token, LexerError> {
        self.advance(); // skip '['
        let text = self.read_until_char(']');
        self.advance(); // skip ']'
        if self.current_char() == '(' {
            self.advance(); // skip '('
            let url = self.read_until_char(')');
            self.advance(); // skip ')'
            Ok(Token::Link(text, url))
        } else {
            Err(LexerError::UnknownToken(text))
        }
    }

    fn parse_image(&mut self) -> Result<Token, LexerError> {
        self.advance(); // skip '!'
        if self.current_char() == '[' {
            self.advance(); // skip '['
            let alt_text = self.read_until_char(']');
            self.advance(); // skip ']'
            if self.current_char() == '(' {
                self.advance(); // skip '('
                let url = self.read_until_char(')');
                self.advance(); // skip ')'
                Ok(Token::Image(alt_text, url))
            } else {
                Err(LexerError::UnknownToken(alt_text))
            }
        } else {
            Err(LexerError::UnknownToken("!".to_string()))
        }
    }

    fn parse_newline(&mut self) -> Result<Token, LexerError> {
        self.advance();
        Ok(Token::Newline)
    }

    fn parse_text(&mut self) -> Result<Token, LexerError> {
        let content = self.read_until_special_char_or_newline();
        if content.is_empty() {
            return Err(LexerError::UnknownToken(self.current_char().to_string()));
        }
        Ok(Token::Text(content))
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.position + 1).unwrap_or('\0')
    }

    fn read_until_newline(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '\n' {
            self.advance();
        }
        self.input[start..self.position].to_string()
    }

    fn read_until_char(&mut self, delimiter: char) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != delimiter {
            self.advance();
        }
        self.input[start..self.position].to_string()
    }

    fn read_until_special_char_or_newline(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() {
            let current_char = self.current_char();
            if self.is_special_char(current_char) || current_char == '\n' {
                break;
            }
            self.advance();
        }
        self.input[start..self.position].to_string()
    }

    fn is_special_char(&self, ch: char) -> bool {
        matches!(ch, '#' | '*' | '_' | '`' | '>' | '-' | '[' | '!' | '\n' | '+' | ')')
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_heading_parsing() {
        let input = "# Ismael Shakverdiev\n## Skills";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.parse().unwrap();

        assert_eq!(tokens[0], Token::Heading("Ismael Shakverdiev".to_string(), 1));
        assert_eq!(tokens[1], Token::Newline);
        assert_eq!(tokens[2], Token::Heading("Skills".to_string(), 2));
    }

    #[test]
    fn test_link_parsing() {
        let input = "[theiskaa.com](https://theiskaa.com) | [GitHub](https://github.com/theiskaa)";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.parse().unwrap();

        assert_eq!(tokens[0], Token::Link("theiskaa.com".to_string(), "https://theiskaa.com".to_string()));
        assert_eq!(tokens[1], Token::Text("| ".to_string()));
        assert_eq!(tokens[2], Token::Link("GitHub".to_string(), "https://github.com/theiskaa".to_string()));
    }

    #[test]
    fn test_list_item_parsing() {
        let input = "- **Actively used Languages**: Dart, Go, Rust.";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.parse().unwrap();

        assert_eq!(tokens[0], Token::ListItem("**Actively used Languages**: Dart, Go, Rust.".to_string()));
    }

    #[test]
    fn test_blockquote_parsing() {
        let input = "> This is a blockquote.";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.parse().unwrap();

        assert_eq!(tokens[0], Token::BlockQuote("This is a blockquote.".to_string()));
    }

    #[test]
    fn test_mixed_parsing() {
        let input = "# Ismael Shakverdiev\n[GitHub](https://github.com/theiskaa)\n- **Languages**: Dart, Go, Rust.";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.parse().unwrap();

        assert_eq!(tokens[0], Token::Heading("Ismael Shakverdiev".to_string(), 1));
        assert_eq!(tokens[1], Token::Newline);
        assert_eq!(tokens[2], Token::Link("GitHub".to_string(), "https://github.com/theiskaa".to_string()));
        assert_eq!(tokens[3], Token::Newline);
        assert_eq!(tokens[4], Token::ListItem("**Languages**: Dart, Go, Rust.".to_string()));
    }

    #[test]
    // TODO: this fails, because link inside emphasis returns emphasis
    fn test_nt_file_from_test_data() {
        let file_path = Path::new("src/lib/test_data/nt.md");
        let input = fs::read_to_string(file_path).expect("Failed to read test file");

        let mut lexer = Lexer::new(input);
        let tokens = lexer.parse().expect("Failed bo parse markdown");
        for token in tokens.iter() {
            println!("{:?}", token);
        }

        // Heading: # NT
        assert_eq!(tokens[0], Token::Heading("NT".to_string(), 1));
        assert_eq!(tokens[1], Token::Newline);

        // Horizontal Rule: ---
        assert_eq!(tokens[2], Token::HorizontalRule);
        assert_eq!(tokens[3], Token::Newline);
        assert_eq!(tokens[4], Token::Newline);

        // Comment: <!-- ... -->
        assert_eq!(tokens[5], Token::Newline);  // Lexer skips comments and just produces a newline
        assert_eq!(tokens[6], Token::Newline);

        // Heading: # Installation
        assert_eq!(tokens[7], Token::Heading("Installation".to_string(), 1));
        assert_eq!(tokens[8], Token::Newline);

        // Text: See the [last release](https://github.com/insolite-dev/nt/releases/latest), where you can find binary files for your ecosystem
        assert_eq!(tokens[9], Token::Text("See the ".to_string()));
        assert_eq!(tokens[10], Token::Link("last release".to_string(), "https://github.com/insolite-dev/nt/releases/latest".to_string()));
        assert_eq!(tokens[11], Token::Text(", where you can find binary files for your ecosystem".to_string()));
        assert_eq!(tokens[12], Token::Newline);
        assert_eq!(tokens[13], Token::Newline);

        // Heading: ### Brew:
        assert_eq!(tokens[14], Token::Heading("Brew:".to_string(), 3));
        assert_eq!(tokens[15], Token::Newline);

        // Code Block: brew install --build-from-source nt
        assert_eq!(tokens[16], Token::Code("brew install --build-from-source nt".to_string()));
        assert_eq!(tokens[17], Token::Newline);
        assert_eq!(tokens[18], Token::Newline);

        // Heading: ### Curl:
        assert_eq!(tokens[19], Token::Heading("Curl:".to_string(), 3));
        assert_eq!(tokens[20], Token::Newline);

        // Code Block: curl -sfL https://raw.githubusercontent.com/insolite-dev/nt/main/install.sh | sh
        assert_eq!(tokens[21], Token::Code("curl -sfL https://raw.githubusercontent.com/insolite-dev/nt/main/install.sh | sh".to_string()));
        assert_eq!(tokens[22], Token::Newline);
        assert_eq!(tokens[23], Token::Newline);

        // Heading: # Usage
        assert_eq!(tokens[24], Token::Heading("Usage".to_string(), 1));
        assert_eq!(tokens[25], Token::Newline);

        // Heading: ### Help:
        assert_eq!(tokens[26], Token::Heading("Help:".to_string(), 3));
        assert_eq!(tokens[27], Token::Newline);

        // Text: Run `nt help` or `nt -h` to see default [help.txt](https://github.com/insolite-dev/nt/wiki/help.txt). <br>
        assert_eq!(tokens[28], Token::Text("Run ".to_string()));
        assert_eq!(tokens[29], Token::Code("nt help".to_string()));
        assert_eq!(tokens[30], Token::Text(" or ".to_string()));
        assert_eq!(tokens[31], Token::Code("nt -h".to_string()));
        assert_eq!(tokens[32], Token::Text(" to see default ".to_string()));
        assert_eq!(tokens[33], Token::Link("help.txt".to_string(), "https://github.com/insolite-dev/nt/wiki/help.txt".to_string()));
        assert_eq!(tokens[34], Token::Text(". ".to_string()));
        assert_eq!(tokens[35], Token::Newline);

        // Text: Use `nt [command] --help` for more information about a command.
        assert_eq!(tokens[36], Token::Text("Use ".to_string()));
        assert_eq!(tokens[37], Token::Code("nt [command] --help".to_string()));
        assert_eq!(tokens[38], Token::Text(" for more information about a command.".to_string()));
        assert_eq!(tokens[39], Token::Newline);
        assert_eq!(tokens[40], Token::Newline);

        // Horizontal Rule: ---
        assert_eq!(tokens[41], Token::HorizontalRule);
        assert_eq!(tokens[42], Token::Newline);
        assert_eq!(tokens[43], Token::Newline);

        // Heading: ### Settings (config):
        assert_eq!(tokens[44], Token::Heading("Settings (config):".to_string(), 3));
        assert_eq!(tokens[45], Token::Newline);

        // Text: The configuration file of **nt** is auto-generated...
        assert_eq!(tokens[46], Token::Text("The configuration file of ".to_string()));
        assert_eq!(tokens[47], Token::StrongEmphasis("nt".to_string()));
        assert_eq!(tokens[48], Token::Text(" is auto-generated, it'd be generated by ".to_string()));
        assert_eq!(tokens[49], Token::Code("Init".to_string()));
        assert_eq!(tokens[50], Token::Text(" command automatically whenever you run ".to_string()));
        assert_eq!(tokens[51], Token::StrongEmphasis("nt".to_string()));
        assert_eq!(tokens[52], Token::Text(" on your command line. ".to_string()));
        assert_eq!(tokens[53], Token::Newline);

        // Text: **Refer to settings documentation for details - [Settings Wiki](https://github.com/insolite-dev/nt/wiki/Settings)**
        assert_eq!(tokens[54], Token::StrongEmphasis("Refer to settings documentation for details - ".to_string()));
        assert_eq!(tokens[55], Token::Link("Settings Wiki".to_string(), "https://github.com/insolite-dev/nt/wiki/Settings".to_string()));
        assert_eq!(tokens[56], Token::Newline);
        assert_eq!(tokens[57], Token::Newline);

        // Horizontal Rule: ---
        assert_eq!(tokens[58], Token::HorizontalRule);
        assert_eq!(tokens[59], Token::Newline);
        assert_eq!(tokens[60], Token::Newline);

        // Heading: ### Remote service integration:
        assert_eq!(tokens[61], Token::Heading("Remote service integration:".to_string(), 3));
        assert_eq!(tokens[62], Token::Newline);

        // Text: Currently available remote service is only Firebase...
        assert_eq!(tokens[63], Token::Text("Currently available remote service is only Firebase, looking forward to provide another ways of decentralizing remote connections. ".to_string()));
        assert_eq!(tokens[64], Token::Newline);

        // Text: **Refer to [remote] command documentation for more - [Remote Wiki](https://github.com/insolite-dev/nt/wiki/Remote)**
        assert_eq!(tokens[65], Token::StrongEmphasis("Refer to ".to_string()));
        assert_eq!(tokens[66], Token::Link("[remote] command documentation".to_string(), "https://github.com/insolite-dev/nt/wiki/Remote".to_string()));
        assert_eq!(tokens[67], Token::Text(" for more - ".to_string()));
        assert_eq!(tokens[68], Token::Link("Remote Wiki".to_string(), "https://github.com/insolite-dev/nt/wiki/Remote".to_string()));
        assert_eq!(tokens[69], Token::Newline);
        assert_eq!(tokens[70], Token::Newline);

        // Horizontal Rule: ---
        assert_eq!(tokens[71], Token::HorizontalRule);
        assert_eq!(tokens[72], Token::Newline);
        assert_eq!(tokens[73], Token::Newline);

        // Heading: ### Commands:
        assert_eq!(tokens[74], Token::Heading("Commands:".to_string(), 3));
        assert_eq!(tokens[75], Token::Newline);

        // List Items
        assert_eq!(tokens[76], Token::ListItem("See all notes".to_string()));
        assert_eq!(tokens[77], Token::Link("https://github.com/insolite-dev/nt/wiki/List".to_string(), "nt list".to_string()));
        assert_eq!(tokens[78], Token::Newline);

        assert_eq!(tokens[79], Token::ListItem("View note".to_string()));
        assert_eq!(tokens[80], Token::Link("https://github.com/insolite-dev/nt/wiki/View".to_string(), "nt view or nt view [name]".to_string()));
        assert_eq!(tokens[81], Token::Newline);

        assert_eq!(tokens[82], Token::ListItem("Create node(file or folder)".to_string()));
        assert_eq!(tokens[83], Token::Link("https://github.com/insolite-dev/nt/wiki/Create".to_string(), "nt create or nt create [title]".to_string()));
        assert_eq!(tokens[84], Token::Newline);

        assert_eq!(tokens[85], Token::ListItem("Make a directory".to_string()));
        assert_eq!(tokens[86], Token::Link("https://github.com/insolite-dev/nt/wiki/Mkdir".to_string(), "nt mkdir or nt md [name]".to_string()));
        assert_eq!(tokens[87], Token::Newline);

        assert_eq!(tokens[88], Token::ListItem("Rename node(file or folder)".to_string()));
        assert_eq!(tokens[89], Token::Link("https://github.com/insolite-dev/nt/wiki/Rename".to_string(), "nt rename or nt rename [name]".to_string()));
        assert_eq!(tokens[90], Token::Newline);

        assert_eq!(tokens[91], Token::ListItem("Edit note".to_string()));
        assert_eq!(tokens[92], Token::Link("https://github.com/insolite-dev/nt/wiki/Edit".to_string(), "nt edit or nt edit [name]".to_string()));
        assert_eq!(tokens[93], Token::Newline);

        assert_eq!(tokens[94], Token::ListItem("Remove node(file or folder)".to_string()));
        assert_eq!(tokens[95], Token::Link("https://github.com/insolite-dev/nt/wiki/Remove".to_string(), "nt remove or nt rm [name]".to_string()));
        assert_eq!(tokens[96], Token::Newline);

        assert_eq!(tokens[97], Token::ListItem("Copy note".to_string()));
        assert_eq!(tokens[98], Token::Link("https://github.com/insolite-dev/nt/wiki/Copy".to_string(), "nt copy".to_string()));
        assert_eq!(tokens[99], Token::Newline);

        assert_eq!(tokens[100], Token::ListItem("Cut note".to_string()));
        assert_eq!(tokens[101], Token::Link("https://github.com/insolite-dev/nt/wiki/Cut".to_string(), "nt cut".to_string()));
        assert_eq!(tokens[102], Token::Newline);

        assert_eq!(tokens[103], Token::ListItem("Fetch nodes(files and folders)".to_string()));
        assert_eq!(tokens[104], Token::Link("https://github.com/insolite-dev/nt/wiki/Fetch".to_string(), "nt fetch or nt pull".to_string()));
        assert_eq!(tokens[105], Token::Newline);

        assert_eq!(tokens[106], Token::ListItem("Push nodes(files and folders)".to_string()));
        assert_eq!(tokens[107], Token::Link("https://github.com/insolite-dev/nt/wiki/Push".to_string(), "nt push".to_string()));
        assert_eq!(tokens[108], Token::Newline);

        assert_eq!(tokens[109], Token::ListItem("Migrate Services(files and folders)".to_string()));
        assert_eq!(tokens[110], Token::Link("https://github.com/insolite-dev/nt/wiki/Migrate".to_string(), "nt migrate".to_string()));
        assert_eq!(tokens[111], Token::Newline);

        assert_eq!(tokens[112], Token::ListItem("Manage Settings".to_string()));
        assert_eq!(tokens[113], Token::Link("https://github.com/insolite-dev/nt/wiki/Settings".to_string(), "nt settings".to_string()));
        assert_eq!(tokens[114], Token::Newline);

        assert_eq!(tokens[115], Token::ListItem("Manage Remote Services".to_string()));
        assert_eq!(tokens[116], Token::Link("https://github.com/insolite-dev/nt/wiki/Remote".to_string(), "nt remote".to_string()));
        assert_eq!(tokens[117], Token::Newline);
        assert_eq!(tokens[118], Token::Newline);

        // Heading: # Contributing
        assert_eq!(tokens[119], Token::Heading("Contributing".to_string(), 1));
        assert_eq!(tokens[120], Token::Newline);

        // Text: For information regarding contributions...
        assert_eq!(tokens[121], Token::Text("For information regarding contributions, please refer to ".to_string()));
        assert_eq!(tokens[122], Token::Link("CONTRIBUTING.md".to_string(), "https://github.com/insolite-dev/nt/blob/develop/CONTRIBUTING.md".to_string()));
        assert_eq!(tokens[123], Token::Text(" file.".to_string()));
        assert_eq!(tokens[124], Token::Newline);

        // Ensure no extra tokens are present
        assert_eq!(tokens.len(), 125);
    }
}
