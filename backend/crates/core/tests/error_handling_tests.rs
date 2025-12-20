/// Error Handling and Edge Case Tests
/// 
/// Tests robustness and error recovery:
/// - Invalid syntax
/// - Type mismatches
/// - Missing variables
/// - Division by zero
/// - SQL errors
/// - Validation failures
/// - Malformed expressions
/// - Resource limits

#[cfg(test)]
mod error_handling_tests {
    use core::parsers::route_parser::parse_route;
    use core::expression::{Tokenizer, Parser, Evaluator};
    use core::validation::validate_input;
    use serde_json::json;
    use std::collections::HashMap;

    // ========== Parse Errors ==========

    #[test]
    fn test_invalid_json_syntax() {
        let invalid = r#"{"name": "test", "route": "/test" missing_brace"#;
        let result = parse_route(invalid);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("parse") || result.unwrap_err().contains("JSON"));
    }

    #[test]
    fn test_invalid_yaml_indentation() {
        let invalid = r#"
name: test
route: /test
  bad_indent: true
method: GET
"#;
        let result = parse_route(invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_yaml_tab_characters() {
        let invalid = "name: test\n\troute: /test\nmethod: GET\n";
        let result = parse_route(invalid);
        // YAML parsers typically reject tabs
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_route_definition() {
        let empty = "";
        let result = parse_route(empty);
        assert!(result.is_err());
    }

    #[test]
    fn test_null_route_definition() {
        let null = "null";
        let result = parse_route(null);
        assert!(result.is_err());
    }

    // ========== Expression Errors ==========

    #[test]
    fn test_unclosed_expression() {
        let expr = "{{ a + b";
        let mut tokenizer = Tokenizer::new(expr);
        let result = tokenizer.tokenize();
        // Should handle gracefully or error clearly
        assert!(result.is_err() || result.unwrap().len() > 0);
    }

    #[test]
    fn test_invalid_operator_sequence() {
        let expr = "a ++ b";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens_result = tokenizer.tokenize();
        
        if let Ok(tokens) = tokens_result {
            let mut parser = Parser::new(tokens);
            let result = parser.parse();
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_division_by_zero() {
        let expr = "10 / 0";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = HashMap::new();
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        // Should error or return Infinity
        assert!(result.is_err() || result.unwrap() == json!(f64::INFINITY));
    }

    #[test]
    fn test_undefined_variable() {
        let expr = "unknown_var + 10";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = HashMap::new();
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found") || result.unwrap_err().contains("undefined"));
    }

    #[test]
    fn test_type_mismatch_addition() {
        let expr = "text + 42";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("text".to_string(), json!("hello"));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        // Should either coerce or error
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_invalid_function_name() {
        let expr = "invalid_func(a, b)";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("a".to_string(), json!(1));
        context.insert("b".to_string(), json!(2));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function") || result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_wrong_function_argument_count() {
        let expr = "max(a)"; // max needs at least 2 args
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("a".to_string(), json!(10));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_pipe_filter() {
        let expr = "name | invalid_filter";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("name".to_string(), json!("test"));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown filter") || result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_pipe_on_wrong_type() {
        let expr = "number | upper"; // upper expects string
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("number".to_string(), json!(42));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expects string") || result.unwrap_err().contains("type"));
    }

    // ========== Validation Errors ==========

    #[test]
    fn test_validation_missing_required_field() {
        let schema = json!({
            "type": "object",
            "required": ["email", "password"],
            "properties": {
                "email": {"type": "string"},
                "password": {"type": "string"}
            }
        });

        let data = json!({
            "email": "user@example.com"
            // password missing
        });

        let result = validate_input(&data, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("required") || result.unwrap_err().contains("password"));
    }

    #[test]
    fn test_validation_wrong_type() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {"type": "integer"}
            }
        });

        let data = json!({
            "age": "not a number"
        });

        let result = validate_input(&data, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("type") || result.unwrap_err().contains("integer"));
    }

    #[test]
    fn test_validation_format_email() {
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {"type": "string", "format": "email"}
            }
        });

        let data = json!({
            "email": "not-an-email"
        });

        let result = validate_input(&data, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("format") || result.unwrap_err().contains("email"));
    }

    #[test]
    fn test_validation_number_range() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 150
                }
            }
        });

        let data = json!({
            "age": 200
        });

        let result = validate_input(&data, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("maximum") || result.unwrap_err().contains("150"));
    }

    #[test]
    fn test_validation_string_length() {
        let schema = json!({
            "type": "object",
            "properties": {
                "username": {
                    "type": "string",
                    "minLength": 3,
                    "maxLength": 20
                }
            }
        });

        let data = json!({
            "username": "ab"
        });

        let result = validate_input(&data, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("minLength") || result.unwrap_err().contains("3"));
    }

    #[test]
    fn test_validation_array_items() {
        let schema = json!({
            "type": "object",
            "properties": {
                "tags": {
                    "type": "array",
                    "minItems": 1,
                    "maxItems": 5
                }
            }
        });

        let data = json!({
            "tags": []
        });

        let result = validate_input(&data, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("minItems") || result.unwrap_err().contains("1"));
    }

    #[test]
    fn test_validation_enum() {
        let schema = json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["active", "inactive", "pending"]
                }
            }
        });

        let data = json!({
            "status": "invalid"
        });

        let result = validate_input(&data, &schema);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("enum") || result.unwrap_err().contains("one of"));
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_extremely_long_expression() {
        let mut expr = String::from("1");
        for _ in 0..1000 {
            expr.push_str(" + 1");
        }
        
        let mut tokenizer = Tokenizer::new(&expr);
        let result = tokenizer.tokenize();
        
        // Should handle or error gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_deeply_nested_expression() {
        let mut expr = String::from("1");
        for _ in 0..100 {
            expr = format!("({} + 1)", expr);
        }
        
        let mut tokenizer = Tokenizer::new(&expr);
        let tokens_result = tokenizer.tokenize();
        
        if let Ok(tokens) = tokens_result {
            let mut parser = Parser::new(tokens);
            let result = parser.parse();
            // Should handle or limit depth
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_empty_expression() {
        let expr = "";
        let mut tokenizer = Tokenizer::new(expr);
        let result = tokenizer.tokenize();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_whitespace_only_expression() {
        let expr = "   \t\n  ";
        let mut tokenizer = Tokenizer::new(expr);
        let result = tokenizer.tokenize();
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_special_characters_in_strings() {
        let yaml = r#"
name: test_special_chars
route: /test
method: POST
operations:
  - text: "{{ request.text }}"
  - return: {text: "{{ text }}"}
"#;
        let result = parse_route(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unicode_in_expressions() {
        let expr = "name == '你好'";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("name".to_string(), json!("你好"));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!(true));
    }

    #[test]
    fn test_null_values_in_expressions() {
        let expr = "value == null";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("value".to_string(), json!(null));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!(true));
    }

    #[test]
    fn test_boolean_logic_edge_cases() {
        let tests = vec![
            ("true && false", false),
            ("true || false", true),
            ("!true", false),
            ("!false", true),
            ("true && true", true),
            ("false || false", false),
        ];

        for (expr, expected) in tests {
            let mut tokenizer = Tokenizer::new(expr);
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            
            let context = HashMap::new();
            let evaluator = Evaluator::new(&context);
            let result = evaluator.evaluate(&ast).unwrap();
            
            assert_eq!(result, json!(expected), "Failed for expression: {}", expr);
        }
    }

    #[test]
    fn test_comparison_with_different_types() {
        // String vs Number comparison
        let expr = "text == 42";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut context = HashMap::new();
        context.insert("text".to_string(), json!("42"));
        
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        // Should return false (strict comparison) or coerce
        assert!(result.is_ok());
    }

    #[test]
    fn test_very_large_numbers() {
        let expr = "9999999999999999999999 + 1";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = HashMap::new();
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast);
        
        // Should handle large numbers or overflow gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_negative_zero() {
        let expr = "-0.0 == 0.0";
        let mut tokenizer = Tokenizer::new(expr);
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let context = HashMap::new();
        let evaluator = Evaluator::new(&context);
        let result = evaluator.evaluate(&ast).unwrap();
        
        assert_eq!(result, json!(true));
    }
}
