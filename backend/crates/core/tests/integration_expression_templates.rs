// Integration tests for expression templates in YAML routes
// Tests {{variable}} and {{expression | filter}} syntax

use core::expression::resolve_templates;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_yaml_route_with_simple_variables() {
    let mut context = HashMap::new();
    context.insert("user_name".to_string(), json!("John Doe"));
    context.insert("user_email".to_string(), json!("john@example.com"));
    
    let route_def = json!({
        "name": "user_profile",
        "path": "/users/{{user_name}}",
        "response": {
            "message": "Welcome {{user_name}}!",
            "email": "{{user_email}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["path"], json!("/users/John Doe"));
    assert_eq!(result["response"]["message"], json!("Welcome John Doe!"));
    assert_eq!(result["response"]["email"], json!("john@example.com"));
}

#[test]
fn test_yaml_route_with_arithmetic() {
    let mut context = HashMap::new();
    context.insert("price".to_string(), json!(100));
    context.insert("quantity".to_string(), json!(3));
    context.insert("tax_rate".to_string(), json!(0.1));
    
    let route_def = json!({
        "name": "checkout",
        "response": {
            "subtotal": "{{price * quantity}}",
            "tax": "{{price * quantity * tax_rate}}",
            "total": "{{price * quantity * (1 + tax_rate)}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["response"]["subtotal"].as_f64().unwrap(), 300.0);
    assert_eq!(result["response"]["tax"].as_f64().unwrap(), 30.0);
    assert_eq!(result["response"]["total"].as_f64().unwrap(), 330.0);
}

#[test]
fn test_yaml_route_with_pipe_filters() {
    let mut context = HashMap::new();
    context.insert("email".to_string(), json!("  USER@EXAMPLE.COM  "));
    context.insert("name".to_string(), json!("john doe"));
    
    let route_def = json!({
        "name": "user_registration",
        "data": {
            "email": "{{email | trim | lower}}",
            "name": "{{name | capitalize}}",
            "display_name": "{{name | upper}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["data"]["email"], json!("user@example.com"));
    assert_eq!(result["data"]["name"], json!("John doe"));
    assert_eq!(result["data"]["display_name"], json!("JOHN DOE"));
}

#[test]
fn test_yaml_route_with_conditionals() {
    let mut context = HashMap::new();
    context.insert("age".to_string(), json!(25));
    context.insert("is_premium".to_string(), json!(true));
    
    let route_def = json!({
        "response": {
            "status": "{{age >= 18 ? \"verified\" : \"pending\"}}",
            "discount": "{{is_premium ? 0.2 : 0.05}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["response"]["status"], json!("verified"));
    assert_eq!(result["response"]["discount"].as_f64().unwrap(), 0.2);
}

#[test]
fn test_yaml_route_with_array_operations() {
    let mut context = HashMap::new();
    context.insert("items".to_string(), json!([10, 20, 30]));
    context.insert("tags".to_string(), json!(["rust", "api", "backend"]));
    
    let route_def = json!({
        "response": {
            "item_count": "{{items | length}}",
            "first_item": "{{items | first}}",
            "last_item": "{{items | last}}",
            "tags": "{{tags | join(\", \")}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["response"]["item_count"].as_u64().unwrap(), 3);
    assert_eq!(result["response"]["first_item"].as_f64().unwrap(), 10.0);
    assert_eq!(result["response"]["last_item"].as_f64().unwrap(), 30.0);
    assert_eq!(result["response"]["tags"], json!("rust, api, backend"));
}

#[test]
fn test_yaml_route_with_complex_expressions() {
    let mut context = HashMap::new();
    context.insert("base_price".to_string(), json!(100));
    context.insert("quantity".to_string(), json!(5));
    context.insert("discount_rate".to_string(), json!(0.15));
    context.insert("user_level".to_string(), json!("premium"));
    
    let route_def = json!({
        "order": {
            "subtotal": "{{base_price * quantity}}",
            "discount": "{{base_price * quantity * discount_rate}}",
            "final_price": "{{base_price * quantity * (1 - discount_rate)}}",
            "message": "{{user_level | upper}} user gets {{discount_rate * 100}}% off!",
            "savings": "You saved {{base_price * quantity * discount_rate | round}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["order"]["subtotal"].as_f64().unwrap(), 500.0);
    assert_eq!(result["order"]["discount"].as_f64().unwrap(), 75.0);
    assert_eq!(result["order"]["final_price"].as_f64().unwrap(), 425.0);
    assert_eq!(result["order"]["message"], json!("PREMIUM user gets 15% off!"));
    assert_eq!(result["order"]["savings"], json!("You saved 75"));
}

#[test]
fn test_yaml_route_with_nested_structures() {
    let mut context = HashMap::new();
    context.insert("user_id".to_string(), json!(123));
    context.insert("first_name".to_string(), json!("john"));
    context.insert("last_name".to_string(), json!("doe"));
    context.insert("orders".to_string(), json!([1, 2, 3]));
    
    let route_def = json!({
        "user": {
            "id": "{{user_id}}",
            "full_name": "{{first_name | capitalize}} {{last_name | capitalize}}",
            "stats": {
                "order_count": "{{orders | length}}",
                "first_order": "{{orders | first}}"
            }
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["user"]["id"].as_f64().unwrap(), 123.0);
    assert_eq!(result["user"]["full_name"], json!("John Doe"));
    assert_eq!(result["user"]["stats"]["order_count"].as_u64().unwrap(), 3);
    assert_eq!(result["user"]["stats"]["first_order"].as_f64().unwrap(), 1.0);
}

#[test]
fn test_yaml_route_error_handling() {
    let context = HashMap::new();
    
    let route_def = json!({
        "response": {
            "message": "{{undefined_variable}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Undefined variable"));
}

#[test]
fn test_yaml_route_with_functions() {
    let mut context = HashMap::new();
    context.insert("a".to_string(), json!(10));
    context.insert("b".to_string(), json!(20));
    context.insert("c".to_string(), json!(-5));
    
    let route_def = json!({
        "response": {
            "max_value": "{{max(10, 20, 5)}}",
            "min_value": "{{min(10, 20, 5)}}",
            "absolute": "{{abs(-5)}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["response"]["max_value"].as_f64().unwrap(), 20.0);
    assert_eq!(result["response"]["min_value"].as_f64().unwrap(), 5.0);
    assert_eq!(result["response"]["absolute"].as_f64().unwrap(), 5.0);
}

#[test]
fn test_yaml_route_realistic_example() {
    // Simulate a real checkout route with complex expressions
    let mut context = HashMap::new();
    context.insert("cart_items".to_string(), json!([
        {"name": "Laptop", "price": 1000, "qty": 1},
        {"name": "Mouse", "price": 25, "qty": 2}
    ]));
    context.insert("user_email".to_string(), json!("  USER@SHOP.COM  "));
    context.insert("discount_code".to_string(), json!("SAVE20"));
    context.insert("has_discount".to_string(), json!(true));
    
    let route_def = json!({
        "checkout": {
            "email": "{{user_email | trim | lower}}",
            "discount_applied": "{{has_discount}}",
            "discount_message": "{{has_discount ? \"Discount applied!\" : \"No discount\"}}",
            "item_count": "{{cart_items | length}}"
        }
    });
    
    let result = resolve_templates(&route_def, &context).unwrap();
    
    assert_eq!(result["checkout"]["email"], json!("user@shop.com"));
    assert_eq!(result["checkout"]["discount_applied"], json!(true));
    assert_eq!(result["checkout"]["discount_message"], json!("Discount applied!"));
    assert_eq!(result["checkout"]["item_count"].as_u64().unwrap(), 2);
}
