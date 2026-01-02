//! SQL Named Parameters Parser and Replacer
//!
//! Phase 3 Day 14: Support for named parameters in SQL queries
//!
//! Features:
//! - Parse :param_name syntax in SQL
//! - Replace with actual values from context
//! - Support expression evaluation in parameters
//! - Type-safe parameter binding

use serde_json::Value;
use std::collections::HashMap;
use crate::expression::{Tokenizer, Parser, Evaluator};

/// Represents a SQL query with named parameters
#[derive(Debug, Clone)]
pub struct NamedQuery {
    /// Original SQL with named parameters
    pub original: String,
    /// SQL with parameters replaced by positional placeholders (?)
    pub query: String,
    /// Parameter names in order of appearance
    pub param_names: Vec<String>,
}

impl NamedQuery {
    /// Parse SQL query and extract named parameters
    /// 
    /// # Example
    /// ```
    /// use worpen_core::sql_params::NamedQuery;
    /// 
    /// let query = "SELECT * FROM users WHERE email = :email AND age > :min_age";
    /// let named = NamedQuery::parse(query);
    /// 
    /// assert_eq!(named.param_names, vec!["email", "min_age"]);
    /// assert_eq!(named.query, "SELECT * FROM users WHERE email = ? AND age > ?");
    /// ```
    pub fn parse(sql: &str) -> Self {
        let mut query = String::new();
        let mut param_names = Vec::new();
        let mut chars = sql.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == ':' {
                // Check if next char is alphanumeric (start of parameter name)
                if let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphabetic() || next_ch == '_' {
                        // Extract parameter name
                        let mut param_name = String::new();
                        while let Some(&ch) = chars.peek() {
                            if ch.is_alphanumeric() || ch == '_' {
                                param_name.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        
                        param_names.push(param_name);
                        query.push('?');
                        continue;
                    }
                }
            }
            
            query.push(ch);
        }
        
        Self {
            original: sql.to_string(),
            query,
            param_names,
        }
    }
    
    /// Bind parameters from context values
    /// 
    /// # Example
    /// ```
    /// use worpen_core::sql_params::NamedQuery;
    /// use serde_json::json;
    /// use std::collections::HashMap;
    /// 
    /// let query = NamedQuery::parse("SELECT * FROM users WHERE id = :user_id");
    /// let mut params = HashMap::new();
    /// params.insert("user_id".to_string(), json!(123));
    /// 
    /// let values = query.bind_params(&params).unwrap();
    /// assert_eq!(values, vec![json!(123)]);
    /// ```
    pub fn bind_params(&self, params: &HashMap<String, Value>) -> Result<Vec<Value>, String> {
        let mut values = Vec::new();
        
        for param_name in &self.param_names {
            let value = params.get(param_name)
                .ok_or_else(|| format!("Missing parameter: {}", param_name))?;
            values.push(value.clone());
        }
        
        Ok(values)
    }
    
    /// Bind parameters with expression evaluation
    /// 
    /// Parameters can be expressions like "{{ user.email }}" or "{{ now() }}"
    /// 
    /// # Example
    /// ```
    /// use worpen_core::sql_params::NamedQuery;
    /// use serde_json::json;
    /// use std::collections::HashMap;
    /// 
    /// let query = NamedQuery::parse("INSERT INTO logs (message, created_at) VALUES (:msg, :time)");
    /// let mut params = HashMap::new();
    /// params.insert("msg".to_string(), json!("{{ 'Hello ' + name }}"));
    /// params.insert("time".to_string(), json!("{{ now() }}"));
    /// 
    /// let mut context = HashMap::new();
    /// context.insert("name".to_string(), json!("Alice"));
    /// 
    /// let values = query.bind_params_with_eval(&params, &context).unwrap();
    /// // values[0] will be evaluated expression result
    /// ```
    pub fn bind_params_with_eval(
        &self,
        params: &HashMap<String, Value>,
        context: &HashMap<String, Value>,
    ) -> Result<Vec<Value>, String> {
        let mut values = Vec::new();
        
        for param_name in &self.param_names {
            let param_value = params.get(param_name)
                .ok_or_else(|| format!("Missing parameter: {}", param_name))?;
            
            // Check if value is a string expression
            let evaluated = if let Some(expr_str) = param_value.as_str() {
                if is_expression(expr_str) {
                    // Evaluate expression
                    evaluate_expression(expr_str, context)?
                } else {
                    param_value.clone()
                }
            } else {
                param_value.clone()
            };
            
            values.push(evaluated);
        }
        
        Ok(values)
    }
}

/// Check if string is an expression (contains {{ }})
fn is_expression(s: &str) -> bool {
    s.trim().starts_with("{{") && s.trim().ends_with("}}")
}

/// Evaluate an expression string
fn evaluate_expression(expr_str: &str, context: &HashMap<String, Value>) -> Result<Value, String> {
    // Remove {{ and }}
    let expr = expr_str.trim()
        .trim_start_matches("{{")
        .trim_end_matches("}}")
        .trim();
    
    // Tokenize and parse
    let mut tokenizer = Tokenizer::new(expr);
    let tokens = tokenizer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    // Evaluate with context
    let evaluator = Evaluator::with_variables(context.clone());
    evaluator.evaluate(&ast)
}

/// SQL Parameter Builder - fluent interface for building queries
pub struct SqlParamBuilder {
    sql: String,
    params: HashMap<String, Value>,
}

impl SqlParamBuilder {
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            params: HashMap::new(),
        }
    }
    
    pub fn param(mut self, name: impl Into<String>, value: Value) -> Self {
        self.params.insert(name.into(), value);
        self
    }
    
    pub fn build(self) -> (NamedQuery, HashMap<String, Value>) {
        (NamedQuery::parse(&self.sql), self.params)
    }
    
    pub fn build_with_eval(
        self,
        context: &HashMap<String, Value>,
    ) -> Result<(String, Vec<Value>), String> {
        let query = NamedQuery::parse(&self.sql);
        let values = query.bind_params_with_eval(&self.params, context)?;
        Ok((query.query, values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_parse_simple_query() {
        let sql = "SELECT * FROM users WHERE id = :id";
        let named = NamedQuery::parse(sql);
        
        assert_eq!(named.param_names, vec!["id"]);
        assert_eq!(named.query, "SELECT * FROM users WHERE id = ?");
    }
    
    #[test]
    fn test_parse_multiple_params() {
        let sql = "SELECT * FROM users WHERE email = :email AND age > :min_age";
        let named = NamedQuery::parse(sql);
        
        assert_eq!(named.param_names, vec!["email", "min_age"]);
        assert_eq!(named.query, "SELECT * FROM users WHERE email = ? AND age > ?");
    }
    
    #[test]
    fn test_parse_insert_query() {
        let sql = "INSERT INTO users (name, email, created_at) VALUES (:name, :email, :created_at)";
        let named = NamedQuery::parse(sql);
        
        assert_eq!(named.param_names, vec!["name", "email", "created_at"]);
        assert_eq!(named.query, "INSERT INTO users (name, email, created_at) VALUES (?, ?, ?)");
    }
    
    #[test]
    fn test_parse_update_query() {
        let sql = "UPDATE users SET name = :name, email = :email WHERE id = :id";
        let named = NamedQuery::parse(sql);
        
        assert_eq!(named.param_names, vec!["name", "email", "id"]);
        assert_eq!(named.query, "UPDATE users SET name = ?, email = ? WHERE id = ?");
    }
    
    #[test]
    fn test_parse_no_params() {
        let sql = "SELECT * FROM users";
        let named = NamedQuery::parse(sql);
        
        assert!(named.param_names.is_empty());
        assert_eq!(named.query, sql);
    }
    
    #[test]
    fn test_parse_with_colons_in_strings() {
        let sql = "SELECT * FROM users WHERE note = 'Time: 10:30' AND id = :id";
        let named = NamedQuery::parse(sql);
        
        assert_eq!(named.param_names, vec!["id"]);
        assert!(named.query.contains("Time: 10:30"));
    }
    
    #[test]
    fn test_bind_params_simple() {
        let query = NamedQuery::parse("SELECT * FROM users WHERE id = :id");
        let mut params = HashMap::new();
        params.insert("id".to_string(), json!(123));
        
        let values = query.bind_params(&params).unwrap();
        assert_eq!(values, vec![json!(123)]);
    }
    
    #[test]
    fn test_bind_params_multiple() {
        let query = NamedQuery::parse("SELECT * FROM users WHERE email = :email AND age > :age");
        let mut params = HashMap::new();
        params.insert("email".to_string(), json!("test@example.com"));
        params.insert("age".to_string(), json!(18));
        
        let values = query.bind_params(&params).unwrap();
        assert_eq!(values, vec![json!("test@example.com"), json!(18)]);
    }
    
    #[test]
    fn test_bind_params_missing() {
        let query = NamedQuery::parse("SELECT * FROM users WHERE id = :id");
        let params = HashMap::new();
        
        let result = query.bind_params(&params);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing parameter: id"));
    }
    
    #[test]
    fn test_bind_params_with_eval_literal() {
        let query = NamedQuery::parse("SELECT * FROM users WHERE id = :id");
        let mut params = HashMap::new();
        params.insert("id".to_string(), json!(123));
        
        let context = HashMap::new();
        let values = query.bind_params_with_eval(&params, &context).unwrap();
        assert_eq!(values, vec![json!(123)]);
    }
    
    #[test]
    fn test_bind_params_with_eval_expression() {
        let query = NamedQuery::parse("SELECT * FROM users WHERE id = :user_id");
        let mut params = HashMap::new();
        params.insert("user_id".to_string(), json!("{{ user.id }}"));
        
        let mut context = HashMap::new();
        context.insert("user".to_string(), json!({"id": 456}));
        
        let values = query.bind_params_with_eval(&params, &context).unwrap();
        assert_eq!(values[0]["id"], json!(456));
    }
    
    #[test]
    fn test_sql_param_builder() {
        let (query, params) = SqlParamBuilder::new("SELECT * FROM users WHERE id = :id")
            .param("id", json!(123))
            .build();
        
        assert_eq!(query.param_names, vec!["id"]);
        assert_eq!(params.get("id"), Some(&json!(123)));
    }
    
    #[test]
    fn test_sql_param_builder_multiple() {
        let (query, params) = SqlParamBuilder::new(
            "INSERT INTO users (name, email) VALUES (:name, :email)"
        )
            .param("name", json!("Alice"))
            .param("email", json!("alice@example.com"))
            .build();
        
        assert_eq!(query.param_names, vec!["name", "email"]);
        assert_eq!(params.get("name"), Some(&json!("Alice")));
    }
    
    #[test]
    fn test_is_expression() {
        assert!(is_expression("{{ user.name }}"));
        assert!(is_expression("{{user.name}}"));
        assert!(is_expression("  {{ user.name }}  "));
        assert!(!is_expression("user.name"));
        assert!(!is_expression("{ user.name }"));
    }
}
