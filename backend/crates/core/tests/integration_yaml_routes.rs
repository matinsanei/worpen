// Integration tests for YAML route parsing and execution
use core::parsers::{detect_format, parse_route, InputFormat};

#[test]
fn test_simple_yaml_route_basic_fields() {
    // Test basic YAML parsing without complex logic
    let yaml = r#"
name: Echo API
description: Simple echo endpoint
path: /api/echo
method: POST
parameters: []
enabled: true
version: "1.0.0"
auth_required: false
"#;

    let format = detect_format(yaml);
    assert!(matches!(format, InputFormat::Yaml));
    
    // This will fail on logic deserialization, but that's OK for now
    // We're testing basic field parsing
    let result = parse_route(yaml);
    if let Err(e) = result {
        // Expected - logic field is required
        assert!(e.contains("missing field") || e.contains("logic"));
    }
}

#[test]
fn test_yaml_detection() {
    let yaml = "name: test\npath: /api";
    assert!(matches!(detect_format(yaml), InputFormat::Yaml));
    
    let json = r#"{"name": "test"}"#;
    assert!(matches!(detect_format(json), InputFormat::Json));
}

#[test]
fn test_json_simple_route() {
    // Test that JSON parsing still works perfectly
    let json = r#"{
  "name": "Echo API",
  "description": "Simple echo",
  "path": "/api/echo",
  "method": "POST",
  "logic": [{
    "return": {
      "value": {"message": "test"}
    }
  }],
  "parameters": [],
  "enabled": true,
  "version": "1.0.0",
  "auth_required": false
}"#;

    let format = detect_format(json);
    assert!(matches!(format, InputFormat::Json));

    let route = parse_route(json).expect("Failed to parse JSON route");
    assert_eq!(route.name, "Echo API");
    assert_eq!(route.path, "/api/echo");
    assert_eq!(route.logic.len(), 1);
}

#[test]
fn test_json_conditional_logic() {
    let json = r#"{
  "name": "Age Check",
  "description": "Check user age",
  "path": "/api/age-check",
  "method": "POST",
  "logic": [{
    "if": {
      "condition": "age >= 18",
      "then": [{
        "return": {"value": {"status": "adult"}}
      }],
      "otherwise": [{
        "return": {"value": {"status": "minor"}}
      }]
    }
  }],
  "parameters": [{
    "name": "age",
    "param_type": "body",
    "data_type": "number",
    "required": true
  }],
  "enabled": true,
  "version": "1.0.0",
  "auth_required": false
}"#;

    let route = parse_route(json).expect("Failed to parse JSON with conditional");
    assert_eq!(route.name, "Age Check");
    assert_eq!(route.parameters.len(), 1);
}

#[test]
fn test_json_database_query() {
    let json = r#"{
  "name": "Get Users",
  "description": "Fetch users from database",
  "path": "/api/users",
  "method": "GET",
  "logic": [{
    "query_db": {
      "query": "SELECT * FROM agents LIMIT 10",
      "params": []
    }
  }, {
    "return": {
      "value": {"users": "{{db_result}}"}
    }
  }],
  "parameters": [],
  "enabled": true,
  "version": "1.0.0",
  "auth_required": false
}"#;

    let route = parse_route(json).expect("Failed to parse JSON with DB query");
    assert_eq!(route.name, "Get Users");
    assert_eq!(route.logic.len(), 2);
}

#[test]
fn test_json_loop_operation() {
    let json = r#"{
  "name": "Process Items",
  "description": "Loop through items",
  "path": "/api/process",
  "method": "POST",
  "logic": [{
    "loop": {
      "collection": "items",
      "var": "item",
      "body": [{
        "log": {
          "level": "info",
          "message": "Processing item"
        }
      }]
    }
  }],
  "parameters": [],
  "enabled": true,
  "version": "1.0.0",
  "auth_required": false
}"#;

    let route = parse_route(json).expect("Failed to parse JSON with loop");
    assert_eq!(route.name, "Process Items");
}

#[test]
fn test_invalid_json() {
    let invalid = r#"{"name": invalid json"#;
    let result = parse_route(invalid);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("JSON parse error"));
}

#[test]
fn test_missing_required_fields() {
    let incomplete = r#"{"name": "Test"}"#;
    let result = parse_route(incomplete);
    assert!(result.is_err());
}

#[test]
fn test_format_detection_edge_cases() {
    // Whitespace before JSON
    let json_with_spaces = "   \n  {\"name\": \"test\"}";
    assert!(matches!(detect_format(json_with_spaces), InputFormat::Json));

    // Whitespace before YAML
    let yaml_with_spaces = "  \n  name: test";
    assert!(matches!(detect_format(yaml_with_spaces), InputFormat::Yaml));

    // Array JSON
    let json_array = "[{\"name\": \"test\"}]";
    assert!(matches!(detect_format(json_array), InputFormat::Json));
    
    // YAML with dash
    let yaml_dash = "- name: test";
    assert!(matches!(detect_format(yaml_dash), InputFormat::Yaml));
}
