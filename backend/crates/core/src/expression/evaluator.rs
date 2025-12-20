// Expression evaluator - evaluates AST to produce values
// Phase 2 Day 7-8: Full implementation

use crate::expression::ast::{Expr, BinaryOp, UnaryOp, PipeFilter};
use crate::expression::filters;
use crate::helpers;
use serde_json::{Value, Number};
use std::collections::HashMap;

type EvaluatorFunction = Box<dyn Fn(Vec<Value>) -> Result<Value, String>>;

pub struct Evaluator {
    variables: HashMap<String, Value>,
    functions: HashMap<String, EvaluatorFunction>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut evaluator = Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };
        evaluator.register_builtin_functions();
        evaluator
    }
    
    pub fn with_variables(variables: HashMap<String, Value>) -> Self {
        let mut evaluator = Self::new();
        evaluator.variables = variables;
        evaluator
    }
    
    pub fn evaluate(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(
                Number::from_f64(*n).ok_or("Invalid number")?
            )),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Boolean(b) => Ok(Value::Bool(*b)),
            
            Expr::Null => Ok(Value::Null),
            
            Expr::Variable(name) => {
                self.variables.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.eval_binary_op(&left_val, op.clone(), &right_val)
            }
            
            Expr::Unary { op, expr } => {
                let val = self.evaluate(expr)?;
                self.eval_unary_op(op.clone(), &val)
            }
            
            Expr::Call { name, args } => {
                let evaluated_args: Result<Vec<Value>, String> = args
                    .iter()
                    .map(|arg| self.evaluate(arg))
                    .collect();
                let evaluated_args = evaluated_args?;
                
                self.functions.get(name)
                    .ok_or_else(|| format!("Undefined function: {}", name))?
                    (evaluated_args)
            }
            
            Expr::Pipe { expr, filters } => {
                let mut value = self.evaluate(expr)?;
                for filter in filters {
                    value = self.apply_filter(&value, filter)?;
                }
                Ok(value)
            }
            
            Expr::Ternary { condition, true_expr, false_expr } => {
                let cond_val = self.evaluate(condition)?;
                if self.is_truthy(&cond_val) {
                    self.evaluate(true_expr)
                } else {
                    self.evaluate(false_expr)
                }
            }
            
            Expr::Array(items) => {
                let evaluated: Result<Vec<Value>, String> = items
                    .iter()
                    .map(|item| self.evaluate(item))
                    .collect();
                Ok(Value::Array(evaluated?))
            }
            
            Expr::Object(pairs) => {
                let mut map = serde_json::Map::new();
                for (key, value_expr) in pairs {
                    let value = self.evaluate(value_expr)?;
                    map.insert(key.clone(), value);
                }
                Ok(Value::Object(map))
            }
        }
    }
    
    fn eval_binary_op(&self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value, String> {
        match op {
            BinaryOp::Add | BinaryOp::Concat => self.add(left, right),
            BinaryOp::Sub => self.numeric_op(left, right, |a, b| a - b),
            BinaryOp::Mul => self.numeric_op(left, right, |a, b| a * b),
            BinaryOp::Div => self.numeric_op(left, right, |a, b| {
                if b == 0.0 {
                    f64::NAN
                } else {
                    a / b
                }
            }),
            BinaryOp::Mod => self.numeric_op(left, right, |a, b| a % b),
            BinaryOp::Pow => self.numeric_op(left, right, |a, b| a.powf(b)),
            
            BinaryOp::Eq => Ok(Value::Bool(left == right)),
            BinaryOp::Ne => Ok(Value::Bool(left != right)),
            
            BinaryOp::Lt => self.comparison_op(left, right, |a, b| a < b),
            BinaryOp::Le => self.comparison_op(left, right, |a, b| a <= b),
            BinaryOp::Gt => self.comparison_op(left, right, |a, b| a > b),
            BinaryOp::Ge => self.comparison_op(left, right, |a, b| a >= b),
            
            BinaryOp::And => {
                if !self.is_truthy(left) {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }
            
            BinaryOp::Or => {
                if self.is_truthy(left) {
                    Ok(left.clone())
                } else {
                    Ok(right.clone())
                }
            }
        }
    }
    
    fn eval_unary_op(&self, op: UnaryOp, val: &Value) -> Result<Value, String> {
        match op {
            UnaryOp::Neg => {
                if let Some(n) = val.as_f64() {
                    Ok(Value::Number(Number::from_f64(-n).ok_or("Invalid number")?))
                } else {
                    Err(format!("Cannot negate non-numeric value: {:?}", val))
                }
            }
            UnaryOp::Not => Ok(Value::Bool(!self.is_truthy(val))),
        }
    }
    
    fn add(&self, left: &Value, right: &Value) -> Result<Value, String> {
        // Number + Number
        if let (Some(a), Some(b)) = (left.as_f64(), right.as_f64()) {
            return Ok(Value::Number(Number::from_f64(a + b).ok_or("Invalid number")?));
        }
        
        // String + String (concatenation)
        if let (Some(a), Some(b)) = (left.as_str(), right.as_str()) {
            return Ok(Value::String(format!("{}{}", a, b)));
        }
        
        // Array + Array (concatenation)
        if let (Some(a), Some(b)) = (left.as_array(), right.as_array()) {
            let mut result = a.clone();
            result.extend(b.clone());
            return Ok(Value::Array(result));
        }
        
        Err(format!("Cannot add {:?} and {:?}", left, right))
    }
    
    fn numeric_op<F>(&self, left: &Value, right: &Value, op: F) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> f64,
    {
        let a = left.as_f64().ok_or_else(|| format!("Left operand is not a number: {:?}", left))?;
        let b = right.as_f64().ok_or_else(|| format!("Right operand is not a number: {:?}", right))?;
        Ok(Value::Number(Number::from_f64(op(a, b)).ok_or("Invalid number")?))
    }
    
    fn comparison_op<F>(&self, left: &Value, right: &Value, op: F) -> Result<Value, String>
    where
        F: Fn(f64, f64) -> bool,
    {
        let a = left.as_f64().ok_or_else(|| format!("Left operand is not a number: {:?}", left))?;
        let b = right.as_f64().ok_or_else(|| format!("Right operand is not a number: {:?}", right))?;
        Ok(Value::Bool(op(a, b)))
    }
    
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }
    
    fn apply_filter(&self, value: &Value, filter: &PipeFilter) -> Result<Value, String> {
        // Evaluate filter arguments
        let args: Result<Vec<Value>, String> = filter.args
            .iter()
            .map(|arg| self.evaluate(arg))
            .collect();
        let args = args?;
        
        // Use the filters module for all filter operations
        filters::apply_filter(value, &filter.name, &args)
    }
    
    fn register_builtin_functions(&mut self) {
        // Math functions
        self.register_function("abs", |args| {
            if args.len() != 1 {
                return Err("abs requires 1 argument".to_string());
            }
            let n = args[0].as_f64().ok_or("abs requires number")?;
            Ok(Value::Number(Number::from_f64(n.abs()).ok_or("Invalid number")?))
        });
        
        self.register_function("max", |args| {
            if args.is_empty() {
                return Err("max requires at least 1 argument".to_string());
            }
            let mut max = args[0].as_f64().ok_or("max requires numbers")?;
            for arg in &args[1..] {
                let n = arg.as_f64().ok_or("max requires numbers")?;
                if n > max {
                    max = n;
                }
            }
            Ok(Value::Number(Number::from_f64(max).ok_or("Invalid number")?))
        });
        
        self.register_function("min", |args| {
            if args.is_empty() {
                return Err("min requires at least 1 argument".to_string());
            }
            let mut min = args[0].as_f64().ok_or("min requires numbers")?;
            for arg in &args[1..] {
                let n = arg.as_f64().ok_or("min requires numbers")?;
                if n < min {
                    min = n;
                }
            }
            Ok(Value::Number(Number::from_f64(min).ok_or("Invalid number")?))
        });
        
        // String functions
        self.register_function("len", |args| {
            if args.len() != 1 {
                return Err("len requires 1 argument".to_string());
            }
            let len = match &args[0] {
                Value::String(s) => s.len(),
                Value::Array(a) => a.len(),
                Value::Object(o) => o.len(),
                _ => return Err("len requires string, array, or object".to_string()),
            };
            Ok(Value::Number(Number::from(len)))
        });
        
        // UUID functions
        self.register_function("uuid", |args| {
            if !args.is_empty() {
                return Err("uuid requires no arguments".to_string());
            }
            Ok(Value::String(helpers::generate_uuid()))
        });
        
        // Hashing functions
        self.register_function("hash_password", |args| {
            if args.len() != 1 {
                return Err("hash_password requires 1 argument".to_string());
            }
            let password = args[0].as_str().ok_or("hash_password requires string")?;
            Ok(Value::String(helpers::hash_password(password)))
        });
        
        self.register_function("md5", |args| {
            if args.len() != 1 {
                return Err("md5 requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("md5 requires string")?;
            Ok(Value::String(helpers::md5_hash(text)))
        });
        
        // DateTime functions
        self.register_function("now", |args| {
            if !args.is_empty() {
                return Err("now requires no arguments".to_string());
            }
            Ok(Value::String(helpers::now_iso()))
        });
        
        self.register_function("now_unix", |args| {
            if !args.is_empty() {
                return Err("now_unix requires no arguments".to_string());
            }
            Ok(Value::Number(Number::from(helpers::now_unix())))
        });
        
        self.register_function("today", |args| {
            if !args.is_empty() {
                return Err("today requires no arguments".to_string());
            }
            Ok(Value::String(helpers::today()))
        });
        
        self.register_function("now_time", |args| {
            if !args.is_empty() {
                return Err("now_time requires no arguments".to_string());
            }
            Ok(Value::String(helpers::now_time()))
        });
        
        self.register_function("add_days", |args| {
            if args.len() != 2 {
                return Err("add_days requires 2 arguments".to_string());
            }
            let timestamp = args[0].as_str().ok_or("add_days requires string timestamp")?;
            let days = args[1].as_i64().ok_or("add_days requires integer days")? as i32;
            helpers::add_days(timestamp, days)
                .map(Value::String)
                .map_err(|e| e.to_string())
        });
        
        self.register_function("add_hours", |args| {
            if args.len() != 2 {
                return Err("add_hours requires 2 arguments".to_string());
            }
            let timestamp = args[0].as_str().ok_or("add_hours requires string timestamp")?;
            let hours = args[1].as_i64().ok_or("add_hours requires integer hours")? as i32;
            helpers::add_hours(timestamp, hours)
                .map(Value::String)
                .map_err(|e| e.to_string())
        });
        
        self.register_function("format_timestamp", |args| {
            if args.len() != 2 {
                return Err("format_timestamp requires 2 arguments".to_string());
            }
            let timestamp = args[0].as_str().ok_or("format_timestamp requires string timestamp")?;
            let format = args[1].as_str().ok_or("format_timestamp requires string format")?;
            helpers::format_timestamp(timestamp, format)
                .map(Value::String)
                .map_err(|e| e.to_string())
        });
        
        self.register_function("parse_timestamp", |args| {
            if args.len() != 1 {
                return Err("parse_timestamp requires 1 argument".to_string());
            }
            let timestamp = args[0].as_str().ok_or("parse_timestamp requires string")?;
            helpers::parse_iso_timestamp(timestamp)
                .map(|ts| Value::Number(Number::from(ts)))
                .map_err(|e| e.to_string())
        });
        
        // Random functions
        self.register_function("random_int", |args| {
            if args.len() != 2 {
                return Err("random_int requires 2 arguments".to_string());
            }
            let min = args[0].as_i64().ok_or("random_int requires integer min")?;
            let max = args[1].as_i64().ok_or("random_int requires integer max")?;
            Ok(Value::Number(Number::from(helpers::random_int(min, max))))
        });
        
        self.register_function("random_float", |args| {
            if args.len() != 2 {
                return Err("random_float requires 2 arguments".to_string());
            }
            let min = args[0].as_f64().ok_or("random_float requires number min")?;
            let max = args[1].as_f64().ok_or("random_float requires number max")?;
            Ok(Value::Number(Number::from_f64(helpers::random_float(min, max)).ok_or("Invalid number")?))
        });
        
        self.register_function("random_string", |args| {
            if args.len() != 1 {
                return Err("random_string requires 1 argument".to_string());
            }
            let length = args[0].as_u64().ok_or("random_string requires positive integer")? as usize;
            Ok(Value::String(helpers::random_string(length)))
        });
        
        // Encoding functions
        self.register_function("base64_encode", |args| {
            if args.len() != 1 {
                return Err("base64_encode requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("base64_encode requires string")?;
            Ok(Value::String(helpers::base64_encode(text)))
        });
        
        self.register_function("base64_decode", |args| {
            if args.len() != 1 {
                return Err("base64_decode requires 1 argument".to_string());
            }
            let encoded = args[0].as_str().ok_or("base64_decode requires string")?;
            helpers::base64_decode(encoded)
                .map(Value::String)
                .map_err(|e| e.to_string())
        });
        
        self.register_function("url_encode", |args| {
            if args.len() != 1 {
                return Err("url_encode requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("url_encode requires string")?;
            Ok(Value::String(helpers::url_encode(text)))
        });
        
        self.register_function("url_decode", |args| {
            if args.len() != 1 {
                return Err("url_decode requires 1 argument".to_string());
            }
            let encoded = args[0].as_str().ok_or("url_decode requires string")?;
            helpers::url_decode(encoded)
                .map(Value::String)
                .map_err(|e| e.to_string())
        });
        
        self.register_function("html_escape", |args| {
            if args.len() != 1 {
                return Err("html_escape requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("html_escape requires string")?;
            Ok(Value::String(helpers::html_escape(text)))
        });
        
        self.register_function("html_unescape", |args| {
            if args.len() != 1 {
                return Err("html_unescape requires 1 argument".to_string());
            }
            let escaped = args[0].as_str().ok_or("html_unescape requires string")?;
            Ok(Value::String(helpers::html_unescape(escaped)))
        });
        
        // String utility functions
        self.register_function("slugify", |args| {
            if args.len() != 1 {
                return Err("slugify requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("slugify requires string")?;
            Ok(Value::String(helpers::slugify(text)))
        });
        
        self.register_function("truncate", |args| {
            if args.len() != 2 {
                return Err("truncate requires 2 arguments".to_string());
            }
            let text = args[0].as_str().ok_or("truncate requires string")?;
            let max_len = args[1].as_u64().ok_or("truncate requires positive integer")? as usize;
            Ok(Value::String(helpers::truncate(text, max_len)))
        });
        
        self.register_function("word_count", |args| {
            if args.len() != 1 {
                return Err("word_count requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("word_count requires string")?;
            Ok(Value::Number(Number::from(helpers::word_count(text))))
        });
        
        self.register_function("initials", |args| {
            if args.len() != 1 {
                return Err("initials requires 1 argument".to_string());
            }
            let name = args[0].as_str().ok_or("initials requires string")?;
            Ok(Value::String(helpers::initials(name)))
        });
        
        // Email utility functions
        self.register_function("email_domain", |args| {
            if args.len() != 1 {
                return Err("email_domain requires 1 argument".to_string());
            }
            let email = args[0].as_str().ok_or("email_domain requires string")?;
            helpers::email_domain(email)
                .map(Value::String)
                .ok_or_else(|| "Invalid email format".to_string())
        });
        
        self.register_function("email_username", |args| {
            if args.len() != 1 {
                return Err("email_username requires 1 argument".to_string());
            }
            let email = args[0].as_str().ok_or("email_username requires string")?;
            helpers::email_username(email)
                .map(Value::String)
                .ok_or_else(|| "Invalid email format".to_string())
        });
        
        self.register_function("is_email", |args| {
            if args.len() != 1 {
                return Err("is_email requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("is_email requires string")?;
            Ok(Value::Bool(helpers::is_email(text)))
        });
        
        self.register_function("is_url", |args| {
            if args.len() != 1 {
                return Err("is_url requires 1 argument".to_string());
            }
            let text = args[0].as_str().ok_or("is_url requires string")?;
            Ok(Value::Bool(helpers::is_url(text)))
        });
        
        // JSON utility functions
        self.register_function("json_parse", |args| {
            if args.len() != 1 {
                return Err("json_parse requires 1 argument".to_string());
            }
            let json_str = args[0].as_str().ok_or("json_parse requires string")?;
            helpers::json_parse(json_str)
                .map_err(|e| e.to_string())
        });
        
        self.register_function("json_stringify", |args| {
            if args.len() != 1 {
                return Err("json_stringify requires 1 argument".to_string());
            }
            helpers::json_stringify(&args[0])
                .map(Value::String)
                .map_err(|e| e.to_string())
        });
        
        self.register_function("json_pretty", |args| {
            if args.len() != 1 {
                return Err("json_pretty requires 1 argument".to_string());
            }
            helpers::json_pretty(&args[0])
                .map(Value::String)
                .map_err(|e| e.to_string())
        });
        
        // Formatting functions
        self.register_function("format_number", |args| {
            if args.is_empty() || args.len() > 2 {
                return Err("format_number requires 1 or 2 arguments".to_string());
            }
            let number = args[0].as_f64().ok_or("format_number requires number")?;
            let decimals = if args.len() == 2 {
                args[1].as_u64().ok_or("format_number decimals requires positive integer")? as usize
            } else {
                2
            };
            Ok(Value::String(helpers::format_number(number, decimals)))
        });
        
        self.register_function("format_bytes", |args| {
            if args.len() != 1 {
                return Err("format_bytes requires 1 argument".to_string());
            }
            let bytes = args[0].as_u64().ok_or("format_bytes requires positive integer")?;
            Ok(Value::String(helpers::format_bytes(bytes)))
        });
    }
    
    fn register_function<F>(&mut self, name: &str, func: F)
    where
        F: Fn(Vec<Value>) -> Result<Value, String> + 'static,
    {
        self.functions.insert(name.to_string(), Box::new(func));
    }
    
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{Tokenizer, Parser};
    
    fn eval(input: &str) -> Result<Value, String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;
        let evaluator = Evaluator::new();
        evaluator.evaluate(&expr)
    }
    
    fn eval_with_vars(input: &str, vars: HashMap<String, Value>) -> Result<Value, String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;
        let evaluator = Evaluator::with_variables(vars);
        evaluator.evaluate(&expr)
    }
    
    #[test]
    fn test_eval_number() {
        let result = eval("42").unwrap();
        assert_eq!(result.as_f64().unwrap(), 42.0);
    }
    
    #[test]
    fn test_eval_string() {
        let result = eval(r#""hello""#).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_eval_boolean() {
        assert_eq!(eval("true").unwrap(), Value::Bool(true));
        assert_eq!(eval("false").unwrap(), Value::Bool(false));
    }
    
    #[test]
    fn test_eval_null() {
        assert_eq!(eval("null").unwrap(), Value::Null);
    }
    
    #[test]
    fn test_eval_arithmetic() {
        assert_eq!(eval("2 + 3").unwrap().as_f64().unwrap(), 5.0);
        assert_eq!(eval("10 - 4").unwrap().as_f64().unwrap(), 6.0);
        assert_eq!(eval("3 * 4").unwrap().as_f64().unwrap(), 12.0);
        assert_eq!(eval("15 / 3").unwrap().as_f64().unwrap(), 5.0);
        assert_eq!(eval("10 % 3").unwrap().as_f64().unwrap(), 1.0);
    }
    
    #[test]
    fn test_eval_power() {
        let result = eval("2 ** 3").unwrap();
        assert_eq!(result.as_f64().unwrap(), 8.0);
    }
    
    #[test]
    fn test_eval_precedence() {
        assert_eq!(eval("2 + 3 * 4").unwrap().as_f64().unwrap(), 14.0);
        assert_eq!(eval("(2 + 3) * 4").unwrap().as_f64().unwrap(), 20.0);
    }
    
    #[test]
    fn test_eval_comparison() {
        assert_eq!(eval("5 > 3").unwrap(), Value::Bool(true));
        assert_eq!(eval("5 < 3").unwrap(), Value::Bool(false));
        assert_eq!(eval("5 >= 5").unwrap(), Value::Bool(true));
        assert_eq!(eval("5 <= 4").unwrap(), Value::Bool(false));
    }
    
    #[test]
    fn test_eval_equality() {
        assert_eq!(eval("5 == 5").unwrap(), Value::Bool(true));
        assert_eq!(eval("5 == 3").unwrap(), Value::Bool(false));
        assert_eq!(eval("5 != 3").unwrap(), Value::Bool(true));
    }
    
    #[test]
    fn test_eval_logical() {
        assert_eq!(eval("true && true").unwrap(), Value::Bool(true));
        assert_eq!(eval("true && false").unwrap(), Value::Bool(false));
        assert_eq!(eval("false || true").unwrap(), Value::Bool(true));
        assert_eq!(eval("false || false").unwrap(), Value::Bool(false));
    }
    
    #[test]
    fn test_eval_unary() {
        assert_eq!(eval("-5").unwrap().as_f64().unwrap(), -5.0);
        assert_eq!(eval("!true").unwrap(), Value::Bool(false));
        assert_eq!(eval("!false").unwrap(), Value::Bool(true));
    }
    
    #[test]
    fn test_eval_variable() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Value::Number(Number::from(42)));
        
        let result = eval_with_vars("x", vars).unwrap();
        assert_eq!(result.as_f64().unwrap(), 42.0);
    }
    
    #[test]
    fn test_eval_undefined_variable() {
        let result = eval("undefined_var");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }
    
    #[test]
    fn test_eval_string_concatenation() {
        let result = eval(r#""hello" + " " + "world""#).unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }
    
    #[test]
    fn test_eval_array() {
        let result = eval("[1, 2, 3]").unwrap();
        assert!(result.is_array());
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_f64().unwrap(), 1.0);
    }
    
    #[test]
    fn test_eval_object() {
        let result = eval(r#"{name: "John", age: 30}"#).unwrap();
        assert!(result.is_object());
        let obj = result.as_object().unwrap();
        assert_eq!(obj.get("name").unwrap(), &Value::String("John".to_string()));
        assert_eq!(obj.get("age").unwrap().as_f64().unwrap(), 30.0);
    }
    
    #[test]
    fn test_eval_ternary() {
        assert_eq!(eval("true ? 1 : 2").unwrap().as_f64().unwrap(), 1.0);
        assert_eq!(eval("false ? 1 : 2").unwrap().as_f64().unwrap(), 2.0);
        assert_eq!(eval("5 > 3 ? 10 : 20").unwrap().as_f64().unwrap(), 10.0);
    }
    
    #[test]
    fn test_eval_function_abs() {
        assert_eq!(eval("abs(-5)").unwrap().as_f64().unwrap(), 5.0);
        assert_eq!(eval("abs(3)").unwrap().as_f64().unwrap(), 3.0);
    }
    
    #[test]
    fn test_eval_function_max() {
        assert_eq!(eval("max(1, 5, 3)").unwrap().as_f64().unwrap(), 5.0);
        assert_eq!(eval("max(10, 20)").unwrap().as_f64().unwrap(), 20.0);
    }
    
    #[test]
    fn test_eval_function_min() {
        assert_eq!(eval("min(1, 5, 3)").unwrap().as_f64().unwrap(), 1.0);
        assert_eq!(eval("min(10, 20)").unwrap().as_f64().unwrap(), 10.0);
    }
    
    #[test]
    fn test_eval_function_len() {
        assert_eq!(eval(r#"len("hello")"#).unwrap(), Value::Number(Number::from(5)));
        assert_eq!(eval("len([1, 2, 3])").unwrap(), Value::Number(Number::from(3)));
    }
    
    #[test]
    fn test_eval_pipe_upper() {
        let result = eval(r#""hello" | upper"#).unwrap();
        assert_eq!(result, Value::String("HELLO".to_string()));
    }
    
    #[test]
    fn test_eval_pipe_lower() {
        let result = eval(r#""WORLD" | lower"#).unwrap();
        assert_eq!(result, Value::String("world".to_string()));
    }
    
    #[test]
    fn test_eval_pipe_trim() {
        let result = eval(r#""  hello  " | trim"#).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_eval_pipe_chain() {
        let result = eval(r#""  HELLO  " | trim | lower"#).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_eval_complex_expression() {
        let mut vars = HashMap::new();
        vars.insert("price".to_string(), Value::Number(Number::from(100)));
        vars.insert("quantity".to_string(), Value::Number(Number::from(3)));
        vars.insert("discount".to_string(), Value::Number(Number::from_f64(0.1).unwrap()));
        
        let result = eval_with_vars("price * quantity * (1 - discount)", vars).unwrap();
        assert_eq!(result.as_f64().unwrap(), 270.0);
    }
    
    #[test]
    fn test_eval_truthy_values() {
        let evaluator = Evaluator::new();
        assert!(evaluator.is_truthy(&Value::Bool(true)));
        assert!(!evaluator.is_truthy(&Value::Bool(false)));
        assert!(!evaluator.is_truthy(&Value::Null));
        assert!(!evaluator.is_truthy(&Value::Number(Number::from(0))));
        assert!(evaluator.is_truthy(&Value::Number(Number::from(1))));
        assert!(!evaluator.is_truthy(&Value::String("".to_string())));
        assert!(evaluator.is_truthy(&Value::String("hello".to_string())));
    }
    
    // Advanced filter integration tests
    #[test]
    fn test_eval_pipe_replace() {
        let result = eval(r#""hello world" | replace("world", "rust")"#).unwrap();
        assert_eq!(result, Value::String("hello rust".to_string()));
    }
    
    #[test]
    fn test_eval_pipe_split() {
        let result = eval(r#""a,b,c" | split(",")"#).unwrap();
        assert_eq!(result.as_array().unwrap().len(), 3);
    }
    
    #[test]
    fn test_eval_pipe_capitalize() {
        let result = eval(r#""hello" | capitalize"#).unwrap();
        assert_eq!(result, Value::String("Hello".to_string()));
    }
    
    #[test]
    fn test_eval_pipe_array_operations() {
        let result = eval(r#"[3, 1, 2] | sort | first"#).unwrap();
        assert_eq!(result.as_f64().unwrap(), 1.0);
        
        let result = eval(r#"[1, 2, 3] | last"#).unwrap();
        assert_eq!(result.as_f64().unwrap(), 3.0);
    }
    
    #[test]
    fn test_eval_pipe_join() {
        let result = eval(r#"["a", "b", "c"] | join("-")"#).unwrap();
        assert_eq!(result, Value::String("a-b-c".to_string()));
    }
    
    #[test]
    fn test_eval_pipe_keys_values() {
        let result = eval(r#"{a: 1, b: 2} | keys | length"#).unwrap();
        assert_eq!(result.as_u64().unwrap(), 2);
        
        let result = eval(r#"{a: 1, b: 2} | values | first"#).unwrap();
        assert!(result.is_number());
    }
    
    #[test]
    fn test_eval_pipe_math_filters() {
        let result = eval(r#"-5.7 | abs | round"#).unwrap();
        assert_eq!(result.as_f64().unwrap(), 6.0);
        
        let result = eval(r#"3.2 | floor"#).unwrap();
        assert_eq!(result.as_f64().unwrap(), 3.0);
        
        let result = eval(r#"3.2 | ceil"#).unwrap();
        assert_eq!(result.as_f64().unwrap(), 4.0);
    }
    
    #[test]
    fn test_eval_pipe_type_conversion() {
        let result = eval(r#"42 | string | length"#).unwrap();
        assert_eq!(result.as_u64().unwrap(), 2);
        
        let result = eval(r#""123" | number"#).unwrap();
        assert_eq!(result.as_f64().unwrap(), 123.0);
        
        let result = eval(r#""" | bool"#).unwrap();
        assert_eq!(result, Value::Bool(false));
    }
    
    #[test]
    fn test_eval_complex_pipe_chain() {
        let result = eval(r#""  HELLO WORLD  " | trim | lower | split(" ") | join("-")"#).unwrap();
        assert_eq!(result, Value::String("hello-world".to_string()));
    }
    
    #[test]
    fn test_eval_pipe_with_variables() {
        let mut vars = HashMap::new();
        vars.insert("email".to_string(), Value::String("  USER@EXAMPLE.COM  ".to_string()));
        
        let result = eval_with_vars("email | trim | lower", vars).unwrap();
        assert_eq!(result, Value::String("user@example.com".to_string()));
    }
}
