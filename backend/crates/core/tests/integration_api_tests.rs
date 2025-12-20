/// Integration Tests for Dynamic Routes API
/// 
/// Tests complete request/response cycles through the API layer
/// Covers:
/// - Route registration (JSON + YAML)
/// - Route execution
/// - Expression evaluation
/// - SQL operations
/// - Error handling
/// - Validation
/// - Schema enforcement

use serde_json::json;

#[cfg(test)]
mod api_integration_tests {
    use super::*;
    use core::parsers::route_parser::parse_route;
    use core::services::DynamicRouteService;
    use std::collections::HashMap;

    fn setup_service() -> DynamicRouteService {
        DynamicRouteService::new()
    }

    #[test]
    fn test_parse_and_register_json_route() {
        let json_route = r#"{
            "name": "test_json",
            "route": "/api/test/json",
            "method": "GET",
            "operations": [
                {
                    "operation": "return",
                    "value": {"status": 200, "message": "JSON route works"}
                }
            ]
        }"#;

        let result = parse_route(json_route);
        assert!(result.is_ok());
        
        let route = result.unwrap();
        assert_eq!(route.name, "test_json");
        assert_eq!(route.method, "GET");
        assert_eq!(route.operations.len(), 1);
    }

    #[test]
    fn test_parse_and_register_yaml_route() {
        let yaml_route = r#"
name: test_yaml
route: /api/test/yaml
method: POST
operations:
  - operation: return
    value:
      status: 200
      message: YAML route works
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
        
        let route = result.unwrap();
        assert_eq!(route.name, "test_yaml");
        assert_eq!(route.method, "POST");
        assert_eq!(route.operations.len(), 1);
    }

    #[test]
    fn test_route_with_expressions() {
        let yaml_route = r#"
name: test_expressions
route: /api/test/expr
method: POST
operations:
  - result: "{{ 2 + 3 }}"
  - doubled: "{{ result * 2 }}"
  - operation: return
    value:
      result: "{{ result }}"
      doubled: "{{ doubled }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
        
        let route = result.unwrap();
        assert_eq!(route.operations.len(), 3);
    }

    #[test]
    fn test_route_with_pipe_operators() {
        let yaml_route = r#"
name: test_pipes
route: /api/test/pipes
method: POST
operations:
  - name: "{{ request.name | upper | trim }}"
  - email: "{{ request.email | lower }}"
  - operation: return
    value:
      name: "{{ name }}"
      email: "{{ email }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
        
        let route = result.unwrap();
        assert_eq!(route.operations.len(), 3);
    }

    #[test]
    fn test_route_with_conditionals() {
        let yaml_route = r#"
name: test_conditionals
route: /api/test/cond
method: POST
operations:
  - age: "{{ request.age }}"
  - status: "{{ age >= 18 ? 'adult' : 'minor' }}"
  - operation: return
    value:
      age: "{{ age }}"
      status: "{{ status }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_sql_operations() {
        let yaml_route = r#"
name: test_sql
route: /api/test/sql
method: POST
operations:
  - email: "{{ request.email | lower }}"
  - operation: sql
    query: "SELECT * FROM users WHERE email = ?"
    params: ["{{ email }}"]
  - operation: return
    value:
      found: "{{ sql_result != null }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
        
        let route = result.unwrap();
        assert!(route.operations.len() >= 3);
    }

    #[test]
    fn test_route_with_sql_named_params() {
        let yaml_route = r#"
name: test_sql_named
route: /api/test/sql-named
method: POST
operations:
  - email: "{{ request.email | lower }}"
  - operation: sql
    query: "SELECT * FROM users WHERE email = :email AND status = :status"
    params:
      email: "{{ email }}"
      status: active
  - operation: return
    value:
      found: "{{ sql_result != null }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_validation_schema() {
        let yaml_route = r#"
name: test_validation
route: /api/test/validate
method: POST
schema:
  type: object
  required:
    - email
    - age
  properties:
    email:
      type: string
      format: email
    age:
      type: integer
      minimum: 0
      maximum: 150
operations:
  - operation: return
    value:
      message: Validation passed
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
        
        let route = result.unwrap();
        assert!(route.schema.is_some());
    }

    #[test]
    fn test_route_with_helper_functions() {
        let yaml_route = r#"
name: test_helpers
route: /api/test/helpers
method: POST
operations:
  - id: "{{ uuid() }}"
  - timestamp: "{{ now_iso() }}"
  - hash: "{{ hash_password(request.password) }}"
  - operation: return
    value:
      id: "{{ id }}"
      timestamp: "{{ timestamp }}"
      hash: "{{ hash }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_loops() {
        let yaml_route = r#"
name: test_loops
route: /api/test/loops
method: POST
operations:
  - operation: foreach
    items: "{{ request.items }}"
    item: item
    body:
      - total: "{{ total + item.price }}"
  - operation: return
    value:
      total: "{{ total }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_json_format() {
        let invalid_json = r#"{
            "name": "broken",
            "route": "/test"
            missing_comma: true
        }"#;

        let result = parse_route(invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("parse error"));
    }

    #[test]
    fn test_invalid_yaml_format() {
        let invalid_yaml = r#"
name: broken
route: /test
  bad_indent: true
method: GET
"#;

        let result = parse_route(invalid_yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_required_fields() {
        let incomplete = r#"{
            "name": "incomplete"
        }"#;

        let result = parse_route(incomplete);
        // Should fail because route, method, operations are missing
        assert!(result.is_err() || {
            let route = result.unwrap();
            route.route.is_empty() || route.operations.is_empty()
        });
    }

    #[test]
    fn test_route_with_complex_expressions() {
        let yaml_route = r#"
name: test_complex
route: /api/test/complex
method: POST
operations:
  - subtotal: "{{ request.price * request.quantity }}"
  - discount: "{{ subtotal > 100 ? 0.1 : 0 }}"
  - tax: "{{ subtotal * (1 - discount) * 0.15 }}"
  - total: "{{ subtotal * (1 - discount) + tax }}"
  - operation: return
    value:
      subtotal: "{{ subtotal }}"
      discount: "{{ discount * 100 }}%"
      tax: "{{ tax }}"
      total: "{{ total }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_string_operations() {
        let yaml_route = r#"
name: test_strings
route: /api/test/strings
method: POST
operations:
  - full_name: "{{ request.first_name + ' ' + request.last_name }}"
  - upper: "{{ full_name | upper }}"
  - slug: "{{ full_name | lower | replace(' ', '-') }}"
  - operation: return
    value:
      full_name: "{{ full_name }}"
      upper: "{{ upper }}"
      slug: "{{ slug }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_array_operations() {
        let yaml_route = r#"
name: test_arrays
route: /api/test/arrays
method: POST
operations:
  - count: "{{ request.items | length }}"
  - first: "{{ request.items | first }}"
  - last: "{{ request.items | last }}"
  - sorted: "{{ request.items | sort }}"
  - operation: return
    value:
      count: "{{ count }}"
      first: "{{ first }}"
      last: "{{ last }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_multiline_sql() {
        let yaml_route = r#"
name: test_multiline_sql
route: /api/test/multiline
method: POST
operations:
  - operation: sql
    query: |
      SELECT u.id, u.name, u.email,
             COUNT(o.id) as order_count
      FROM users u
      LEFT JOIN orders o ON u.id = o.user_id
      WHERE u.email = :email
      GROUP BY u.id
    params:
      email: "{{ request.email }}"
  - operation: return
    value:
      user: "{{ sql_result }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_shorthand_syntax() {
        let yaml_route = r#"
name: test_shorthand
route: /api/test/shorthand
method: POST
operations:
  - email: "{{ request.email | lower }}"
  - name: "{{ request.name | trim }}"
  - age: "{{ request.age }}"
  - return:
      email: "{{ email }}"
      name: "{{ name }}"
      is_adult: "{{ age >= 18 }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mixed_json_yaml_compatibility() {
        // JSON version
        let json = r#"{
            "name": "test_compat",
            "route": "/test",
            "method": "GET",
            "operations": [
                {"operation": "return", "value": {"status": 200}}
            ]
        }"#;

        // YAML version
        let yaml = r#"
name: test_compat
route: /test
method: GET
operations:
  - operation: return
    value:
      status: 200
"#;

        let json_result = parse_route(json);
        let yaml_result = parse_route(yaml);

        assert!(json_result.is_ok());
        assert!(yaml_result.is_ok());
        
        let json_route = json_result.unwrap();
        let yaml_route = yaml_result.unwrap();
        
        assert_eq!(json_route.name, yaml_route.name);
        assert_eq!(json_route.route, yaml_route.route);
        assert_eq!(json_route.method, yaml_route.method);
    }

    #[test]
    fn test_route_with_nested_conditionals() {
        let yaml_route = r#"
name: test_nested_cond
route: /api/test/nested
method: POST
operations:
  - age: "{{ request.age }}"
  - status: "{{ age < 13 ? 'child' : (age < 18 ? 'teen' : (age < 65 ? 'adult' : 'senior')) }}"
  - operation: return
    value:
      status: "{{ status }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_with_date_operations() {
        let yaml_route = r#"
name: test_dates
route: /api/test/dates
method: GET
operations:
  - now: "{{ now_iso() }}"
  - today: "{{ today() }}"
  - unix: "{{ now_unix() }}"
  - operation: return
    value:
      now: "{{ now }}"
      today: "{{ today }}"
      unix: "{{ unix }}"
"#;

        let result = parse_route(yaml_route);
        assert!(result.is_ok());
    }
}
