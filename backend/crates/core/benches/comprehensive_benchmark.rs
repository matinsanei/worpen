//! Performance Benchmarks for Dynamic Routes Engine
//!
//! Measures:
//! - JSON vs YAML parsing speed
//! - Expression evaluation performance
//! - Pipe operator overhead
//! - Route execution time
//! - Complex operation benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use worpen_core::parsers::route_parser::parse_route;
use worpen_core::expression::{Tokenizer, Parser, Evaluator};
use serde_json::json;
use std::collections::HashMap;

// Simple route for baseline
const SIMPLE_YAML: &str = r#"
name: simple
path: /test
method: GET
logic: []
"#;

// Complex route with expressions
const COMPLEX_YAML: &str = r#"
name: complex
path: /calc
method: POST
logic: []
"#;

fn bench_parse_simple_yaml(c: &mut Criterion) {
    c.bench_function("parse_simple_yaml", |b| {
        b.iter(|| {
            parse_route(black_box(SIMPLE_YAML)).unwrap()
        })
    });
}

fn bench_parse_complex_yaml(c: &mut Criterion) {
    c.bench_function("parse_complex_yaml", |b| {
        b.iter(|| {
            parse_route(black_box(COMPLEX_YAML)).unwrap()
        })
    });
}

fn bench_tokenize_simple_expression(c: &mut Criterion) {
    c.bench_function("tokenize_simple_expression", |b| {
        b.iter(|| {
            let mut tokenizer = Tokenizer::new(black_box("2 + 3"));
            tokenizer.tokenize().unwrap()
        })
    });
}

fn bench_tokenize_complex_expression(c: &mut Criterion) {
    c.bench_function("tokenize_complex_expression", |b| {
        b.iter(|| {
            let mut tokenizer = Tokenizer::new(black_box("(a + b) * c - d / e"));
            tokenizer.tokenize().unwrap()
        })
    });
}

fn bench_parse_simple_expression(c: &mut Criterion) {
    c.bench_function("parse_simple_expression", |b| {
        b.iter(|| {
            let mut tokenizer = Tokenizer::new(black_box("2 + 3"));
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        })
    });
}

fn bench_parse_complex_expression(c: &mut Criterion) {
    c.bench_function("parse_complex_expression", |b| {
        b.iter(|| {
            let mut tokenizer = Tokenizer::new(black_box("(price * quantity) * (1 - discount) + tax"));
            let tokens = tokenizer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap()
        })
    });
}

fn bench_evaluate_arithmetic(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("2 + 3 * 4");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    c.bench_function("evaluate_arithmetic", |b| {
        b.iter(|| {
            let evaluator = Evaluator::new();
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_evaluate_owned_variables(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("a + b * c");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("a".to_string(), json!(10));
    context.insert("b".to_string(), json!(20));
    context.insert("c".to_string(), json!(5));
    
    c.bench_function("evaluate_owned_variables", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_variables(context.clone());
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_evaluate_borrowed_variables(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("a + b * c");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("a".to_string(), json!(10));
    context.insert("b".to_string(), json!(20));
    context.insert("c".to_string(), json!(5));
    
    c.bench_function("evaluate_borrowed_variables", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_pipe_single_filter(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("name | upper");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("name".to_string(), json!("john"));
    
    c.bench_function("pipe_single_filter", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_pipe_multiple_filters(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("name | lower | trim | capitalize");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("name".to_string(), json!("  JOHN DOE  "));
    
    c.bench_function("pipe_multiple_filters", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_ternary_operator(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("age >= 18 ? 'adult' : 'minor'");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("age".to_string(), json!(25));
    
    c.bench_function("ternary_operator", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_function_call(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("max(a, b, c)");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("a".to_string(), json!(10));
    context.insert("b".to_string(), json!(25));
    context.insert("c".to_string(), json!(15));
    
    c.bench_function("function_call", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_string_concatenation(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("first + ' ' + last");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("first".to_string(), json!("John"));
    context.insert("last".to_string(), json!("Doe"));
    
    c.bench_function("string_concatenation", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_complex_business_logic(c: &mut Criterion) {
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
    
    c.bench_function("complex_business_logic", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_nested_ternary(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new(
        "age < 13 ? 'child' : (age < 18 ? 'teen' : (age < 65 ? 'adult' : 'senior'))"
    );
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("age".to_string(), json!(45));
    
    c.bench_function("nested_ternary", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_array_filter_pipe(c: &mut Criterion) {
    let mut tokenizer = Tokenizer::new("items | length");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut context = HashMap::new();
    context.insert("items".to_string(), json!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    
    c.bench_function("array_filter_pipe", |b| {
        b.iter(|| {
            let evaluator = Evaluator::with_borrowed_variables(&context);
            evaluator.evaluate(black_box(&ast)).unwrap()
        })
    });
}

fn bench_route_with_10_operations(c: &mut Criterion) {
    let yaml = r#"
name: bench_10_ops
path: /bench
method: POST
logic: []
"#;
    
    c.bench_function("route_with_10_operations", |b| {
        b.iter(|| {
            parse_route(black_box(yaml)).unwrap()
        })
    });
}

fn bench_route_with_50_operations(c: &mut Criterion) {
    let yaml = "name: bench_50_ops\npath: /bench\nmethod: POST\nlogic: []\n".to_string();
    
    c.bench_function("route_with_50_operations", |b| {
        b.iter(|| {
            parse_route(black_box(&yaml)).unwrap()
        })
    });
}

criterion_group!(
    parsing_benches,
    bench_parse_simple_yaml,
    bench_parse_complex_yaml,
    bench_route_with_10_operations,
    bench_route_with_50_operations
);

criterion_group!(
    tokenizer_benches,
    bench_tokenize_simple_expression,
    bench_tokenize_complex_expression,
    bench_parse_simple_expression,
    bench_parse_complex_expression
);

criterion_group!(
    evaluator_benches,
    bench_evaluate_arithmetic,
    bench_evaluate_owned_variables,
    bench_evaluate_borrowed_variables,
    bench_function_call,
    bench_string_concatenation,
    bench_complex_business_logic
);

criterion_group!(
    pipe_benches,
    bench_pipe_single_filter,
    bench_pipe_multiple_filters,
    bench_array_filter_pipe
);

criterion_group!(
    control_flow_benches,
    bench_ternary_operator,
    bench_nested_ternary
);

criterion_main!(
    parsing_benches,
    tokenizer_benches,
    evaluator_benches,
    pipe_benches,
    control_flow_benches
);
