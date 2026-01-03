// Performance benchmarks for JSON vs YAML parsing
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use worpen_core::parsers::route_parser::parse_route;

const SIMPLE_JSON: &str = r#"{
  "name": "Benchmark Route",
  "description": "Performance test",
  "path": "/api/bench",
  "method": "GET",
  "logic": [{"return": {"value": {"status": "ok"}}}],
  "parameters": [],
  "enabled": true,
  "version": "1.0.0",
  "auth_required": false
}"#;

const SIMPLE_YAML: &str = r#"
name: Benchmark Route
description: Performance test
path: /api/bench
method: GET
logic:
  - return:
      value:
        status: ok
parameters: []
enabled: true
version: "1.0.0"
auth_required: false
"#;

const COMPLEX_JSON: &str = r#"{
  "name": "Complex Route",
  "description": "Heavy route",
  "path": "/api/complex",
  "method": "POST",
  "logic": [
    {"query_db": {"query": "SELECT * FROM users", "params": []}},
    {"if": {"condition": "count > 0", "then": [{"return": {"value": {"found": true}}}], "otherwise": [{"return": {"value": {"found": false}}}]}},
    {"loop": {"collection": "items", "var": "item", "body": [{"log": {"level": "info", "message": "Processing"}}]}}
  ],
  "parameters": [
    {"name": "query", "param_type": "body", "data_type": "string", "required": true}
  ],
  "enabled": true,
  "version": "1.0.0",
  "auth_required": true
}"#;

const COMPLEX_YAML: &str = r#"
name: Complex Route
description: Heavy route
path: /api/complex
method: POST
logic:
  - query_db:
      query: SELECT * FROM users
      params: []
  - if:
      condition: "count > 0"
      then:
        - return:
            value:
              found: true
      otherwise:
        - return:
            value:
              found: false
  - loop:
      collection: items
      var: item
      body:
        - log:
            level: info
            message: Processing
parameters:
  - name: query
    param_type: body
    data_type: string
    required: true
enabled: true
version: "1.0.0"
auth_required: true
"#;

fn bench_simple_json(c: &mut Criterion) {
    c.bench_function("parse_simple_json", |b| {
        b.iter(|| parse_route(black_box(SIMPLE_JSON)))
    });
}

fn bench_simple_yaml(c: &mut Criterion) {
    c.bench_function("parse_simple_yaml", |b| {
        b.iter(|| parse_route(black_box(SIMPLE_YAML)))
    });
}

fn bench_complex_json(c: &mut Criterion) {
    c.bench_function("parse_complex_json", |b| {
        b.iter(|| parse_route(black_box(COMPLEX_JSON)))
    });
}

fn bench_complex_yaml(c: &mut Criterion) {
    c.bench_function("parse_complex_yaml", |b| {
        b.iter(|| parse_route(black_box(COMPLEX_YAML)))
    });
}

criterion_group!(
    benches,
    bench_simple_json,
    bench_simple_yaml,
    bench_complex_json,
    bench_complex_yaml
);
criterion_main!(benches);
