#[derive(Debug, PartialEq)]
pub enum Token {
    Heading(Vec<Token>, usize), // (content, level)
    Emphasis(Vec<Token>),
    StrongEmphasis(Vec<Token>),
    Code(String),
    BlockQuote(String),
    ListItem(Vec<Token>),
    Link(String, String),  // (text, url)
    Image(String, String), // (alt text, url)
    Text(String),
    HtmlComment(String),
    Newline,
    HorizontalRule,
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

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
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

    fn parse_nested_content(&mut self, delimiter: Option<char>) -> Result<Vec<Token>, LexerError> {
        let mut nested_tokens = Vec::new();
        while self.position < self.input.len() {
            if let Some(delim) = delimiter {
                if self.current_char() == delim {
                    break;
                }
            }
            if let Some(token) = self.next_token()? {
                nested_tokens.push(token);
            }
        }
        Ok(nested_tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let current_char = self.current_char();

        let token = match current_char {
            '#' => self.parse_heading()?,
            '*' | '_' => self.parse_emphasis()?,
            '`' => self.parse_code()?,
            '>' => self.parse_blockquote()?,
            '-' | '+' => self.parse_list_item_or_horizontal_rule()?,
            '[' => self.parse_link()?,
            '!' => self.parse_image()?,
            '<' => {
                if self.is_html_comment_start() {
                    self.parse_html_comment()?
                } else {
                    self.parse_text()?
                }
            }
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
        let content = self.parse_nested_content(Some('\n'))?;
        Ok(Token::Heading(content, level))
    }

    // TODO: Refactor emphasis parsing. Currently, the delimiter is passed as a single character '*',
    // leading to misinterpretation when parsing nested tokens. This also prevents the correct handling
    // of nested emphasis within emphasis tokens.
    fn parse_emphasis(&mut self) -> Result<Token, LexerError> {
        let delimiter = self.current_char();
        self.advance();
        if self.current_char() == delimiter {
            self.advance();
            let content = self.parse_nested_content(Some(delimiter))?;
            self.advance();
            Ok(Token::StrongEmphasis(content))
        } else {
            let content = self.parse_nested_content(Some(delimiter))?;
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

        let content = self.parse_nested_content(Some('\n'))?;
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

    fn parse_html_comment(&mut self) -> Result<Token, LexerError> {
        self.position += 4; // Skip past '<!--'
        let start = self.position;

        while self.position < self.input.len() && !self.input[self.position..].starts_with("-->") {
            self.advance();
        }

        if self.position < self.input.len() {
            let comment = self.input[start..self.position].to_string();
            self.position += 3; // Skip past '-->'
            Ok(Token::HtmlComment(comment))
        } else {
            Err(LexerError::UnexpectedEndOfInput)
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.current_char().is_whitespace()
            && self.current_char() != '\n'
        {
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

    fn is_html_comment_start(&self) -> bool {
        self.input[self.position..].starts_with("<!--")
    }

    fn is_special_char(&self, ch: char) -> bool {
        matches!(
            ch,
            '#' | '*' | '_' | '`' | '>' | '-' | '[' | '!' | '\n' | '+' | ')'
        )
    }
}
