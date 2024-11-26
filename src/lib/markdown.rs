//! Markdown lexical analysis and token representation.
//!
//! This module provides the core lexical analysis functionality for parsing Markdown text into a
//! structured token stream. It handles both block-level elements like headings and lists, as well
//! as inline formatting like emphasis and links.
//!
//! The lexer maintains proper nesting of elements and handles edge cases around delimiter matching
//! and whitespace handling according to CommonMark spec.
//!
//! # Examples
//! ```rust
//! use markdown2pdf::Token;
//!
//! // Heading token with nested content
//! let heading = Token::Heading(vec![Token::Text("Title".to_string())], 1);
//!
//! // Emphasis token with nested content
//! let emphasis = Token::Emphasis {
//!     level: 1,
//!     content: vec![Token::Text("italic".to_string())]
//! };
//!
//! // Link token with text and URL
//! let link = Token::Link("Click here".to_string(), "https://example.com".to_string());
//! ```
//!
//! Token (nested) structure looks like:
//! Token::Heading
//! └── Vec<Token>
//!     ├── Token::Text
//!     ├── Token::Emphasis
//!     │   └── Vec<Token>
//!     │       └── Token::Text
//!     └── Token::Link
//!         ├── text: String
//!         └── url: String

/// Represents the different types of tokens that can be parsed from Markdown text.
/// Each variant captures both the semantic meaning and associated content/metadata
/// needed to properly render the element.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A heading with nested content and level (e.g., # h1, ## h2)
    Heading(Vec<Token>, usize),
    /// Emphasized text with configurable level (1-3) for * or _ delimiters
    Emphasis { level: usize, content: Vec<Token> },
    /// Strong emphasis (bold) text using ** or __ delimiters
    StrongEmphasis(Vec<Token>),
    /// Code block with optional language specification and content
    Code(String, String),
    /// Block quote containing quoted text
    BlockQuote(String),
    /// List item with nested content and type information
    ListItem {
        content: Vec<Token>,
        ordered: bool,
        number: Option<usize>, // For ordered lists (e.g., "1.", "2.")
    },
    /// Link with display text and URL
    Link(String, String),
    /// Image with alt text and URL
    Image(String, String),
    /// Plain text content
    Text(String),
    /// HTML comment content
    HtmlComment(String),
    /// Line break
    Newline,
    /// Horizontal rule (---)
    HorizontalRule,
    /// Unknown or malformed token
    Unknown(String),
}

/// Error types that can occur during lexical analysis
#[derive(Debug)]
pub enum LexerError {
    /// Input ended unexpectedly while parsing a token
    UnexpectedEndOfInput,
    /// Encountered an invalid or malformed token
    UnknownToken(String),
}

/// A lexical analyzer that converts Markdown text into a sequence of tokens.
/// Handles nested structures and special Markdown syntax elements while maintaining
/// proper context and state during parsing.
pub struct Lexer {
    /// Input text as character vector for efficient parsing
    input: Vec<char>,
    /// Current position in the input stream
    position: usize,
}

impl Lexer {
    /// Creates a new lexer instance from input string
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Parses the entire input string into a sequence of tokens.
    /// Returns a Result containing either a Vec of parsed tokens or a LexerError.
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
        let initial_indent = self.get_current_indent();

        while self.position < self.input.len() {
            let ch = self.current_char();

            if is_delimiter(ch) {
                break;
            }

            // Handle nested content
            if self.is_at_line_start() {
                let current_indent = self.get_current_indent();

                // If more indented than parent, treat as nested content
                if current_indent > initial_indent {
                    self.position += current_indent;

                    match self.current_char() {
                        '-' | '+' => {
                            if !self.check_horizontal_rule()? {
                                content.push(self.parse_list_item(false, current_indent)?);
                                continue;
                            }
                        }
                        '0'..='9' => {
                            if self.check_ordered_list_marker().is_some() {
                                content.push(self.parse_list_item(true, current_indent)?);
                                continue;
                            }
                        }
                        _ => {}
                    }
                }
            }

            // Parse regular content
            if let Some(token) = self.next_token()? {
                content.push(token);
            }
        }

        Ok(content)
    }

    /// Determines the next token in the input stream based on the current character
    /// and context. Handles special cases like line starts differently.
    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        // Only skip whitespace if we're not immediately after a special token
        if !self.is_after_special_token() {
            self.skip_whitespace();
        }

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
            '-' | '+' if is_line_start => {
                if self.check_horizontal_rule()? {
                    Token::HorizontalRule
                } else {
                    self.parse_list_item(false, 0)?
                }
            }
            '0'..='9' if is_line_start => {
                if let Some(_) = self.check_ordered_list_marker() {
                    self.parse_list_item(true, 0)?
                } else {
                    self.parse_text()?
                }
            }
            '[' => self.parse_link()?,
            '!' => self.parse_image()?,
            '<' if self.is_html_comment_start() => self.parse_html_comment()?,
            '\n' => self.parse_newline()?,
            _ => self.parse_text()?,
        };

        Ok(Some(token))
    }

    /// Parses a heading token, counting '#' characters for level and collecting nested content
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

    /// Parses code blocks, handling both inline code and fenced code blocks
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

    /// Helper method to count consecutive backticks
    fn count_backticks(&mut self) -> usize {
        let mut count = 0;
        while self.position < self.input.len() && self.current_char() == '`' {
            count += 1;
            self.advance();
        }
        count
    }

    /// Parses a blockquote, collecting text until newline
    fn parse_blockquote(&mut self) -> Result<Token, LexerError> {
        self.advance();
        self.skip_whitespace();
        let content = self.read_until_newline();
        Ok(Token::BlockQuote(content))
    }

    /// Parses a link token, extracting display text and URL
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

    /// Parses an image token, extracting alt text and URL
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

    /// Parses a newline token
    fn parse_newline(&mut self) -> Result<Token, LexerError> {
        self.advance();
        Ok(Token::Newline)
    }

    /// Parses regular text until a special token start or newline is encountered.
    /// Returns an error if no text could be parsed.
    fn parse_text(&mut self) -> Result<Token, LexerError> {
        let mut content = String::new();
        let start_pos = self.position;

        // If we're starting with a space after a special token, include it
        if self.position > 0 && self.current_char() == ' ' {
            content.push(' ');
            self.advance();
        }

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

    /// Parses an HTML comment, extracting the comment content
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

    /// Checks if current position is at the start of a line
    fn is_at_line_start(&self) -> bool {
        self.position == 0 || self.input.get(self.position - 1) == Some(&'\n')
    }

    /// Skips whitespace characters except newlines
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.current_char().is_whitespace()
            && self.current_char() != '\n'
        {
            self.advance();
        }
    }

    /// Advances the position counter by one
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Returns the current character or '\0' if at end of input
    fn current_char(&self) -> char {
        *self.input.get(self.position).unwrap_or(&'\0')
    }

    /// Reads characters until a newline is encountered
    fn read_until_newline(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '\n' {
            self.advance();
        }
        self.input[start..self.position].iter().collect()
    }

    /// Reads characters until a specific delimiter is encountered
    fn read_until_char(&mut self, delimiter: char) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != delimiter {
            self.advance();
        }
        self.input[start..self.position].iter().collect()
    }

    /// Checks if current position starts an HTML comment
    fn is_html_comment_start(&self) -> bool {
        self.input[self.position..]
            .iter()
            .collect::<String>()
            .starts_with("<!--")
    }

    /// Checks if current character could start a special token
    fn is_start_of_special_token(&self) -> bool {
        let ch = self.current_char();
        match ch {
            '#' | '*' | '_' | '`' | '[' | '!' => true,
            '<' => self.is_html_comment_start(),
            _ => false,
        }
    }

    /// Checks if we're immediately after a special token that should preserve following spaces
    fn is_after_special_token(&self) -> bool {
        if self.position == 0 {
            return false;
        }

        let prev_char = self.input[self.position - 1];
        match prev_char {
            '`' | ')' => true,
            _ => false,
        }
    }

    /// Checks if the current position contains a horizontal rule (---)
    fn check_horizontal_rule(&mut self) -> Result<bool, LexerError> {
        if self.current_char() == '-' {
            let mut count = 1;
            let mut pos = self.position + 1;

            // Look ahead for at least 3 consecutive hyphens
            while pos < self.input.len() && self.input[pos] == '-' {
                count += 1;
                pos += 1;
            }

            if count >= 3 {
                self.position = pos;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Checks if current position starts an ordered list marker (e.g., "1.")
    fn check_ordered_list_marker(&mut self) -> Option<usize> {
        let start_pos = self.position;
        let mut pos = start_pos;
        let mut number_str = String::new();

        while pos < self.input.len() && self.input[pos].is_ascii_digit() {
            number_str.push(self.input[pos]);
            pos += 1;
        }

        if pos < self.input.len() && self.input[pos] == '.' {
            if let Ok(number) = number_str.parse::<usize>() {
                return Some(number);
            }
        }

        None
    }

    /// Parses a list item, handling both ordered and unordered types
    fn parse_list_item(&mut self, ordered: bool, indent_level: usize) -> Result<Token, LexerError> {
        let mut number = None;

        if !ordered {
            self.advance();
        } else {
            number = self.check_ordered_list_marker();
            // Skip past number and period
            while self.position < self.input.len()
                && (self.current_char().is_ascii_digit() || self.current_char() == '.')
            {
                self.advance();
            }
        }

        self.skip_whitespace();

        let mut content = Vec::new();
        while self.position < self.input.len() && self.current_char() != '\n' {
            if let Some(token) = self.next_token()? {
                content.push(token);
            }
        }

        // Move to next line if exists
        if self.position < self.input.len() && self.current_char() == '\n' {
            self.advance();
        }

        while self.position < self.input.len() {
            let current_indent = self.get_current_indent();
            if current_indent <= indent_level {
                // Back to same or lower indentation level, exit nested parsing
                break;
            }

            self.position += current_indent;
            match self.current_char() {
                '-' | '+' => {
                    if !self.check_horizontal_rule()? {
                        content.push(self.parse_list_item(false, current_indent)?);
                    }
                }
                '0'..='9' => {
                    if self.check_ordered_list_marker().is_some() {
                        content.push(self.parse_list_item(true, current_indent)?);
                    }
                }
                _ => break,
            }
        }

        Ok(Token::ListItem {
            content,
            ordered,
            number,
        })
    }

    /// Gets the current line's indentation level
    fn get_current_indent(&self) -> usize {
        let mut count = 0;
        let mut pos = self.position;

        while pos < self.input.len() {
            match self.input[pos] {
                ' ' => count += 1,
                '\t' => count += 4, // Convert tabs to spaces (common convention)
                _ => break,
            }
            pos += 1;
        }
        count
    }
}
