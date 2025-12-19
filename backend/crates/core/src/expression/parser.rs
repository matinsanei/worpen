// Recursive descent parser for expressions
// Builds AST from tokens

use crate::expression::tokenizer::{Token, TokenType};
use crate::expression::ast::{Expr, BinaryOp, UnaryOp, PipeFilter};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }
    
    // expression → ternary
    fn expression(&mut self) -> Result<Expr, String> {
        self.ternary()
    }
    
    // ternary → pipe ( "?" expression ":" expression )?
    fn ternary(&mut self) -> Result<Expr, String> {
        let mut expr = self.pipe()?;
        
        if self.match_token(&TokenType::Question) {
            let true_expr = Box::new(self.expression()?);
            self.consume(&TokenType::Colon, "Expected ':' after ternary true branch")?;
            let false_expr = Box::new(self.expression()?);
            
            expr = Expr::Ternary {
                condition: Box::new(expr),
                true_expr,
                false_expr,
            };
        }
        
        Ok(expr)
    }
    
    // pipe → logical_or ( "|" IDENTIFIER ( "(" arguments? ")" )? )*
    fn pipe(&mut self) -> Result<Expr, String> {
        let mut expr = self.logical_or()?;
        
        if self.check(&TokenType::Pipe) {
            let mut filters = Vec::new();
            
            while self.match_token(&TokenType::Pipe) {
                let name = match &self.peek().token_type {
                    TokenType::Identifier(n) => {
                        let name = n.clone();
                        self.advance();
                        name
                    }
                    _ => return Err("Expected filter name after '|'".to_string()),
                };
                
                let args = if self.match_token(&TokenType::LParen) {
                    let args = if !self.check(&TokenType::RParen) {
                        self.arguments()?
                    } else {
                        Vec::new()
                    };
                    self.consume(&TokenType::RParen, "Expected ')' after arguments")?;
                    args
                } else {
                    Vec::new()
                };
                
                filters.push(PipeFilter { name, args });
            }
            
            expr = Expr::Pipe {
                expr: Box::new(expr),
                filters,
            };
        }
        
        Ok(expr)
    }
    
    // logical_or → logical_and ( "||" logical_and )*
    fn logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.logical_and()?;
        
        while self.match_token(&TokenType::PipePipe) {
            let right = self.logical_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // logical_and → equality ( "&&" equality )*
    fn logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.equality()?;
        
        while self.match_token(&TokenType::AmpAmp) {
            let right = self.equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // equality → comparison ( ( "==" | "!=" ) comparison )*
    fn equality(&mut self) -> Result<Expr, String> {
        let mut left = self.comparison()?;
        
        while let Some(op) = self.match_any(&[TokenType::EqEq, TokenType::BangEq]) {
            let binary_op = match op {
                TokenType::EqEq => BinaryOp::Eq,
                TokenType::BangEq => BinaryOp::Ne,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: binary_op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // comparison → addition ( ( "<" | "<=" | ">" | ">=" ) addition )*
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.addition()?;
        
        while let Some(op) = self.match_any(&[TokenType::Lt, TokenType::LtEq, TokenType::Gt, TokenType::GtEq]) {
            let binary_op = match op {
                TokenType::Lt => BinaryOp::Lt,
                TokenType::LtEq => BinaryOp::Le,
                TokenType::Gt => BinaryOp::Gt,
                TokenType::GtEq => BinaryOp::Ge,
                _ => unreachable!(),
            };
            let right = self.addition()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: binary_op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // addition → multiplication ( ( "+" | "-" ) multiplication )*
    fn addition(&mut self) -> Result<Expr, String> {
        let mut left = self.multiplication()?;
        
        while let Some(op) = self.match_any(&[TokenType::Plus, TokenType::Minus]) {
            let binary_op = match op {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.multiplication()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: binary_op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // multiplication → power ( ( "*" | "/" | "%" ) power )*
    fn multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.power()?;
        
        while let Some(op) = self.match_any(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let binary_op = match op {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                TokenType::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.power()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: binary_op,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // power → unary ( "**" unary )*
    fn power(&mut self) -> Result<Expr, String> {
        let mut left = self.unary()?;
        
        while self.match_token(&TokenType::StarStar) {
            let right = self.unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Pow,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // unary → ( "!" | "-" ) unary | call
    fn unary(&mut self) -> Result<Expr, String> {
        if let Some(op) = self.match_any(&[TokenType::Bang, TokenType::Minus]) {
            let unary_op = match op {
                TokenType::Bang => UnaryOp::Not,
                TokenType::Minus => UnaryOp::Neg,
                _ => unreachable!(),
            };
            let expr = self.unary()?;
            return Ok(Expr::Unary {
                op: unary_op,
                expr: Box::new(expr),
            });
        }
        
        self.call()
    }
    
    // call → primary ( "(" arguments? ")" )*
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        
        while self.match_token(&TokenType::LParen) {
            if let Expr::Variable(name) = expr {
                let args = if !self.check(&TokenType::RParen) {
                    self.arguments()?
                } else {
                    Vec::new()
                };
                self.consume(&TokenType::RParen, "Expected ')' after arguments")?;
                
                expr = Expr::Call { name, args };
            } else {
                return Err("Only identifiers can be called as functions".to_string());
            }
        }
        
        Ok(expr)
    }
    
    // arguments → expression ( "," expression )*
    fn arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = vec![self.expression()?];
        
        while self.match_token(&TokenType::Comma) {
            args.push(self.expression()?);
        }
        
        Ok(args)
    }
    
    // primary → NUMBER | STRING | "true" | "false" | "null"
    //         | IDENTIFIER | "(" expression ")" | "[" array "]" | "{" object "}"
    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek().clone();
        
        match &token.token_type {
            TokenType::Number(n) => {
                self.advance();
                Ok(Expr::Number(*n))
            }
            TokenType::String(s) => {
                self.advance();
                Ok(Expr::String(s.clone()))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            TokenType::Null => {
                self.advance();
                Ok(Expr::Null)
            }
            TokenType::Identifier(name) => {
                self.advance();
                Ok(Expr::Variable(name.clone()))
            }
            TokenType::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(&TokenType::RParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenType::LBracket => {
                self.advance();
                let items = if !self.check(&TokenType::RBracket) {
                    self.array_items()?
                } else {
                    Vec::new()
                };
                self.consume(&TokenType::RBracket, "Expected ']' after array items")?;
                Ok(Expr::Array(items))
            }
            TokenType::LBrace => {
                self.advance();
                let pairs = if !self.check(&TokenType::RBrace) {
                    self.object_pairs()?
                } else {
                    Vec::new()
                };
                self.consume(&TokenType::RBrace, "Expected '}' after object pairs")?;
                Ok(Expr::Object(pairs))
            }
            _ => Err(format!("Unexpected token: {:?}", token.token_type)),
        }
    }
    
    fn array_items(&mut self) -> Result<Vec<Expr>, String> {
        let mut items = vec![self.expression()?];
        
        while self.match_token(&TokenType::Comma) {
            items.push(self.expression()?);
        }
        
        Ok(items)
    }
    
    fn object_pairs(&mut self) -> Result<Vec<(String, Expr)>, String> {
        let mut pairs = Vec::new();
        
        loop {
            let key = match &self.peek().token_type {
                TokenType::Identifier(k) => {
                    let key = k.clone();
                    self.advance();
                    key
                }
                TokenType::String(k) => {
                    let key = k.clone();
                    self.advance();
                    key
                }
                _ => return Err("Expected object key (identifier or string)".to_string()),
            };
            
            self.consume(&TokenType::Colon, "Expected ':' after object key")?;
            let value = self.expression()?;
            pairs.push((key, value));
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        Ok(pairs)
    }
    
    // Helper methods
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn match_any(&mut self, types: &[TokenType]) -> Option<TokenType> {
        for t in types {
            if self.check(t) {
                let matched = self.peek().token_type.clone();
                self.advance();
                return Some(matched);
            }
        }
        None
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<(), String> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            Err(format!("{} (got {:?})", message, self.peek().token_type))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::tokenizer::Tokenizer;
    
    fn parse(input: &str) -> Result<Expr, String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
    
    #[test]
    fn test_parse_number() {
        let expr = parse("42").unwrap();
        assert!(matches!(expr, Expr::Number(42.0)));
    }
    
    #[test]
    fn test_parse_string() {
        let expr = parse(r#""hello""#).unwrap();
        match expr {
            Expr::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }
    }
    
    #[test]
    fn test_parse_variable() {
        let expr = parse("foo").unwrap();
        match expr {
            Expr::Variable(name) => assert_eq!(name, "foo"),
            _ => panic!("Expected variable"),
        }
    }
    
    #[test]
    fn test_parse_binary() {
        let expr = parse("1 + 2").unwrap();
        match expr {
            Expr::Binary { left, op, right } => {
                assert!(matches!(*left, Expr::Number(1.0)));
                assert!(matches!(op, BinaryOp::Add));
                assert!(matches!(*right, Expr::Number(2.0)));
            }
            _ => panic!("Expected binary expression"),
        }
    }
    
    #[test]
    fn test_parse_precedence() {
        let expr = parse("1 + 2 * 3").unwrap();
        match expr {
            Expr::Binary { left, op: BinaryOp::Add, right } => {
                assert!(matches!(*left, Expr::Number(1.0)));
                match *right {
                    Expr::Binary { left, op: BinaryOp::Mul, right } => {
                        assert!(matches!(*left, Expr::Number(2.0)));
                        assert!(matches!(*right, Expr::Number(3.0)));
                    }
                    _ => panic!("Expected multiplication"),
                }
            }
            _ => panic!("Expected addition"),
        }
    }
    
    #[test]
    fn test_parse_function_call() {
        let expr = parse("foo(1, 2)").unwrap();
        match expr {
            Expr::Call { name, args } => {
                assert_eq!(name, "foo");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected function call"),
        }
    }
    
    #[test]
    fn test_parse_pipe() {
        let expr = parse("x | upper | trim").unwrap();
        match expr {
            Expr::Pipe { expr, filters } => {
                assert!(matches!(*expr, Expr::Variable(_)));
                assert_eq!(filters.len(), 2);
                assert_eq!(filters[0].name, "upper");
                assert_eq!(filters[1].name, "trim");
            }
            _ => panic!("Expected pipe expression"),
        }
    }
    
    #[test]
    fn test_parse_ternary() {
        let expr = parse("x > 0 ? 1 : -1").unwrap();
        match expr {
            Expr::Ternary { condition, true_expr, false_expr } => {
                assert!(matches!(*condition, Expr::Binary { .. }));
                assert!(matches!(*true_expr, Expr::Number(1.0)));
                assert!(matches!(*false_expr, Expr::Unary { .. }));
            }
            _ => panic!("Expected ternary expression"),
        }
    }
}
