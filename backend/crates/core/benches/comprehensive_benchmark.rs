/// Performance Benchmarks for Dynamic Routes Engine
/// 
/// Measures:
/// - JSON vs YAML parsing speed
/// - Expression evaluation performance
/// - Pipe operator overhead
/// - Route execution time
/// - Complex operation benchmarks

#![feature(test)]
extern crate test;

use test::Bencher;
use core::parsers::route_parser::parse_route;
use core::expression::{Tokenizer, Parser, Evaluator};
use serde_json::json;
use std::collections::HashMap;

// Simple route for baseline
const SIMPLE_JSON: &str = r#"{
    "name": "simple",
    "route": "/test",
    "method": "GET",
    "operations": [{"operation": "return", "value": {"status": 200}}]
}"#;

const SIMPLE_YAML: &str = r#"
name: simple
route: /test
method: GET
operations:
  - operation: return
    value: {status: 200}
"#;

// Complex route with expressions
const COMPLEX_JSON: &str = r#"{
    "name": "complex",
    "route": "/calc",
    "method": "POST",
    "operations": [
        {"operation": "set", "variable": "a", "value": "{{request.a}}"},
        {"operation": "set", "variable": "b", "value": "{{request.b}}"},
        {"operation": "math", "type": "multiply", "args": ["{{a}}", "{{b}}"]},
        {"operation": "set", "variable": "result", "value": "{{math_result}}"},
        {"operation": "return", "value": {"result": "{{result}}"}}
    ]
}"#;

const COMPLEX_YAML: &str = r#"
name: complex
route: /calc
method: POST
operations:
  - a: "{{ request.a }}"
  - b: "{{ request.b }}"
  - result: "{{ a * b }}"
  - return: {result: "{{ result }}"}
"#;

#[bench]
fn bench_parse_simple_json(b: &mut Bencher) {
    b.iter(|| {
        parse_route(SIMPLE_JSON).unwrap()
    });
}

#[bench]
fn bench_parse_simple_yaml(b: &mut Bencher) {
    b.iter(|| {
        parse_route(SIMPLE_YAML).unwrap()
    });
}

#[bench]
fn bench_parse_complex_json(b: &mut Bencher) {
    b.iter(|| {
        parse_route(COMPLEX_JSON).unwrap()
    });
}

#[bench]
fn bench_parse_complex_yaml(b: &mut Bencher) {
    b.iter(|| {
        parse_route(COMPLEX_YAML).unwrap()
    });
}

#[bench]
fn bench_tokenize_simple_expression(b: &mut Bencher) {
    b.iter(|| {
        let mut tokenizer = Tokenizer::new("2 + 3");
        tokenizer.tokenize().unwrap()
    });
}

#[bench]
fn bench_tokenize_complex_expression(b: &mut Bencher) {
    b.iter(|| {
        let mut tokenizer = Tokenizer::new("(a + b) * c - d / e");
        tokenizer.tokenize().unwrap()
    });
}

#[bench]
fn bench_parse_simple_expression(b: &mut Bencher) {
    b.iter(|| {
        let mut tokenizer = Tokenizer::new("2 + 3");
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    });
}

#[bench]
fn bench_parse_complex_expression(b: &mut Bencher) {
    b.iter(|| {
        let mut tokenizer = Tokenizer::new("(price * quantity) * (1 - discount) + tax");
        let tokens = tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    });
}

#[bench]
fn bench_evaluate_arithmetic(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("2 + 3 * 4");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let context = HashMap::new();
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_evaluate_with_variables(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("a + b * c");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("a".to_string(), json!(10));
    context.insert("b".to_string(), json!(20));
    context.insert("c".to_string(), json!(5));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_pipe_single_filter(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("name | upper");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("name".to_string(), json!("john"));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_pipe_multiple_filters(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("name | lower | trim | capitalize");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("name".to_string(), json!("  JOHN DOE  "));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_ternary_operator(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("age >= 18 ? 'adult' : 'minor'");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("age".to_string(), json!(25));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_function_call(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("max(a, b, c)");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("a".to_string(), json!(10));
    context.insert("b".to_string(), json!(25));
    context.insert("c".to_string(), json!(15));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_string_concatenation(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("first + ' ' + last");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("first".to_string(), json!("John"));
    context.insert("last".to_string(), json!("Doe"));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_complex_business_logic(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new(
        "(price * quantity) * (1 - (discount / 100)) * (1 + (tax / 100))"
    );
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("price".to_string(), json!(99.99));
    context.insert("quantity".to_string(), json!(3));
    context.insert("discount".to_string(), json!(10));
    context.insert("tax".to_string(), json!(15));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_nested_ternary(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new(
        "age < 13 ? 'child' : (age < 18 ? 'teen' : (age < 65 ? 'adult' : 'senior'))"
    );
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("age".to_string(), json!(45));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_array_filter_pipe(b: &mut Bencher) {
    let mut tokenizer = Tokenizer::new("items | length");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("items".to_string(), json!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    
    b.iter(|| {
        let evaluator = Evaluator::new(&context);
        evaluator.evaluate(&ast).unwrap()
    });
}

#[bench]
fn bench_route_with_10_operations(b: &mut Bencher) {
    let yaml = r#"
name: bench_10_ops
route: /bench
method: POST
operations:
  - v1: "{{ request.a }}"
  - v2: "{{ request.b }}"
  - v3: "{{ v1 + v2 }}"
  - v4: "{{ v3 * 2 }}"
  - v5: "{{ v4 - v1 }}"
  - v6: "{{ v5 / v2 }}"
  - v7: "{{ v6 + 10 }}"
  - v8: "{{ v7 * v1 }}"
  - v9: "{{ v8 - v4 }}"
  - return: {result: "{{ v9 }}"}
"#;
    
    b.iter(|| {
        parse_route(yaml).unwrap()
    });
}

#[bench]
fn bench_route_with_50_operations(b: &mut Bencher) {
    let mut yaml = String::from("name: bench_50_ops\nroute: /bench\nmethod: POST\noperations:\n");
    for i in 1..=50 {
        yaml.push_str(&format!("  - v{}: \"{{{{ v{} + 1 }}}}\"\n", i, i.saturating_sub(1)));
    }
    yaml.push_str("  - return: {result: \"{{ v50 }}\"}\n");
    
    b.iter(|| {
        parse_route(&yaml).unwrap()
    });
}
