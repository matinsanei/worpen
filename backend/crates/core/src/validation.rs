/// JSON Schema validation for YAML route inputs
/// 
/// Validates request data against JSON Schema definitions
/// defined in YAML routes.

use jsonschema::{Draft, JSONSchema};
use serde_json::Value;

/// Validate input data against a JSON Schema
/// 
/// # Arguments
/// * `data` - The input data to validate
/// * `schema` - The JSON Schema definition
/// 
/// # Returns
/// * `Ok(())` - Validation passed
/// * `Err(String)` - Validation failed with error messages
/// 
/// # Example
/// ```
/// use serde_json::json;
/// use core::validation::validate_input;
/// 
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "email": {
///             "type": "string",
///             "format": "email"
///         },
///         "age": {
///             "type": "number",
///             "minimum": 0
///         }
///     },
///     "required": ["email"]
/// });
/// 
/// let data = json!({
///     "email": "user@example.com",
///     "age": 25
/// });
/// 
/// assert!(validate_input(&data, &schema).is_ok());
/// ```
pub fn validate_input(data: &Value, schema: &Value) -> Result<(), String> {
    // Compile schema
    let compiled = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema)
        .map_err(|e| format!("Invalid schema: {}", e))?;
    
    // Validate data and collect errors
    let validation_result = compiled.validate(data);
    
    match validation_result {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .map(|e| format!("{} at {}", e, e.instance_path))
                .collect();
            
            Err(format!("Validation failed:\n- {}", error_messages.join("\n- ")))
        }
    }
}

/// Validate input data and return detailed validation results
/// 
/// Similar to `validate_input` but returns structured error information
pub fn validate_with_details(data: &Value, schema: &Value) -> ValidationResult {
    let compiled = match JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema)
    {
        Ok(schema) => schema,
        Err(e) => {
            return ValidationResult {
                valid: false,
                errors: vec![ValidationError {
                    path: String::new(),
                    message: format!("Invalid schema: {}", e),
                    schema_path: String::new(),
                }],
            };
        }
    };
    
    let validation_result = compiled.validate(data);
    
    match validation_result {
        Ok(_) => ValidationResult {
            valid: true,
            errors: vec![],
        },
        Err(errors) => ValidationResult {
            valid: false,
            errors: errors
                .map(|e| ValidationError {
                    path: e.instance_path.to_string(),
                    message: e.to_string(),
                    schema_path: e.schema_path.to_string(),
                })
                .collect(),
        },
    }
}

/// Structured validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub schema_path: String,
}

impl ValidationResult {
    /// Convert to Result type
    pub fn to_result(&self) -> Result<(), String> {
        if self.valid {
            Ok(())
        } else {
            let messages: Vec<String> = self
                .errors
                .iter()
                .map(|e| format!("{} at {}", e.message, e.path))
                .collect();
            Err(format!("Validation failed:\n- {}", messages.join("\n- ")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_simple_object() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number"}
            },
            "required": ["name"]
        });

        let valid_data = json!({"name": "John", "age": 30});
        assert!(validate_input(&valid_data, &schema).is_ok());

        let invalid_data = json!({"age": 30}); // missing required "name"
        assert!(validate_input(&invalid_data, &schema).is_err());
    }

    #[test]
    fn test_validate_with_format() {
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {
                    "type": "string",
                    "format": "email"
                }
            }
        });

        let valid_data = json!({"email": "user@example.com"});
        assert!(validate_input(&valid_data, &schema).is_ok());

        let invalid_data = json!({"email": "not-an-email"});
        assert!(validate_input(&invalid_data, &schema).is_err());
    }

    #[test]
    fn test_validate_number_constraints() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {
                    "type": "number",
                    "minimum": 0,
                    "maximum": 120
                },
                "price": {
                    "type": "number",
                    "minimum": 0
                }
            }
        });

        let valid_data = json!({"age": 25, "price": 99.99});
        assert!(validate_input(&valid_data, &schema).is_ok());

        let invalid_age = json!({"age": -5, "price": 10});
        assert!(validate_input(&invalid_age, &schema).is_err());

        let invalid_price = json!({"age": 25, "price": -10});
        assert!(validate_input(&invalid_price, &schema).is_err());
    }

    #[test]
    fn test_validate_string_constraints() {
        let schema = json!({
            "type": "object",
            "properties": {
                "username": {
                    "type": "string",
                    "minLength": 3,
                    "maxLength": 20
                },
                "password": {
                    "type": "string",
                    "minLength": 8
                }
            }
        });

        let valid_data = json!({
            "username": "john",
            "password": "secret123"
        });
        assert!(validate_input(&valid_data, &schema).is_ok());

        let short_username = json!({
            "username": "ab",
            "password": "secret123"
        });
        assert!(validate_input(&short_username, &schema).is_err());

        let short_password = json!({
            "username": "john",
            "password": "short"
        });
        assert!(validate_input(&short_password, &schema).is_err());
    }

    #[test]
    fn test_validate_array() {
        let schema = json!({
            "type": "object",
            "properties": {
                "tags": {
                    "type": "array",
                    "items": {"type": "string"},
                    "minItems": 1,
                    "maxItems": 5
                }
            }
        });

        let valid_data = json!({"tags": ["rust", "backend"]});
        assert!(validate_input(&valid_data, &schema).is_ok());

        let empty_array = json!({"tags": []});
        assert!(validate_input(&empty_array, &schema).is_err());

        let too_many_items = json!({"tags": ["a", "b", "c", "d", "e", "f"]});
        assert!(validate_input(&too_many_items, &schema).is_err());

        let wrong_type = json!({"tags": [1, 2, 3]});
        assert!(validate_input(&wrong_type, &schema).is_err());
    }

    #[test]
    fn test_validate_nested_object() {
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
                }
            }
        });

        let valid_data = json!({
            "user": {
                "name": "John",
                "email": "john@example.com"
            }
        });
        assert!(validate_input(&valid_data, &schema).is_ok());

        let missing_field = json!({
            "user": {
                "name": "John"
            }
        });
        assert!(validate_input(&missing_field, &schema).is_err());
    }

    #[test]
    fn test_validate_enum() {
        let schema = json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["active", "inactive", "pending"]
                }
            }
        });

        let valid_data = json!({"status": "active"});
        assert!(validate_input(&valid_data, &schema).is_ok());

        let invalid_enum = json!({"status": "unknown"});
        assert!(validate_input(&invalid_enum, &schema).is_err());
    }

    #[test]
    fn test_validate_pattern() {
        let schema = json!({
            "type": "object",
            "properties": {
                "phone": {
                    "type": "string",
                    "pattern": "^\\+?[1-9]\\d{1,14}$"
                }
            }
        });

        let valid_data = json!({"phone": "+1234567890"});
        assert!(validate_input(&valid_data, &schema).is_ok());

        let invalid_pattern = json!({"phone": "not-a-phone"});
        assert!(validate_input(&invalid_pattern, &schema).is_err());
    }

    #[test]
    fn test_validate_with_details() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number", "minimum": 0}
            },
            "required": ["name"]
        });

        let valid_data = json!({"name": "John", "age": 30});
        let result = validate_with_details(&valid_data, &schema);
        assert!(result.valid);
        assert_eq!(result.errors.len(), 0);

        let invalid_data = json!({"age": -5});
        let result = validate_with_details(&invalid_data, &schema);
        assert!(!result.valid);
        assert!(result.errors.len() > 0);
    }

    #[test]
    fn test_multiple_validation_errors() {
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
        assert!(result.errors.len() >= 2); // Multiple errors expected
    }

    #[test]
    fn test_additional_properties() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "additionalProperties": false
        });

        let valid_data = json!({"name": "John"});
        assert!(validate_input(&valid_data, &schema).is_ok());

        let extra_property = json!({"name": "John", "age": 30});
        assert!(validate_input(&extra_property, &schema).is_err());
    }
}
