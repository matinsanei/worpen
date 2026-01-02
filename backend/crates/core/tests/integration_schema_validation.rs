// Integration tests for YAML routes with JSON Schema validation

use worpen_core::validation::{validate_input, validate_with_details};
use serde_json::json;

#[test]
fn test_validate_user_registration_schema() {
    let schema = json!({
        "type": "object",
        "properties": {
            "email": {
                "type": "string",
                "format": "email"
            },
            "username": {
                "type": "string",
                "minLength": 3,
                "maxLength": 20
            },
            "password": {
                "type": "string",
                "minLength": 8
            },
            "age": {
                "type": "number",
                "minimum": 13
            }
        },
        "required": ["email", "username", "password"]
    });
    
    // Valid data
    let valid_request = json!({
        "email": "user@example.com",
        "username": "johndoe",
        "password": "secret123",
        "age": 25
    });
    assert!(validate_input(&valid_request, &schema).is_ok());
    
    // Missing required field
    let missing_field = json!({
        "email": "user@example.com",
        "password": "secret123"
    });
    assert!(validate_input(&missing_field, &schema).is_err());
    
    // Invalid email format
    let invalid_email = json!({
        "email": "not-an-email",
        "username": "johndoe",
        "password": "secret123"
    });
    assert!(validate_input(&invalid_email, &schema).is_err());
    
    // Password too short
    let short_password = json!({
        "email": "user@example.com",
        "username": "johndoe",
        "password": "short"
    });
    assert!(validate_input(&short_password, &schema).is_err());
    
    // Age too low
    let underage = json!({
        "email": "user@example.com",
        "username": "johndoe",
        "password": "secret123",
        "age": 10
    });
    assert!(validate_input(&underage, &schema).is_err());
}

#[test]
fn test_validate_product_schema() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "minLength": 1
            },
            "price": {
                "type": "number",
                "minimum": 0
            },
            "quantity": {
                "type": "integer",
                "minimum": 1
            },
            "category": {
                "type": "string",
                "enum": ["electronics", "clothing", "food", "books"]
            },
            "tags": {
                "type": "array",
                "items": {"type": "string"},
                "minItems": 1
            }
        },
        "required": ["name", "price", "quantity", "category"]
    });
    
    // Valid product
    let valid_product = json!({
        "name": "Laptop",
        "price": 999.99,
        "quantity": 5,
        "category": "electronics",
        "tags": ["computer", "portable"]
    });
    assert!(validate_input(&valid_product, &schema).is_ok());
    
    // Negative price
    let negative_price = json!({
        "name": "Laptop",
        "price": -100,
        "quantity": 5,
        "category": "electronics"
    });
    assert!(validate_input(&negative_price, &schema).is_err());
    
    // Invalid category
    let invalid_category = json!({
        "name": "Laptop",
        "price": 999.99,
        "quantity": 5,
        "category": "invalid"
    });
    assert!(validate_input(&invalid_category, &schema).is_err());
    
    // Empty tags array
    let empty_tags = json!({
        "name": "Laptop",
        "price": 999.99,
        "quantity": 5,
        "category": "electronics",
        "tags": []
    });
    assert!(validate_input(&empty_tags, &schema).is_err());
}

#[test]
fn test_validate_nested_schema() {
    let schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "email": {"type": "string", "format": "email"}
                },
                "required": ["name", "email"]
            },
            "address": {
                "type": "object",
                "properties": {
                    "street": {"type": "string"},
                    "city": {"type": "string"},
                    "zip": {"type": "string", "pattern": "^\\d{5}$"}
                },
                "required": ["street", "city", "zip"]
            }
        },
        "required": ["user", "address"]
    });
    
    // Valid nested data
    let valid_data = json!({
        "user": {
            "name": "John Doe",
            "email": "john@example.com"
        },
        "address": {
            "street": "123 Main St",
            "city": "New York",
            "zip": "10001"
        }
    });
    assert!(validate_input(&valid_data, &schema).is_ok());
    
    // Missing nested required field
    let missing_nested = json!({
        "user": {
            "name": "John Doe"
        },
        "address": {
            "street": "123 Main St",
            "city": "New York",
            "zip": "10001"
        }
    });
    assert!(validate_input(&missing_nested, &schema).is_err());
    
    // Invalid zip pattern
    let invalid_zip = json!({
        "user": {
            "name": "John Doe",
            "email": "john@example.com"
        },
        "address": {
            "street": "123 Main St",
            "city": "New York",
            "zip": "ABC"
        }
    });
    assert!(validate_input(&invalid_zip, &schema).is_err());
}

#[test]
fn test_validate_array_of_objects() {
    let schema = json!({
        "type": "object",
        "properties": {
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "number"},
                        "name": {"type": "string", "minLength": 1}
                    },
                    "required": ["id", "name"]
                },
                "minItems": 1
            }
        },
        "required": ["items"]
    });
    
    // Valid array
    let valid_data = json!({
        "items": [
            {"id": 1, "name": "Item 1"},
            {"id": 2, "name": "Item 2"}
        ]
    });
    assert!(validate_input(&valid_data, &schema).is_ok());
    
    // Empty array
    let empty_array = json!({"items": []});
    assert!(validate_input(&empty_array, &schema).is_err());
    
    // Missing property in array item
    let missing_property = json!({
        "items": [
            {"id": 1, "name": "Item 1"},
            {"id": 2}
        ]
    });
    assert!(validate_input(&missing_property, &schema).is_err());
}

#[test]
fn test_validate_with_details_multiple_errors() {
    let schema = json!({
        "type": "object",
        "properties": {
            "email": {"type": "string", "format": "email"},
            "age": {"type": "number", "minimum": 0},
            "name": {"type": "string", "minLength": 1}
        },
        "required": ["email", "age", "name"]
    });
    
    let invalid_data = json!({
        "email": "invalid-email",
        "age": -5,
        "name": ""
    });
    
    let result = validate_with_details(&invalid_data, &schema);
    assert!(!result.valid);
    assert!(result.errors.len() >= 2); // Multiple validation errors
    
    // Check that we can convert to Result
    assert!(result.to_result().is_err());
}

#[test]
fn test_validate_checkout_request() {
    let schema = json!({
        "type": "object",
        "properties": {
            "customer_email": {
                "type": "string",
                "format": "email"
            },
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "product_id": {"type": "string"},
                        "quantity": {"type": "integer", "minimum": 1},
                        "price": {"type": "number", "minimum": 0}
                    },
                    "required": ["product_id", "quantity", "price"]
                },
                "minItems": 1
            },
            "payment_method": {
                "type": "string",
                "enum": ["card", "paypal", "crypto"]
            }
        },
        "required": ["customer_email", "items", "payment_method"]
    });
    
    // Valid checkout
    let valid_checkout = json!({
        "customer_email": "customer@example.com",
        "items": [
            {"product_id": "prod-123", "quantity": 2, "price": 29.99},
            {"product_id": "prod-456", "quantity": 1, "price": 49.99}
        ],
        "payment_method": "card"
    });
    assert!(validate_input(&valid_checkout, &schema).is_ok());
    
    // Invalid payment method
    let invalid_payment = json!({
        "customer_email": "customer@example.com",
        "items": [
            {"product_id": "prod-123", "quantity": 2, "price": 29.99}
        ],
        "payment_method": "cash"
    });
    assert!(validate_input(&invalid_payment, &schema).is_err());
    
    // Negative quantity
    let negative_quantity = json!({
        "customer_email": "customer@example.com",
        "items": [
            {"product_id": "prod-123", "quantity": -1, "price": 29.99}
        ],
        "payment_method": "card"
    });
    assert!(validate_input(&negative_quantity, &schema).is_err());
}

#[test]
fn test_validate_api_key_request() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "minLength": 3,
                "maxLength": 50
            },
            "scopes": {
                "type": "array",
                "items": {
                    "type": "string",
                    "enum": ["read", "write", "admin"]
                },
                "minItems": 1,
                "uniqueItems": true
            },
            "expires_in_days": {
                "type": "integer",
                "minimum": 1,
                "maximum": 365
            }
        },
        "required": ["name", "scopes"]
    });
    
    // Valid request
    let valid_request = json!({
        "name": "My API Key",
        "scopes": ["read", "write"],
        "expires_in_days": 30
    });
    assert!(validate_input(&valid_request, &schema).is_ok());
    
    // Invalid scope
    let invalid_scope = json!({
        "name": "My API Key",
        "scopes": ["read", "invalid"]
    });
    assert!(validate_input(&invalid_scope, &schema).is_err());
    
    // Too long expiry
    let long_expiry = json!({
        "name": "My API Key",
        "scopes": ["read"],
        "expires_in_days": 500
    });
    assert!(validate_input(&long_expiry, &schema).is_err());
}

#[test]
fn test_validate_webhook_config() {
    let schema = json!({
        "type": "object",
        "properties": {
            "url": {
                "type": "string",
                "format": "uri"
            },
            "events": {
                "type": "array",
                "items": {"type": "string"},
                "minItems": 1
            },
            "active": {
                "type": "boolean"
            },
            "secret": {
                "type": "string",
                "minLength": 16
            }
        },
        "required": ["url", "events"]
    });
    
    // Valid webhook
    let valid_webhook = json!({
        "url": "https://example.com/webhook",
        "events": ["user.created", "user.updated"],
        "active": true,
        "secret": "very-secret-key-123"
    });
    assert!(validate_input(&valid_webhook, &schema).is_ok());
    
    // Short secret
    let short_secret = json!({
        "url": "https://example.com/webhook",
        "events": ["user.created"],
        "secret": "short"
    });
    assert!(validate_input(&short_secret, &schema).is_err());
}

#[test]
fn test_validate_phone_number() {
    let schema = json!({
        "type": "object",
        "properties": {
            "phone": {
                "type": "string",
                "pattern": "^\\+?[1-9]\\d{1,14}$"
            },
            "country_code": {
                "type": "string",
                "pattern": "^[A-Z]{2}$"
            }
        },
        "required": ["phone"]
    });
    
    // Valid phone
    let valid_phone = json!({
        "phone": "+1234567890",
        "country_code": "US"
    });
    assert!(validate_input(&valid_phone, &schema).is_ok());
    
    // Invalid phone format
    let invalid_phone = json!({
        "phone": "not-a-phone"
    });
    assert!(validate_input(&invalid_phone, &schema).is_err());
    
    // Invalid country code
    let invalid_country = json!({
        "phone": "+1234567890",
        "country_code": "USA"
    });
    assert!(validate_input(&invalid_country, &schema).is_err());
}
