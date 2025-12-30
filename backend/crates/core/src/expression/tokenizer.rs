// Tokenizer for expression parsing
// Converts string expressions into tokens
use std::borrow::Cow;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType<'a> {
    // Literals
    Number(f64),
    String(Cow<'a, str>),
    True,
    False,
    Null,
    
    // Identifiers
    Identifier(Cow<'a, str>),
    
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
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub line: usize,
    pub column: usize,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, String> {
        let mut tokens = Vec::new();
        
        loop {
            self.skip_whitespace();
            
            if self.is_at_end() {
                tokens.push(Token {
                    token_type: TokenType::Eof,
                    line: self.line,
                    column: self.column,
                });
                break;
            }
            
            tokens.push(self.next_token()?);
        }
        
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token<'a>, String> {
        let start_line = self.line;
        let start_column = self.column;
        let start_pos = self.position;
        
        // We peek to see the character, but we need to verify we can actually consume it
        // The implementation logic is slightly different with Peekable
        
        let ch = if let Some(c) = self.peek_char() {
            c
        } else {
             return Ok(Token {
                token_type: TokenType::Eof,
                line: start_line,
                column: start_column,
            });
        };

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
                if self.peek_char() == Some('*') {
                    self.advance();
                    TokenType::StarStar
                } else {
                    TokenType::Star
                }
            }
            
            '=' => {
                self.advance();
                if self.peek_char() == Some('=') {
                     self.advance();
                    TokenType::EqEq
                } else {
                    return Err(format!("Unexpected character '=' at {}:{}", start_line, start_column));
                }
            }
            
            '!' => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenType::BangEq
                } else {
                    TokenType::Bang
                }
            }
            
            '<' => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenType::LtEq
                } else {
                    TokenType::Lt
                }
            }
            
            '>' => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    TokenType::GtEq
                } else {
                    TokenType::Gt
                }
            }
            
            '&' => {
                self.advance();
                if self.peek_char() == Some('&') {
                    self.advance();
                    TokenType::AmpAmp
                } else {
                    return Err(format!("Unexpected character '&' at {}:{}", start_line, start_column));
                }
            }
            
            '|' => {
                self.advance();
                if self.peek_char() == Some('|') {
                    self.advance();
                    TokenType::PipePipe
                } else {
                    TokenType::Pipe
                }
            }
            
            // Strings
            '"' | '\'' => return self.read_string(ch),
            
            // Numbers
            '0'..='9' => return self.read_number(),
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(start_pos),
            
            _ => return Err(format!("Unexpected character '{}' at {}:{}", ch, start_line, start_column)),
        };
        
        Ok(Token {
            token_type,
            line: start_line,
            column: start_column,
        })
    }
    
    // Optimized string reading - zero copy if no escapes
    fn read_string(&mut self, quote: char) -> Result<Token<'a>, String> {
        let start_line = self.line;
        let start_column = self.column;
        let _start_pos = self.position; // Position of the opening quote
        
        self.advance(); // Skip opening quote
        
        let content_start = self.position;
        let mut has_escapes = false;
        
        while let Some(ch) = self.peek_char() {
            if ch == quote {
                break;
            }
            if ch == '\\' {
                has_escapes = true;
                self.advance(); // skip \
                if self.is_at_end() {
                    return Err(format!("Unterminated string at {}:{}", start_line, start_column));
                }
                self.advance(); // skip escaped char
            } else {
                self.advance();
            }
        }
        
        if self.is_at_end() {
            return Err(format!("Unterminated string at {}:{}", start_line, start_column));
        }
        
        let content_end = self.position;
        self.advance(); // Skip closing quote
        
        let value = if has_escapes {
            // Must allocate if there are escapes
            // Fallback to manual processing
            // Note: Since we are in the middle of iterating, we can't easily go back to use indices directly
            // effectively for processing escapes without re-iterating (or writing a helper).
            // For simplicity and correctness with escapes, we allocate.
            // But we can optimize: we know the slice of input.
            let raw_slice = &self.input[content_start..content_end];
            let mut value = String::with_capacity(raw_slice.len());
            let mut chars = raw_slice.chars();
            while let Some(c) = chars.next() {
                if c == '\\' {
                    match chars.next() {
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some('r') => value.push('\r'),
                        Some('\\') => value.push('\\'),
                        Some('"') => value.push('"'),
                        Some('\'') => value.push('\''),
                        Some(other) => {
                            value.push('\\');
                            value.push(other);
                        }
                        None => break, // Should not happen due to previous check
                    }
                } else {
                    value.push(c);
                }
            }
            Cow::Owned(value)
        } else {
            // ZERO COPY!
            Cow::Borrowed(&self.input[content_start..content_end])
        };
        
        Ok(Token {
            token_type: TokenType::String(value),
            line: start_line,
            column: start_column,
        })
    }
    
    fn read_number(&mut self) -> Result<Token<'a>, String> {
        let start_line = self.line;
        let start_column = self.column;
        let start_pos = self.position;
        
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        
        // Check for decimal part
        if self.peek_char() == Some('.') {
            // Need to peek next char to confirm it is a digit, otherwise might be property access (not currently supported but good practice)
            // But wait, our peek_char is just one char. We need to be careful.
            // Tokenizer struct doesn't easily support double peek with simple Chars.
            // Let's assume valid syntax for now or complex logic.
            // Actually, we can consume dot if next is digit.
            
            // Hacky workaround for peek(2):
            // We consume dot temporarily? No.
            // We just look at input slice.
            let current_idx = self.position;
             if current_idx + 1 < self.input.len() {
                  let next_char = self.input[current_idx+1..].chars().next().unwrap();
                  if next_char.is_ascii_digit() {
                       self.advance(); // consume .
                       while let Some(c) = self.peek_char() {
                            if c.is_ascii_digit() {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                  }
             }
        }
        
        let value_str = &self.input[start_pos..self.position];
        
        let num = value_str.parse::<f64>()
            .map_err(|_| format!("Invalid number '{}' at {}:{}", value_str, start_line, start_column))?;
        
        Ok(Token {
            token_type: TokenType::Number(num),
            line: start_line,
            column: start_column,
        })
    }
    
    fn read_identifier(&mut self, start_pos: usize) -> Result<Token<'a>, String> {
        let start_line = self.line;
        let start_column = self.column;
        
        // First char already checked by caller, but not consumed? 
        // Wait, loop in next_token consumes 'a'..'z' etc? 
        // No, `return self.read_identifier()` in matching.
        // The match verified the first char, but didn't advance if we used `peek`?
        // Ah, the logic in `next_token` used `self.current_char()` which was `input[pos]`. 
        // My new `next_token` peeks. It hasn't advanced past the start char.
        // So we consume here.
        
        while let Some(c) = self.peek_char() {
             if c.is_alphanumeric() || c == '_' {
                 self.advance();
             } else {
                 break;
             }
        }
        
        let text = &self.input[start_pos..self.position];
        
        let token_type = match text {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier(Cow::Borrowed(text)),
        };
        
        Ok(Token {
            token_type,
            line: start_line,
            column: start_column,
        })
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                 if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.position += c.len_utf8(); // Advance position manually since we are skipping self.advance() logic for line tracking?
                // Wait, self.advance() handles column/pos.
                // Let's use internal logic that creates the iterator.
                // Actually `chars.next()` advances the internal iterator.
                // `position` is for our slice slicing.
                self.chars.next(); 
            } else {
                break;
            }
        }
    }
    
    // Helper to get current char without consuming
    fn peek_char(&mut self) -> Option<char> {
        // We can't rely on self.input[self.position] safely if we are using chars iterator for unicode correctness?
        // Actually `self.input[..]` indexing works on bytes. `chars()` iterates unicode scalars.
        // Mixing indices and chars iterator is dangerous if not careful.
        // It's better to stick to one source of truth.
        // But we need lifetimes yielding `&'a str` from `input`.
        // So `position` MUST be a byte index.
        
        // We can maintain byte index by adding `c.len_utf8()` on advance.
        self.chars.peek().copied()
    }
    
    fn advance(&mut self) {
        if let Some(c) = self.chars.next() {
            self.position += c.len_utf8();
            self.column += 1;
        }
    }
    
    fn is_at_end(&mut self) -> bool {
        self.peek_char().is_none()
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
    fn test_tokenize_operators() {
        let mut tokenizer = Tokenizer::new("+ - * / % ** == != < <= > >= && || !");
        let tokens = tokenizer.tokenize().unwrap();
         
        assert!(matches!(tokens[0].token_type, TokenType::Plus));
        assert!(matches!(tokens[5].token_type, TokenType::StarStar));
        assert!(matches!(tokens[12].token_type, TokenType::AmpAmp));
        assert!(matches!(tokens[13].token_type, TokenType::PipePipe));
    }
}
