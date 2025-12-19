// Expression evaluator - evaluates AST to produce values
// Phase 2 Day 7-8: Full implementation

use crate::expression::ast::{Expr, BinaryOp, UnaryOp, PipeFilter};
use serde_json::{Value, Number};
use std::collections::HashMap;

pub struct Evaluator {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value, String>>>,
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
        // Pipe filters will be implemented in Phase 2 Day 8-9
        match filter.name.as_str() {
            "upper" => self.filter_upper(value),
            "lower" => self.filter_lower(value),
            "trim" => self.filter_trim(value),
            _ => Err(format!("Unknown filter: {}", filter.name)),
        }
    }
    
    // Basic pipe filters
    fn filter_upper(&self, value: &Value) -> Result<Value, String> {
        match value.as_str() {
            Some(s) => Ok(Value::String(s.to_uppercase())),
            None => Err(format!("upper filter requires string, got {:?}", value)),
        }
    }
    
    fn filter_lower(&self, value: &Value) -> Result<Value, String> {
        match value.as_str() {
            Some(s) => Ok(Value::String(s.to_lowercase())),
            None => Err(format!("lower filter requires string, got {:?}", value)),
        }
    }
    
    fn filter_trim(&self, value: &Value) -> Result<Value, String> {
        match value.as_str() {
            Some(s) => Ok(Value::String(s.trim().to_string())),
            None => Err(format!("trim filter requires string, got {:?}", value)),
        }
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
        assert_eq!(evaluator.is_truthy(&Value::Bool(true)), true);
        assert_eq!(evaluator.is_truthy(&Value::Bool(false)), false);
        assert_eq!(evaluator.is_truthy(&Value::Null), false);
        assert_eq!(evaluator.is_truthy(&Value::Number(Number::from(0))), false);
        assert_eq!(evaluator.is_truthy(&Value::Number(Number::from(1))), true);
        assert_eq!(evaluator.is_truthy(&Value::String("".to_string())), false);
        assert_eq!(evaluator.is_truthy(&Value::String("hello".to_string())), true);
    }
}
