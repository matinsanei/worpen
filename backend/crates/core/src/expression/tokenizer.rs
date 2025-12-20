// Tokenizer for expression parsing
// Converts string expressions into tokens

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    True,
    False,
    Null,
    
    // Identifiers
    Identifier(String),
    
    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    StarStar,       // **
    
    EqEq,           // ==
    BangEq,         // !=
    Lt,             // <
    LtEq,           // <=
    Gt,             // >
    GtEq,           // >=
    
    AmpAmp,         // &&
    PipePipe,       // ||
    Bang,           // !
    
    // Delimiters
    LParen,         // (
    RParen,         // )
    LBracket,       // [
    RBracket,       // ]
    LBrace,         // {
    RBrace,         // }
    
    Comma,          // ,
    Colon,          // :
    Question,       // ?
    Pipe,           // |
    Dot,            // .
    
    // End of input
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        loop {
            self.skip_whitespace();
            
            if self.is_at_end() {
                tokens.push(Token {
                    token_type: TokenType::Eof,
                    lexeme: String::new(),
                    line: self.line,
                    column: self.column,
                });
                break;
            }
            
            tokens.push(self.next_token()?);
        }
        
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;
        let ch = self.current_char();
        
        let token_type = match ch {
            // Single character tokens
            '(' => { self.advance(); TokenType::LParen }
            ')' => { self.advance(); TokenType::RParen }
            '[' => { self.advance(); TokenType::LBracket }
            ']' => { self.advance(); TokenType::RBracket }
            '{' => { self.advance(); TokenType::LBrace }
            '}' => { self.advance(); TokenType::RBrace }
            ',' => { self.advance(); TokenType::Comma }
            ':' => { self.advance(); TokenType::Colon }
            '?' => { self.advance(); TokenType::Question }
            '%' => { self.advance(); TokenType::Percent }
            '.' => { self.advance(); TokenType::Dot }
            
            // Operators (can be multi-char)
            '+' => { self.advance(); TokenType::Plus }
            '-' => { self.advance(); TokenType::Minus }
            '/' => { self.advance(); TokenType::Slash }
            
            '*' => {
                self.advance();
                if self.current_char() == '*' {
                    self.advance();
                    TokenType::StarStar
                } else {
                    TokenType::Star
                }
            }
            
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    TokenType::EqEq
                } else {
                    return Err(format!("Unexpected character '=' at {}:{}", start_line, start_column));
                }
            }
            
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    TokenType::BangEq
                } else {
                    TokenType::Bang
                }
            }
            
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    TokenType::LtEq
                } else {
                    TokenType::Lt
                }
            }
            
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    TokenType::GtEq
                } else {
                    TokenType::Gt
                }
            }
            
            '&' => {
                self.advance();
                if self.current_char() == '&' {
                    self.advance();
                    TokenType::AmpAmp
                } else {
                    return Err(format!("Unexpected character '&' at {}:{}", start_line, start_column));
                }
            }
            
            '|' => {
                self.advance();
                if self.current_char() == '|' {
                    self.advance();
                    TokenType::PipePipe
                } else {
                    TokenType::Pipe
                }
            }
            
            // Strings
            '"' | '\'' => return self.read_string(),
            
            // Numbers
            '0'..='9' => return self.read_number(),
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(),
            
            _ => return Err(format!("Unexpected character '{}' at {}:{}", ch, start_line, start_column)),
        };
        
        Ok(Token {
            token_type,
            lexeme: String::new(),
            line: start_line,
            column: start_column,
        })
    }
    
    fn read_string(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;
        let quote = self.current_char();
        self.advance(); // Skip opening quote
        
        let mut value = String::new();
        
        while !self.is_at_end() && self.current_char() != quote {
            if self.current_char() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err(format!("Unterminated string at {}:{}", start_line, start_column));
                }
                match self.current_char() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    _ => {
                        value.push('\\');
                        value.push(self.current_char());
                    }
                }
                self.advance();
            } else {
                value.push(self.current_char());
                self.advance();
            }
        }
        
        if self.is_at_end() {
            return Err(format!("Unterminated string at {}:{}", start_line, start_column));
        }
        
        self.advance(); // Skip closing quote
        
        Ok(Token {
            token_type: TokenType::String(value.clone()),
            lexeme: value,
            line: start_line,
            column: start_column,
        })
    }
    
    fn read_number(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::new();
        
        while !self.is_at_end() && self.current_char().is_ascii_digit() {
            value.push(self.current_char());
            self.advance();
        }
        
        // Check for decimal part
        if !self.is_at_end() && self.current_char() == '.' {
            let next_pos = self.position + 1;
            if next_pos < self.input.len() && self.input[next_pos].is_ascii_digit() {
                value.push('.');
                self.advance();
                
                while !self.is_at_end() && self.current_char().is_ascii_digit() {
                    value.push(self.current_char());
                    self.advance();
                }
            }
        }
        
        let num = value.parse::<f64>()
            .map_err(|_| format!("Invalid number '{}' at {}:{}", value, start_line, start_column))?;
        
        Ok(Token {
            token_type: TokenType::Number(num),
            lexeme: value,
            line: start_line,
            column: start_column,
        })
    }
    
    fn read_identifier(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;
        let mut value = String::new();
        
        while !self.is_at_end() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            value.push(self.current_char());
            self.advance();
        }
        
        let token_type = match value.as_str() {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier(value.clone()),
        };
        
        Ok(Token {
            token_type,
            lexeme: value,
            line: start_line,
            column: start_column,
        })
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().is_whitespace() {
            if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }
    
    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }
    
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.column += 1;
            self.position += 1;
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_numbers() {
        let mut tokenizer = Tokenizer::new("42 3.14");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3); // 42, 3.14, EOF
        assert!(matches!(tokens[0].token_type, TokenType::Number(42.0)));
        #[allow(clippy::approx_constant)]
        {
            assert!(matches!(tokens[1].token_type, TokenType::Number(3.14)));
        }
    }
    
    #[test]
    fn test_tokenize_strings() {
        let mut tokenizer = Tokenizer::new(r#""hello" 'world'"#);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3);
        match &tokens[0].token_type {
            TokenType::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }
    }
    
    #[test]
    fn test_tokenize_operators() {
        let mut tokenizer = Tokenizer::new("+ - * / % ** == != < <= > >= && || !");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Plus));
        assert!(matches!(tokens[1].token_type, TokenType::Minus));
        assert!(matches!(tokens[2].token_type, TokenType::Star));
        assert!(matches!(tokens[3].token_type, TokenType::Slash));
        assert!(matches!(tokens[4].token_type, TokenType::Percent));
        assert!(matches!(tokens[5].token_type, TokenType::StarStar));
        assert!(matches!(tokens[6].token_type, TokenType::EqEq));
        assert!(matches!(tokens[7].token_type, TokenType::BangEq));
        assert!(matches!(tokens[8].token_type, TokenType::Lt));
        assert!(matches!(tokens[9].token_type, TokenType::LtEq));
        assert!(matches!(tokens[10].token_type, TokenType::Gt));
        assert!(matches!(tokens[11].token_type, TokenType::GtEq));
        assert!(matches!(tokens[12].token_type, TokenType::AmpAmp));
        assert!(matches!(tokens[13].token_type, TokenType::PipePipe));
        assert!(matches!(tokens[14].token_type, TokenType::Bang));
    }
    
    #[test]
    fn test_tokenize_identifiers() {
        let mut tokenizer = Tokenizer::new("foo bar_baz true false null");
        let tokens = tokenizer.tokenize().unwrap();
        
        match &tokens[0].token_type {
            TokenType::Identifier(s) => assert_eq!(s, "foo"),
            _ => panic!("Expected identifier"),
        }
        match &tokens[1].token_type {
            TokenType::Identifier(s) => assert_eq!(s, "bar_baz"),
            _ => panic!("Expected identifier"),
        }
        assert!(matches!(tokens[2].token_type, TokenType::True));
        assert!(matches!(tokens[3].token_type, TokenType::False));
        assert!(matches!(tokens[4].token_type, TokenType::Null));
    }
    
    #[test]
    fn test_tokenize_expression() {
        let mut tokenizer = Tokenizer::new("x + 2 * 3");
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 6); // x, +, 2, *, 3, EOF
    }
}
