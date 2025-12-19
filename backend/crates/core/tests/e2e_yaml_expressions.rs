// End-to-End tests for YAML routes with expression evaluation
// Tests actual route execution with {{expressions}}

use core::expression::resolve_templates;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Simple YAML route definition for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimpleRoute {
    name: String,
    path: String,
    method: String,
    #[serde(default)]
    response: Value,
}

/// Simulate route execution with expression context
fn execute_route_with_expressions(
    route_def: &SimpleRoute,
    request_data: Value,
) -> Result<Value, String> {
    // Build execution context from request data
    let mut context = HashMap::new();
    
    // Extract variables from request body
    if let Some(obj) = request_data.as_object() {
        for (key, value) in obj {
            context.insert(key.clone(), value.clone());
        }
    }
    
    // Resolve expressions in response
    resolve_templates(&route_def.response, &context)
}

#[test]
fn test_e2e_simple_echo_route() {
    let yaml = r#"
name: echo_test
path: /echo
method: POST
response:
  message: "{{message}}"
  uppercase: "{{message | upper}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "message": "hello world"
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["message"], json!("hello world"));
    assert_eq!(response["uppercase"], json!("HELLO WORLD"));
}

#[test]
fn test_e2e_calculator_route() {
    let yaml = r#"
name: calculator
path: /calculate
method: POST
response:
  operation: add
  a: "{{a}}"
  b: "{{b}}"
  result: "{{a + b}}"
  doubled: "{{(a + b) * 2}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "a": 10,
        "b": 20
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["a"].as_f64().unwrap(), 10.0);
    assert_eq!(response["b"].as_f64().unwrap(), 20.0);
    assert_eq!(response["result"].as_f64().unwrap(), 30.0);
    assert_eq!(response["doubled"].as_f64().unwrap(), 60.0);
}

#[test]
fn test_e2e_user_registration_route() {
    let yaml = r#"
name: user_registration
path: /users/register
method: POST
response:
  success: true
  user:
    email: "{{email | trim | lower}}"
    username: "{{username | trim}}"
    display_name: "{{first_name | capitalize}} {{last_name | capitalize}}"
    is_adult: "{{age >= 18}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "email": "  USER@EXAMPLE.COM  ",
        "username": "  johndoe  ",
        "first_name": "john",
        "last_name": "doe",
        "age": 25
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["user"]["email"], json!("user@example.com"));
    assert_eq!(response["user"]["username"], json!("johndoe"));
    assert_eq!(response["user"]["display_name"], json!("John Doe"));
    assert_eq!(response["user"]["is_adult"], json!(true));
}

#[test]
fn test_e2e_checkout_route() {
    let yaml = r#"
name: checkout
path: /checkout
method: POST
response:
  subtotal: "{{price * quantity}}"
  tax: "{{price * quantity * 0.1}}"
  shipping: "{{quantity > 5 ? 0 : 10}}"
  total: "{{price * quantity * 1.1 + (quantity > 5 ? 0 : 10)}}"
  discount_eligible: "{{price * quantity > 100}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "price": 50,
        "quantity": 3
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["subtotal"].as_f64().unwrap(), 150.0);
    assert_eq!(response["tax"].as_f64().unwrap(), 15.0);
    assert_eq!(response["shipping"].as_f64().unwrap(), 10.0); // qty <= 5
    assert_eq!(response["total"].as_f64().unwrap(), 175.0);
    assert_eq!(response["discount_eligible"], json!(true));
}

#[test]
fn test_e2e_array_operations_route() {
    let yaml = r#"
name: array_ops
path: /array
method: POST
response:
  original: "{{items}}"
  count: "{{items | length}}"
  first: "{{items | first}}"
  last: "{{items | last}}"
  sorted: "{{items | sort}}"
  unique: "{{items | unique}}"
  joined: "{{items | join(', ')}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "items": [3, 1, 4, 1, 5, 9, 2, 6]
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["count"].as_u64().unwrap(), 8);
    assert_eq!(response["first"].as_f64().unwrap(), 3.0);
    assert_eq!(response["last"].as_f64().unwrap(), 6.0);
    assert_eq!(response["joined"], json!("3, 1, 4, 1, 5, 9, 2, 6"));
    
    let sorted = response["sorted"].as_array().unwrap();
    assert_eq!(sorted[0].as_f64().unwrap(), 1.0);
    
    let unique = response["unique"].as_array().unwrap();
    assert_eq!(unique.len(), 7); // 3,1,4,5,9,2,6
}

#[test]
fn test_e2e_string_operations_route() {
    let yaml = r#"
name: string_ops
path: /string
method: POST
response:
  original: "{{text}}"
  upper: "{{text | upper}}"
  lower: "{{text | lower}}"
  trimmed: "{{text | trim}}"
  capitalized: "{{text | trim | lower | capitalize}}"
  reversed: "{{text | trim | reverse}}"
  length: "{{text | trim | length}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "text": "  Hello World  "
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["upper"], json!("  HELLO WORLD  "));
    assert_eq!(response["lower"], json!("  hello world  "));
    assert_eq!(response["trimmed"], json!("Hello World"));
    assert_eq!(response["capitalized"], json!("Hello world"));
    assert_eq!(response["reversed"], json!("dlroW olleH"));
    assert_eq!(response["length"].as_u64().unwrap(), 11);
}

#[test]
fn test_e2e_complex_business_logic() {
    let yaml = r#"
name: order_pricing
path: /orders/price
method: POST
response:
  customer:
    email: "{{email | trim | lower}}"
    level: "{{customer_level | upper}}"
    points: "{{loyalty_points}}"
  pricing:
    base_price: "{{base_price}}"
    quantity: "{{quantity}}"
    subtotal: "{{base_price * quantity}}"
    discount_rate: "{{customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05)}}"
    discount_amount: "{{base_price * quantity * (customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05))}}"
    tax_rate: 0.1
    tax_amount: "{{base_price * quantity * 0.9 * 0.1}}"
    shipping: "{{base_price * quantity > 100 ? 0 : 15}}"
    final_total: "{{base_price * quantity * (1 - (customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05))) * 1.1 + (base_price * quantity > 100 ? 0 : 15)}}"
  bonus:
    points_earned: "{{(base_price * quantity / 10) | floor}}"
    free_shipping: "{{base_price * quantity > 100}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "email": "  PREMIUM@USER.COM  ",
        "customer_level": "premium",
        "loyalty_points": 500,
        "base_price": 25,
        "quantity": 5
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    // Customer info
    assert_eq!(response["customer"]["email"], json!("premium@user.com"));
    assert_eq!(response["customer"]["level"], json!("PREMIUM"));
    assert_eq!(response["customer"]["points"].as_f64().unwrap(), 500.0);
    
    // Pricing
    assert_eq!(response["pricing"]["subtotal"].as_f64().unwrap(), 125.0);
    assert_eq!(response["pricing"]["discount_rate"].as_f64().unwrap(), 0.2);
    assert_eq!(response["pricing"]["discount_amount"].as_f64().unwrap(), 25.0);
    assert_eq!(response["pricing"]["shipping"].as_f64().unwrap(), 0.0); // > 100
    
    // Bonus
    assert_eq!(response["bonus"]["points_earned"].as_f64().unwrap(), 12.0);
    assert_eq!(response["bonus"]["free_shipping"], json!(true));
}

#[test]
fn test_e2e_conditional_responses() {
    let yaml = r#"
name: conditional_response
path: /status
method: POST
response:
  age_group: "{{age < 13 ? 'child' : (age < 18 ? 'teen' : (age < 65 ? 'adult' : 'senior'))}}"
  access_level: "{{is_admin ? 'full' : (is_moderator ? 'limited' : 'basic')}}"
  discount: "{{age >= 65 ? 0.3 : (age < 18 ? 0.1 : 0)}}"
  message: "{{age >= 18 ? 'Welcome!' : 'Parental consent required'}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    // Test senior user
    let request = json!({
        "age": 70,
        "is_admin": false,
        "is_moderator": false
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    assert_eq!(response["age_group"], json!("senior"));
    assert_eq!(response["discount"].as_f64().unwrap(), 0.3);
    
    // Test teen user
    let request = json!({
        "age": 15,
        "is_admin": false,
        "is_moderator": true
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    assert_eq!(response["age_group"], json!("teen"));
    assert_eq!(response["access_level"], json!("limited"));
    assert_eq!(response["discount"].as_f64().unwrap(), 0.1);
    assert_eq!(response["message"], json!("Parental consent required"));
}

#[test]
fn test_e2e_math_functions() {
    let yaml = r#"
name: math_operations
path: /math
method: POST
response:
  inputs:
    numbers: "{{numbers}}"
  results:
    max: "{{max(10, 25, 15, 30, 5)}}"
    min: "{{min(10, 25, 15, 30, 5)}}"
    absolute: "{{abs(-42)}}"
    rounded: "{{value | round}}"
    floor: "{{value | floor}}"
    ceil: "{{value | ceil}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "numbers": [10, 25, 15, 30, 5],
        "value": 3.7
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["results"]["max"].as_f64().unwrap(), 30.0);
    assert_eq!(response["results"]["min"].as_f64().unwrap(), 5.0);
    assert_eq!(response["results"]["absolute"].as_f64().unwrap(), 42.0);
    assert_eq!(response["results"]["rounded"].as_f64().unwrap(), 4.0);
    assert_eq!(response["results"]["floor"].as_f64().unwrap(), 3.0);
    assert_eq!(response["results"]["ceil"].as_f64().unwrap(), 4.0);
}

#[test]
fn test_e2e_realistic_api_response() {
    let yaml = r#"
name: api_response
path: /api/users/{id}
method: GET
response:
  status: success
  data:
    user:
      id: "{{user_id}}"
      email: "{{email | trim | lower}}"
      full_name: "{{first_name | capitalize}} {{last_name | capitalize}}"
      initials: JD
      member_since: "{{created_at}}"
    stats:
      order_count: "{{orders | length}}"
      total_spent: "{{(orders | length) * 50}}"
      average_order: "{{((orders | length) * 50) / (orders | length) | round}}"
      loyalty_tier: "{{(orders | length) > 10 ? 'gold' : ((orders | length) > 5 ? 'silver' : 'bronze')}}"
    preferences:
      notifications: "{{enable_notifications}}"
      newsletter: "{{enable_newsletter}}"
"#;
    
    let route: SimpleRoute = serde_yaml::from_str(yaml).unwrap();
    
    let request = json!({
        "user_id": 12345,
        "email": "  John.Doe@Example.Com  ",
        "first_name": "john",
        "last_name": "doe",
        "created_at": "2024-01-15",
        "orders": [1, 2, 3, 4, 5, 6, 7],
        "enable_notifications": true,
        "enable_newsletter": false
    });
    
    let response = execute_route_with_expressions(&route, request).unwrap();
    
    assert_eq!(response["status"], json!("success"));
    assert_eq!(response["data"]["user"]["id"].as_f64().unwrap(), 12345.0);
    assert_eq!(response["data"]["user"]["email"], json!("john.doe@example.com"));
    assert_eq!(response["data"]["user"]["full_name"], json!("John Doe"));
    assert_eq!(response["data"]["user"]["initials"], json!("JD"));
    assert_eq!(response["data"]["stats"]["order_count"].as_u64().unwrap(), 7);
    assert_eq!(response["data"]["stats"]["total_spent"].as_f64().unwrap(), 350.0);
    assert_eq!(response["data"]["stats"]["average_order"].as_f64().unwrap(), 50.0);
    assert_eq!(response["data"]["stats"]["loyalty_tier"], json!("silver"));
}
