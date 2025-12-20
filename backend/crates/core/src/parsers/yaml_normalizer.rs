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
    let mut prev_was_lower = false;
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && prev_was_lower {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_was_lower = false;
        } else {
            result.push(c);
            prev_was_lower = true;
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_yaml_to_json() {
        let yaml = r#"
name: test
value: 123
"#;
        let result = yaml_to_json(yaml).unwrap();
        assert!(result.contains("\"name\":\"test\""));
        assert!(result.contains("\"value\":123"));
    }

    #[test]
    fn test_yaml_with_return() {
        let yaml = r#"
logic:
  - return:
      value:
        message: Hello
"#;
        let result = yaml_to_json(yaml).unwrap();
        // Should convert to proper JSON structure
        assert!(result.contains("\"return\""));
        assert!(result.contains("\"value\""));
    }

    #[test]
    fn test_yaml_array() {
        let yaml = r#"
items:
  - one
  - two
  - three
"#;
        let result = yaml_to_json(yaml).unwrap();
        assert!(result.contains("\"items\""));
    }
}
