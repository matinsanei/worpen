// Expression evaluation module for Dynamic Routes
// Supports: variables, operators, function calls, pipe operators

pub mod ast;
pub mod evaluator;
pub mod filters;
pub mod functions;
pub mod parser;
pub mod template;
pub mod tokenizer;

pub use ast::{BinaryOp, Expr, PipeFilter, UnaryOp};
pub use evaluator::Evaluator;
pub use parser::Parser;
pub use template::resolve_templates;
pub use tokenizer::{Token, TokenType, Tokenizer};
