// Expression template resolver for YAML routes
// Resolves {{variable}} and {{expression | filter}} in route definitions

use crate::expression::{Tokenizer, Parser, Evaluator};
use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;

/// Resolve all expression templates in a JSON value
/// Templates: {{variable}}, {{expr}}, {{expr | filter}}
pub fn resolve_templates(value: &Value, context: &HashMap<String, Value>) -> Result<Value, String> {
    match value {
        Value::String(s) => resolve_string_template(s, context),
        Value::Array(arr) => {
            let resolved: Result<Vec<Value>, String> = arr
                .iter()
                .map(|v| resolve_templates(v, context))
                .collect();
            Ok(Value::Array(resolved?))
        }
        Value::Object(obj) => {
            let mut resolved = serde_json::Map::new();
            for (key, val) in obj {
                resolved.insert(key.clone(), resolve_templates(val, context)?);
            }
            Ok(Value::Object(resolved))
        }
        _ => Ok(value.clone()),
    }
}

/// Resolve expressions in a string template
/// Examples:
///   "Hello {{name}}" → "Hello John"
///   "Total: {{price * quantity}}" → "Total: 150"
///   "Email: {{email | lower | trim}}" → "Email: user@example.com"
fn resolve_string_template(template: &str, context: &HashMap<String, Value>) -> Result<Value, String> {
    // Check if entire string is a single expression
    if template.starts_with("{{") && template.ends_with("}}") && template.matches("{{").count() == 1 {
        // Pure expression: return evaluated value (not string)
        let expr_content = &template[2..template.len()-2].trim();
        return evaluate_expression(expr_content, context);
    }
    
    // Mixed template: replace all {{...}} with evaluated strings
    let re = Regex::new(r"\{\{(.+?)\}\}").map_err(|e| e.to_string())?;
    
    let mut result = template.to_string();
    for cap in re.captures_iter(template) {
        let full_match = cap.get(0).unwrap().as_str();
        let expr_content = cap.get(1).unwrap().as_str().trim();
        
        let value = evaluate_expression(expr_content, context)?;
        let replacement = value_to_string(&value);
        
        result = result.replace(full_match, &replacement);
    }
    
    Ok(Value::String(result))
}

/// Evaluate an expression with context
pub fn evaluate_expression(expr_str: &str, context: &HashMap<String, Value>) -> Result<Value, String> {
    // Tokenize
    let mut tokenizer = Tokenizer::new(expr_str);
    let tokens = tokenizer.tokenize()
        .map_err(|e| format!("Tokenization error: {}", e))?;
    
    // Parse
    let mut parser = Parser::new(tokens);
    let expr = parser.parse()
        .map_err(|e| format!("Parse error: {}", e))?;
    
    // Evaluate
    let evaluator = Evaluator::with_variables(context.clone());
    evaluator.evaluate(&expr)
        .map_err(|e| format!("Evaluation error: {}", e))
}

/// Convert a Value to string for template replacement
fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            // Format numbers cleanly
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else if let Some(u) = n.as_u64() {
                u.to_string()
            } else if let Some(f) = n.as_f64() {
                if f.fract() == 0.0 {
                    (f as i64).to_string()
                } else {
                    f.to_string()
                }
            } else {
                n.to_string()
            }
        }
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string(value).unwrap_or_default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_resolve_simple_variable() {
        let mut context = HashMap::new();
        context.insert("name".to_string(), json!("John"));
        
        let result = resolve_string_template("Hello {{name}}", &context).unwrap();
        assert_eq!(result, json!("Hello John"));
    }
    
    #[test]
    fn test_resolve_pure_expression() {
        let mut context = HashMap::new();
        context.insert("price".to_string(), json!(10));
        context.insert("quantity".to_string(), json!(5));
        
        // Pure expression returns number, not string
        let result = resolve_string_template("{{price * quantity}}", &context).unwrap();
        assert_eq!(result.as_f64().unwrap(), 50.0);
    }
    
    #[test]
    fn test_resolve_mixed_template() {
        let mut context = HashMap::new();
        context.insert("name".to_string(), json!("John"));
        context.insert("age".to_string(), json!(30));
        
        let result = resolve_string_template("{{name}} is {{age}} years old", &context).unwrap();
        assert_eq!(result, json!("John is 30 years old"));
    }
    
    #[test]
    fn test_resolve_pipe_expression() {
        let mut context = HashMap::new();
        context.insert("email".to_string(), json!("  USER@EXAMPLE.COM  "));
        
        let result = resolve_string_template("{{email | trim | lower}}", &context).unwrap();
        assert_eq!(result, json!("user@example.com"));
    }
    
    #[test]
    fn test_resolve_arithmetic() {
        let mut context = HashMap::new();
        context.insert("a".to_string(), json!(10));
        context.insert("b".to_string(), json!(20));
        
        let result = resolve_string_template("Sum: {{a + b}}", &context).unwrap();
        assert_eq!(result, json!("Sum: 30"));
    }
    
    #[test]
    fn test_resolve_array() {
        let mut context = HashMap::new();
        context.insert("items".to_string(), json!([1, 2, 3]));
        
        let input = json!({
            "count": "{{items | length}}",
            "first": "{{items | first}}"
        });
        
        let result = resolve_templates(&input, &context).unwrap();
        assert_eq!(result["count"].as_f64().unwrap(), 3.0);
        assert_eq!(result["first"].as_f64().unwrap(), 1.0);
    }
    
    #[test]
    fn test_resolve_nested_object() {
        let mut context = HashMap::new();
        context.insert("name".to_string(), json!("John"));
        context.insert("price".to_string(), json!(100));
        context.insert("qty".to_string(), json!(2));
        
        let input = json!({
            "user": "{{name}}",
            "order": {
                "total": "{{price * qty}}",
                "message": "Total for {{name}}: {{price * qty}}"
            }
        });
        
        let result = resolve_templates(&input, &context).unwrap();
        assert_eq!(result["user"], json!("John"));
        assert_eq!(result["order"]["total"].as_f64().unwrap(), 200.0);
        assert_eq!(result["order"]["message"], json!("Total for John: 200"));
    }
    
    #[test]
    fn test_resolve_with_functions() {
        let mut context = HashMap::new();
        context.insert("nums".to_string(), json!([1, 5, 3]));
        
        let result = resolve_string_template("Max: {{max(1, 5, 3)}}", &context).unwrap();
        assert_eq!(result, json!("Max: 5"));
    }
    
    #[test]
    fn test_resolve_ternary() {
        let mut context = HashMap::new();
        context.insert("age".to_string(), json!(20));
        
        let result = resolve_string_template("{{age >= 18 ? \"adult\" : \"minor\"}}", &context).unwrap();
        assert_eq!(result, json!("adult"));
    }
    
    #[test]
    fn test_resolve_complex_chain() {
        let mut context = HashMap::new();
        context.insert("text".to_string(), json!("  hello WORLD  "));
        
        let result = resolve_string_template("{{text | trim | lower | capitalize}}", &context).unwrap();
        assert_eq!(result, json!("Hello world"));
    }
    
    #[test]
    fn test_undefined_variable_error() {
        let context = HashMap::new();
        
        let result = resolve_string_template("{{undefined_var}}", &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }
    
    #[test]
    fn test_resolve_array_value() {
        let mut context = HashMap::new();
        context.insert("data".to_string(), json!("a,b,c"));
        
        let result = resolve_string_template("{{data | split(\",\")}}", &context).unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().unwrap().len(), 3);
    }
}
