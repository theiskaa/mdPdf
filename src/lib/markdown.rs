/// Represents the different types of tokens that can be parsed from Markdown text.
///
/// # Examples
/// ```rust
/// use mdp::Token;
///
/// // Heading token with nested content
/// let heading = Token::Heading(vec![Token::Text("Title".to_string())], 1);
///
/// // Emphasis token with nested content
/// let emphasis = Token::Emphasis {
///     level: 1,
///     content: vec![Token::Text("italic".to_string())]
/// };
///
/// // Link token with text and URL
/// let link = Token::Link("Click here".to_string(), "https://example.com".to_string());
/// ```
///
/// # Token Processing Pipeline
/// ```text
/// Input Text           Token Type                          PDF Element
/// -----------          -------------------------           -------------
/// # Heading     -->    Token::Heading(vec[], 1)     -->    <h1> styled text
/// *emphasis*    -->    Token::Emphasis{1, vec[]}    -->    <em> styled text
/// [link](url)   -->    Token::Link(text, url)       -->    <a> styled link
/// ```
///
/// # Nested Token Structure
/// ```text
/// Token::Heading
///   └── Vec<Token>
///       ├── Token::Text
///       ├── Token::Emphasis
///       │   └── Vec<Token>
///       │       └── Token::Text
///       └── Token::Link
///           ├── text: String
///           └── url: String
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A heading with nested content and level (e.g., # h1, ## h2)
    Heading(Vec<Token>, usize), // (content, level)
    /// Emphasized text with configurable level (1-3) for * or _ delimiters
    Emphasis {
        level: usize,
        content: Vec<Token>,
    },
    StrongEmphasis(Vec<Token>),
    /// Code block with language specification and content
    Code(String, String),
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

/// A lexical analyzer that converts Markdown text into a sequence of tokens.
/// Handles nested structures and special Markdown syntax elements.
///
/// # Examples
/// ```rust
/// use mdp::markdown::Lexer;
///
/// let mut lexer = Lexer::new("# Hello\n*world*".to_string());
/// let tokens = lexer.parse().unwrap();
///
/// // Parse specific elements
/// let mut lexer = Lexer::new("**bold text**".to_string());
/// let emphasis = lexer.parse_emphasis().unwrap();
///
/// let mut lexer = Lexer::new("[link](url)".to_string());
/// let link = lexer.parse_link().unwrap();
/// ```
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Parses the entire input string into a sequence of tokens.
    pub fn parse(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }

    /// Helper function to parse nested content until a delimiter is encountered.
    /// Used for parsing content within emphasis, headings, and list items.
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

    /// Determines the next token in the input stream based on the current character
    /// and context. Handles special cases like line starts differently.
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
    /// Parses emphasis tokens (* or _) with support for multiple levels (1-3).
    /// Ensures proper matching of opening and closing delimiters.
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
        let start_backticks = self.count_backticks();

        // Single backtick case
        if start_backticks == 1 {
            let mut content = String::new();

            // Read until either a closing backtick or end of input
            while self.position < self.input.len() {
                let ch = self.current_char();
                if ch == '`' {
                    self.advance(); // skip closing backtick
                    break;
                }
                content.push(ch);
                self.advance();
            }

            return Ok(Token::Code(String::new(), content));
        }

        // Multi-line code block case
        self.skip_whitespace();
        let language = self.read_until_newline();
        let mut content = String::new();

        while self.position < self.input.len() {
            let current_backticks = self.count_backticks();
            if current_backticks == start_backticks {
                break;
            }

            content.push(self.current_char());
            self.advance();
        }

        // Skip closing backticks if they exist
        for _ in 0..start_backticks {
            if self.position < self.input.len() && self.current_char() == '`' {
                self.advance();
            }
        }

        Ok(Token::Code(
            language.trim().to_string(),
            content.trim().to_string(),
        ))
    }

    // Helper method to count consecutive backticks
    fn count_backticks(&mut self) -> usize {
        let mut count = 0;
        while self.position < self.input.len() && self.current_char() == '`' {
            count += 1;
            self.advance();
        }
        count
    }

    fn parse_blockquote(&mut self) -> Result<Token, LexerError> {
        self.advance();
        self.skip_whitespace();
        let content = self.read_until_newline();
        Ok(Token::BlockQuote(content))
    }

    /// Handles both list items and horizontal rules since they can start with the same character.
    /// Checks for three consecutive hyphens to identify a horizontal rule.
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

    /// Parses regular text until a special token start or newline is encountered.
    /// Returns an error if no text could be parsed.
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
