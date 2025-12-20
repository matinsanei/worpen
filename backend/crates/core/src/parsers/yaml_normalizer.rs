/// YAML Normalizer - Converts YAML to JSON-compatible structure
/// 
/// This module handles the serde_yaml enum limitation by converting
/// YAML structure to a format that can be deserialized properly.

use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

/// Convert YAML value to JSON value with enum handling
pub fn yaml_to_json(yaml: &str) -> Result<String, String> {
    // First parse as YAML
    let yaml_value: YamlValue = serde_yaml::from_str(yaml)
        .map_err(|e| format!("YAML parse error: {}", e))?;
    
    // Convert to JSON value (this handles the mapping)
    let json_value: JsonValue = yaml_to_json_value(&yaml_value)?;
    
    // Serialize to JSON string
    serde_json::to_string(&json_value)
        .map_err(|e| format!("JSON serialization error: {}", e))
}

/// Recursively convert YAML value to JSON value
fn yaml_to_json_value(yaml: &YamlValue) -> Result<JsonValue, String> {
    match yaml {
        YamlValue::Null => Ok(JsonValue::Null),
        YamlValue::Bool(b) => Ok(JsonValue::Bool(*b)),
        YamlValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(JsonValue::Number(i.into()))
            } else if let Some(u) = n.as_u64() {
                Ok(JsonValue::Number(u.into()))
            } else if let Some(f) = n.as_f64() {
                Ok(JsonValue::Number(
                    serde_json::Number::from_f64(f)
                        .ok_or_else(|| "Invalid float number".to_string())?
                ))
            } else {
                Err("Invalid number format".to_string())
            }
        }
        YamlValue::String(s) => Ok(JsonValue::String(s.clone())),
        YamlValue::Sequence(seq) => {
            let arr: Result<Vec<JsonValue>, String> = seq
                .iter()
                .map(yaml_to_json_value)
                .collect();
            Ok(JsonValue::Array(arr?))
        }
        YamlValue::Mapping(map) => {
            let mut obj = serde_json::Map::new();
            
            for (key, value) in map.iter() {
                let key_str = match key {
                    YamlValue::String(s) => s.clone(),
                    _ => return Err("Map keys must be strings".to_string()),
                };
                
                obj.insert(key_str, yaml_to_json_value(value)?);
            }
            
            Ok(JsonValue::Object(obj))
        }
        YamlValue::Tagged(tagged) => {
            // Handle YAML tags (e.g., !Return)
            // Convert the tag to string and extract variant name
            let tag_str = format!("{:?}", tagged.tag);
            
            // Extract the tag name (remove "Tag(" prefix and ")" suffix if present)
            let tag_name = if tag_str.starts_with("Tag(") && tag_str.ends_with(")") {
                tag_str[4..tag_str.len()-1].trim_matches('"').to_string()
            } else {
                tag_str
            };
            
            // Remove ! prefix if present and convert to snake_case
            let variant_name = if tag_name.starts_with('!') {
                convert_to_snake_case(&tag_name[1..])
            } else {
                convert_to_snake_case(&tag_name)
            };
            
            let value = &tagged.value;
            
            // Create JSON structure: { "variant_name": {...} }
            let mut obj = serde_json::Map::new();
            obj.insert(variant_name, yaml_to_json_value(value)?);
            
            Ok(JsonValue::Object(obj))
        }
    }
}

/// Convert PascalCase or camelCase to snake_case
fn convert_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            // Add underscore before uppercase if:
            // 1. Not at start (i > 0)
            // 2. Previous was lowercase OR next is lowercase (to handle acronyms like "JSONOp" → "json_op")
            let next_is_lower = s.chars().nth(i + 1).map_or(false, |next| next.is_lowercase());
            
            if i > 0 && (!prev_was_upper || next_is_lower) {
                result.push('_');
            }
            
            result.push(c.to_lowercase().next().unwrap());
            prev_was_upper = true;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tagged_return_with_nested_object() {
        let yaml = r#"logic:
  - !Return
    value: {message: "Success", code: 200, data: {user: "admin", role: "owner"}}"#;
        let json = yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["logic"][0]["return"]["value"]["message"], "Success");
        assert_eq!(parsed["logic"][0]["return"]["value"]["code"], 200);
    }

    #[test]
    fn test_multiple_tagged_operations() {
        let yaml = r#"logic:
  - !QueryDb
    query: "SELECT * FROM users"
    params: [1, "test"]
  - !Return
    value: {status: "ok"}"#;
        let json = yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["logic"][0].get("query_db").is_some());
        assert!(parsed["logic"][1].get("return").is_some());
    }

    #[test]
    fn test_snake_case_conversion() {
        assert_eq!(convert_to_snake_case("Return"), "return");
        assert_eq!(convert_to_snake_case("QueryDb"), "query_db");
        assert_eq!(convert_to_snake_case("HttpRequest"), "http_request");
        assert_eq!(convert_to_snake_case("JSONOp"), "json_op"); // Acronyms stay together
        assert_eq!(convert_to_snake_case("already_snake"), "already_snake");
        assert_eq!(convert_to_snake_case("HTMLParser"), "html_parser");
        assert_eq!(convert_to_snake_case("parseXMLFile"), "parse_xml_file");
    }

    #[test]
    fn test_nested_arrays_and_objects() {
        let yaml = r#"data:
  users:
    - {id: 1, name: "Alice", tags: ["admin", "active"]}
    - {id: 2, name: "Bob", tags: ["user"]}"#;
        let json = yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["data"]["users"][0]["tags"][0], "admin");
        assert_eq!(parsed["data"]["users"][1]["name"], "Bob");
    }

    #[test]
    fn test_tagged_with_null_and_bool() {
        let yaml = r#"logic:
  - !Return
    value: {success: true, error: null, count: 0}"#;
        let json = yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["logic"][0]["return"]["value"]["success"], true);
        assert!(parsed["logic"][0]["return"]["value"]["error"].is_null());
    }

    #[test]
    fn test_complex_route_structure() {
        let yaml = r#"name: "Complex Route"
path: "/api/test"
method: "POST"
logic:
  - !QueryDb
    query: "SELECT * FROM users WHERE id = ?"
    params: [42]
  - !If
    condition: "db_result.length > 0"
    then:
      - !Return
        value: {found: true}
    else:
      - !Return
        value: {found: false}
parameters:
  - {name: "id", type: "integer", required: true}"#;
        let json = yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["name"], "Complex Route");
        assert_eq!(parsed["method"], "POST");
        assert!(parsed["logic"][0].get("query_db").is_some());
        assert!(parsed["logic"][1].get("if").is_some());
    }

    #[test]
    fn test_numbers_float_int_edge_cases() {
        let yaml = r#"values: {int: 42, negative: -100, float: 3.14159, zero: 0, big: 9007199254740991}"#;
        let json = yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["values"]["int"], 42);
        assert_eq!(parsed["values"]["negative"], -100);
        assert_eq!(parsed["values"]["zero"], 0);
    }

    #[test]
    fn test_empty_structures() {
        let yaml = r#"empty_obj: {}
empty_arr: []
has_empty:
  - !Return
    value: {}"#;
        let result = yaml_to_json(yaml);
        assert!(result.is_ok());
        let parsed: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert!(parsed["empty_obj"].is_object());
        assert!(parsed["empty_arr"].is_array());
    }

    #[test]
    fn test_special_strings() {
        let yaml = r#"strings:
  quote: "He said \"hello\""
  newline: "Line1\nLine2"
  unicode: "مرحبا 👋 Hello"
  empty: """#;
        let result = yaml_to_json(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deeply_nested_tagged() {
        let yaml = r#"logic:
  - !If
    condition: "true"
    then:
      - !QueryDb
        query: "SELECT *"
        params: []
      - !If
        condition: "result"
        then:
          - !Return
            value: {nested: {very: {deep: true}}}"#;
        let json = yaml_to_json(yaml).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["logic"][0]["if"]["then"][1]["if"]["then"][0]["return"]["value"]["nested"]["very"]["deep"].as_bool().unwrap());
    }
}
