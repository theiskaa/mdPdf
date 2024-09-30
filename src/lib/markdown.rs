#[derive(Debug, PartialEq)]
pub enum Token {
    Heading(Vec<Token>, usize), // (content, level)
    Emphasis { level: usize, content: Vec<Token> },
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
    input: Vec<char>,
    position: usize,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
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

    fn parse_nested_content<F>(&mut self, is_delimiter: F) -> Result<Vec<Token>, LexerError>
    where
        F: Fn(char) -> bool,
    {
        let mut content = Vec::new();
        while self.position < self.input.len() && !is_delimiter(self.current_char()) {
            if let Some(token) = self.next_token()? {
                content.push(token);
            }
        }
        Ok(content)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let current_char = self.current_char();
        let is_line_start = self.is_at_line_start();

        let token = match current_char {
            '#' if is_line_start => self.parse_heading()?,
            '*' | '_' => self.parse_emphasis()?,
            '`' => self.parse_code()?,
            '>' if is_line_start => self.parse_blockquote()?,
            '-' | '+' if is_line_start => self.parse_list_item_or_horizontal_rule()?,
            '[' => self.parse_link()?,
            '!' => self.parse_image()?,
            '<' if self.is_html_comment_start() => self.parse_html_comment()?,
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
        let content = self.parse_nested_content(|c| c == '\n')?;
        Ok(Token::Heading(content, level))
    }

    fn parse_emphasis(&mut self) -> Result<Token, LexerError> {
        let start_pos = self.position;
        let delimiter = self.current_char();
        let mut level = 0;

        // Count the number of delimiters
        while self.current_char() == delimiter {
            level += 1;
            self.advance();
        }

        let content = self.parse_nested_content(|c| c == delimiter)?;

        // Ensure proper closing
        for _ in 0..level {
            if self.current_char() != delimiter {
                return Err(LexerError::UnknownToken(format!(
                    "Unmatched emphasis at position {}",
                    start_pos
                )));
            }
            self.advance();
        }

        Ok(Token::Emphasis {
            level: level.min(3), // Cap the level at 3
            content,
        })
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

        let content = self.parse_nested_content(|c| c == '\n')?;
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
            return Ok(Token::Link(text, url));
        }
        Ok(Token::Link(text, String::new()))
    }

    fn parse_image(&mut self) -> Result<Token, LexerError> {
        self.advance();
        if self.current_char() == '[' {
            self.advance();
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
        let mut content = String::new();
        let start_pos = self.position;

        while self.position < self.input.len() {
            let ch = self.current_char();

            if ch == '\n' || self.is_start_of_special_token() {
                break;
            }

            content.push(ch);
            self.advance();
        }

        if content.is_empty() {
            Err(LexerError::UnknownToken(format!(
                "Unexpected character at position {}",
                start_pos
            )))
        } else {
            Ok(Token::Text(content))
        }
    }

    fn parse_html_comment(&mut self) -> Result<Token, LexerError> {
        self.position += 4; // Skip past '<', '!', '-', '-'
        let start = self.position;

        while self.position + 2 < self.input.len() {
            if self.input[self.position] == '-'
                && self.input[self.position + 1] == '-'
                && self.input[self.position + 2] == '>'
            {
                break;
            }
            self.advance();
        }

        if self.position + 2 < self.input.len() {
            let comment: String = self.input[start..self.position].iter().collect();
            self.position += 3; // Skip past '-', '-', '>'
            Ok(Token::HtmlComment(comment))
        } else {
            Err(LexerError::UnexpectedEndOfInput)
        }
    }

    fn is_at_line_start(&self) -> bool {
        self.position == 0 || self.input.get(self.position - 1) == Some(&'\n')
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
        *self.input.get(self.position).unwrap_or(&'\0')
    }

    fn peek_char(&self) -> char {
        *self.input.get(self.position + 1).unwrap_or(&'\0')
    }

    fn read_until_newline(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '\n' {
            self.advance();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_until_char(&mut self, delimiter: char) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != delimiter {
            self.advance();
        }
        self.input[start..self.position].iter().collect()
    }

    fn is_html_comment_start(&self) -> bool {
        self.input[self.position..]
            .iter()
            .collect::<String>()
            .starts_with("<!--")
    }

    fn is_start_of_special_token(&self) -> bool {
        let ch = self.current_char();
        match ch {
            '#' | '*' | '_' | '`' | '[' | '!' => true,
            '<' => self.is_html_comment_start(),
            _ => false,
        }
    }
}
