// Advanced pipe filters for expression evaluation
// Phase 2 Days 8-9: String, Array, and Object filters

use serde_json::Value;

/// Apply a pipe filter to a value
pub fn apply_filter(value: &Value, filter_name: &str, args: &[Value]) -> Result<Value, String> {
    match filter_name {
        // String filters
        "upper" => filter_upper(value),
        "lower" => filter_lower(value),
        "trim" => filter_trim(value),
        "replace" => filter_replace(value, args),
        "split" => filter_split(value, args),
        "substring" => filter_substring(value, args),
        "capitalize" => filter_capitalize(value),
        "repeat" => filter_repeat(value, args),
        "reverse" => filter_reverse(value),
        "slice" => filter_slice(value, args),
        
        // Array filters
        "join" => filter_join(value, args),
        "first" => filter_first(value),
        "last" => filter_last(value),
        "sort" => filter_sort(value),
        "unique" => filter_unique(value),
        "length" => filter_length(value),
        "map" => Err("map filter requires evaluator context".to_string()),
        "filter" => Err("filter filter requires evaluator context".to_string()),
        
        // Object filters
        "keys" => filter_keys(value),
        "values" => filter_values(value),
        "has" => filter_has(value, args),
        
        // Type conversion
        "string" => filter_to_string(value),
        "number" => filter_to_number(value),
        "bool" => filter_to_bool(value),
        
        // Math filters (for numbers)
        "abs" => filter_abs(value),
        "round" => filter_round(value),
        "floor" => filter_floor(value),
        "ceil" => filter_ceil(value),
        
        _ => Err(format!("Unknown filter: {}", filter_name)),
    }
}

// ============================================================================
// String Filters
// ============================================================================

fn filter_upper(value: &Value) -> Result<Value, String> {
    match value.as_str() {
        Some(s) => Ok(Value::String(s.to_uppercase())),
        None => Err(format!("upper filter requires string, got {:?}", value)),
    }
}

fn filter_lower(value: &Value) -> Result<Value, String> {
    match value.as_str() {
        Some(s) => Ok(Value::String(s.to_lowercase())),
        None => Err(format!("lower filter requires string, got {:?}", value)),
    }
}

fn filter_trim(value: &Value) -> Result<Value, String> {
    match value.as_str() {
        Some(s) => Ok(Value::String(s.trim().to_string())),
        None => Err(format!("trim filter requires string, got {:?}", value)),
    }
}

fn filter_replace(value: &Value, args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("replace filter requires 2 arguments: (pattern, replacement)".to_string());
    }
    
    let s = value.as_str().ok_or("replace requires string value")?;
    let pattern = args[0].as_str().ok_or("replace pattern must be string")?;
    let replacement = args[1].as_str().ok_or("replace replacement must be string")?;
    
    Ok(Value::String(s.replace(pattern, replacement)))
}

fn filter_split(value: &Value, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() {
        return Err("split filter requires 1 argument: (delimiter)".to_string());
    }
    
    let s = value.as_str().ok_or("split requires string value")?;
    let delimiter = args[0].as_str().ok_or("split delimiter must be string")?;
    
    let parts: Vec<Value> = s
        .split(delimiter)
        .map(|part| Value::String(part.to_string()))
        .collect();
    
    Ok(Value::Array(parts))
}

fn filter_substring(value: &Value, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err("substring filter requires 1-2 arguments: (start, [length])".to_string());
    }
    
    let s = value.as_str().ok_or("substring requires string value")?;
    let start = args[0].as_u64().ok_or("substring start must be number")? as usize;
    
    let result = if args.len() == 2 {
        let length = args[1].as_u64().ok_or("substring length must be number")? as usize;
        s.chars().skip(start).take(length).collect::<String>()
    } else {
        s.chars().skip(start).collect::<String>()
    };
    
    Ok(Value::String(result))
}

fn filter_capitalize(value: &Value) -> Result<Value, String> {
    let s = value.as_str().ok_or("capitalize requires string value")?;
    
    if s.is_empty() {
        return Ok(Value::String(String::new()));
    }
    
    let mut chars = s.chars();
    let first = chars.next().unwrap().to_uppercase().to_string();
    let rest: String = chars.collect();
    
    Ok(Value::String(format!("{}{}", first, rest)))
}

fn filter_repeat(value: &Value, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("repeat filter requires 1 argument: (count)".to_string());
    }
    
    let s = value.as_str().ok_or("repeat requires string value")?;
    let count = args[0].as_u64().ok_or("repeat count must be number")? as usize;
    
    Ok(Value::String(s.repeat(count)))
}

fn filter_reverse(value: &Value) -> Result<Value, String> {
    match value {
        Value::String(s) => {
            let reversed: String = s.chars().rev().collect();
            Ok(Value::String(reversed))
        }
        Value::Array(arr) => {
            let mut reversed = arr.clone();
            reversed.reverse();
            Ok(Value::Array(reversed))
        }
        _ => Err("reverse requires string or array".to_string()),
    }
}

fn filter_slice(value: &Value, args: &[Value]) -> Result<Value, String> {
    if args.is_empty() || args.len() > 2 {
        return Err("slice filter requires 1-2 arguments: (start, [end])".to_string());
    }
    
    match value {
        Value::String(s) => {
            let start = args[0].as_i64().ok_or("slice start must be number")? as usize;
            let chars: Vec<char> = s.chars().collect();
            
            let result = if args.len() == 2 {
                let end = args[1].as_i64().ok_or("slice end must be number")? as usize;
                chars.get(start..end).map(|slice| slice.iter().collect::<String>())
            } else {
                chars.get(start..).map(|slice| slice.iter().collect::<String>())
            };
            
            Ok(Value::String(result.unwrap_or_default()))
        }
        Value::Array(arr) => {
            let start = args[0].as_i64().ok_or("slice start must be number")? as usize;
            
            let result = if args.len() == 2 {
                let end = args[1].as_i64().ok_or("slice end must be number")? as usize;
                arr.get(start..end).map(|slice| slice.to_vec())
            } else {
                arr.get(start..).map(|slice| slice.to_vec())
            };
            
            Ok(Value::Array(result.unwrap_or_default()))
        }
        _ => Err("slice requires string or array".to_string()),
    }
}

// ============================================================================
// Array Filters
// ============================================================================

fn filter_join(value: &Value, args: &[Value]) -> Result<Value, String> {
    let arr = value.as_array().ok_or("join requires array value")?;
    
    let separator = if args.is_empty() {
        ","
    } else {
        args[0].as_str().ok_or("join separator must be string")?
    };
    
    let parts: Vec<String> = arr
        .iter()
        .map(|v| match v {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => serde_json::to_string(v).unwrap_or_default(),
        })
        .collect();
    
    Ok(Value::String(parts.join(separator)))
}

fn filter_first(value: &Value) -> Result<Value, String> {
    let arr = value.as_array().ok_or("first requires array value")?;
    
    arr.first()
        .cloned()
        .ok_or_else(|| "Array is empty".to_string())
}

fn filter_last(value: &Value) -> Result<Value, String> {
    let arr = value.as_array().ok_or("last requires array value")?;
    
    arr.last()
        .cloned()
        .ok_or_else(|| "Array is empty".to_string())
}

fn filter_sort(value: &Value) -> Result<Value, String> {
    let arr = value.as_array().ok_or("sort requires array value")?;
    
    let mut sorted = arr.clone();
    sorted.sort_by(|a, b| {
        // Sort numbers, then strings, then others
        match (a, b) {
            (Value::Number(n1), Value::Number(n2)) => {
                n1.as_f64().partial_cmp(&n2.as_f64()).unwrap_or(std::cmp::Ordering::Equal)
            }
            (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
            (Value::Number(_), _) => std::cmp::Ordering::Less,
            (_, Value::Number(_)) => std::cmp::Ordering::Greater,
            (Value::String(_), _) => std::cmp::Ordering::Less,
            (_, Value::String(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    });
    
    Ok(Value::Array(sorted))
}

fn filter_unique(value: &Value) -> Result<Value, String> {
    let arr = value.as_array().ok_or("unique requires array value")?;
    
    let mut unique: Vec<Value> = Vec::new();
    for item in arr {
        if !unique.contains(item) {
            unique.push(item.clone());
        }
    }
    
    Ok(Value::Array(unique))
}

fn filter_length(value: &Value) -> Result<Value, String> {
    let len = match value {
        Value::String(s) => s.len(),
        Value::Array(a) => a.len(),
        Value::Object(o) => o.len(),
        _ => return Err("length requires string, array, or object".to_string()),
    };
    
    Ok(Value::Number(serde_json::Number::from(len)))
}

// ============================================================================
// Object Filters
// ============================================================================

fn filter_keys(value: &Value) -> Result<Value, String> {
    let obj = value.as_object().ok_or("keys requires object value")?;
    
    let keys: Vec<Value> = obj
        .keys()
        .map(|k| Value::String(k.clone()))
        .collect();
    
    Ok(Value::Array(keys))
}

fn filter_values(value: &Value) -> Result<Value, String> {
    let obj = value.as_object().ok_or("values requires object value")?;
    
    let values: Vec<Value> = obj
        .values()
        .cloned()
        .collect();
    
    Ok(Value::Array(values))
}

fn filter_has(value: &Value, args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("has filter requires 1 argument: (key)".to_string());
    }
    
    let obj = value.as_object().ok_or("has requires object value")?;
    let key = args[0].as_str().ok_or("has key must be string")?;
    
    Ok(Value::Bool(obj.contains_key(key)))
}

// ============================================================================
// Type Conversion Filters
// ============================================================================

fn filter_to_string(value: &Value) -> Result<Value, String> {
    let s = match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            // Format numbers without unnecessary decimals
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else if let Some(u) = n.as_u64() {
                u.to_string()
            } else if let Some(f) = n.as_f64() {
                // Check if it's a whole number
                if f.fract() == 0.0 {
                    (f as i64).to_string()
                } else {
                    f.to_string()
                }
            } else {
                n.to_string()
            }
        }
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    };
    
    Ok(Value::String(s))
}

fn filter_to_number(value: &Value) -> Result<Value, String> {
    match value {
        Value::Number(n) => Ok(Value::Number(n.clone())),
        Value::String(s) => {
            let n: f64 = s.parse().map_err(|_| format!("Cannot convert '{}' to number", s))?;
            Ok(Value::Number(serde_json::Number::from_f64(n).ok_or("Invalid number")?))
        }
        Value::Bool(b) => Ok(Value::Number(serde_json::Number::from(if *b { 1 } else { 0 }))),
        _ => Err(format!("Cannot convert {:?} to number", value)),
    }
}

fn filter_to_bool(value: &Value) -> Result<Value, String> {
    let b = match value {
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(false),
        Value::String(s) => !s.is_empty(),
        Value::Null => false,
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    };
    
    Ok(Value::Bool(b))
}

// ============================================================================
// Math Filters
// ============================================================================

fn filter_abs(value: &Value) -> Result<Value, String> {
    let n = value.as_f64().ok_or("abs requires number value")?;
    Ok(Value::Number(serde_json::Number::from_f64(n.abs()).ok_or("Invalid number")?))
}

fn filter_round(value: &Value) -> Result<Value, String> {
    let n = value.as_f64().ok_or("round requires number value")?;
    Ok(Value::Number(serde_json::Number::from_f64(n.round()).ok_or("Invalid number")?))
}

fn filter_floor(value: &Value) -> Result<Value, String> {
    let n = value.as_f64().ok_or("floor requires number value")?;
    Ok(Value::Number(serde_json::Number::from_f64(n.floor()).ok_or("Invalid number")?))
}

fn filter_ceil(value: &Value) -> Result<Value, String> {
    let n = value.as_f64().ok_or("ceil requires number value")?;
    Ok(Value::Number(serde_json::Number::from_f64(n.ceil()).ok_or("Invalid number")?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    // String filter tests
    #[test]
    fn test_filter_upper() {
        let result = apply_filter(&json!("hello"), "upper", &[]).unwrap();
        assert_eq!(result, json!("HELLO"));
    }
    
    #[test]
    fn test_filter_lower() {
        let result = apply_filter(&json!("WORLD"), "lower", &[]).unwrap();
        assert_eq!(result, json!("world"));
    }
    
    #[test]
    fn test_filter_trim() {
        let result = apply_filter(&json!("  hello  "), "trim", &[]).unwrap();
        assert_eq!(result, json!("hello"));
    }
    
    #[test]
    fn test_filter_replace() {
        let result = apply_filter(&json!("hello world"), "replace", &[json!("world"), json!("rust")]).unwrap();
        assert_eq!(result, json!("hello rust"));
    }
    
    #[test]
    fn test_filter_split() {
        let result = apply_filter(&json!("a,b,c"), "split", &[json!(",")]).unwrap();
        assert_eq!(result, json!(["a", "b", "c"]));
    }
    
    #[test]
    fn test_filter_substring() {
        let result = apply_filter(&json!("hello"), "substring", &[json!(1), json!(3)]).unwrap();
        assert_eq!(result, json!("ell"));
    }
    
    #[test]
    fn test_filter_capitalize() {
        let result = apply_filter(&json!("hello"), "capitalize", &[]).unwrap();
        assert_eq!(result, json!("Hello"));
    }
    
    #[test]
    fn test_filter_repeat() {
        let result = apply_filter(&json!("ha"), "repeat", &[json!(3)]).unwrap();
        assert_eq!(result, json!("hahaha"));
    }
    
    #[test]
    fn test_filter_reverse_string() {
        let result = apply_filter(&json!("hello"), "reverse", &[]).unwrap();
        assert_eq!(result, json!("olleh"));
    }
    
    #[test]
    fn test_filter_slice_string() {
        let result = apply_filter(&json!("hello"), "slice", &[json!(1), json!(4)]).unwrap();
        assert_eq!(result, json!("ell"));
    }
    
    // Array filter tests
    #[test]
    fn test_filter_join() {
        let result = apply_filter(&json!(["a", "b", "c"]), "join", &[json!("-")]).unwrap();
        assert_eq!(result, json!("a-b-c"));
    }
    
    #[test]
    fn test_filter_first() {
        let result = apply_filter(&json!([1, 2, 3]), "first", &[]).unwrap();
        assert_eq!(result, json!(1));
    }
    
    #[test]
    fn test_filter_last() {
        let result = apply_filter(&json!([1, 2, 3]), "last", &[]).unwrap();
        assert_eq!(result, json!(3));
    }
    
    #[test]
    fn test_filter_sort() {
        let result = apply_filter(&json!([3, 1, 2]), "sort", &[]).unwrap();
        assert_eq!(result, json!([1, 2, 3]));
    }
    
    #[test]
    fn test_filter_unique() {
        let result = apply_filter(&json!([1, 2, 2, 3, 1]), "unique", &[]).unwrap();
        assert_eq!(result, json!([1, 2, 3]));
    }
    
    #[test]
    fn test_filter_length() {
        assert_eq!(apply_filter(&json!("hello"), "length", &[]).unwrap(), json!(5));
        assert_eq!(apply_filter(&json!([1, 2, 3]), "length", &[]).unwrap(), json!(3));
    }
    
    // Object filter tests
    #[test]
    fn test_filter_keys() {
        let result = apply_filter(&json!({"a": 1, "b": 2}), "keys", &[]).unwrap();
        let keys = result.as_array().unwrap();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&json!("a")));
        assert!(keys.contains(&json!("b")));
    }
    
    #[test]
    fn test_filter_values() {
        let result = apply_filter(&json!({"a": 1, "b": 2}), "values", &[]).unwrap();
        let values = result.as_array().unwrap();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&json!(1)));
        assert!(values.contains(&json!(2)));
    }
    
    #[test]
    fn test_filter_has() {
        assert_eq!(apply_filter(&json!({"a": 1}), "has", &[json!("a")]).unwrap(), json!(true));
        assert_eq!(apply_filter(&json!({"a": 1}), "has", &[json!("b")]).unwrap(), json!(false));
    }
    
    // Type conversion tests
    #[test]
    fn test_filter_to_string() {
        assert_eq!(apply_filter(&json!(42), "string", &[]).unwrap(), json!("42"));
        assert_eq!(apply_filter(&json!(true), "string", &[]).unwrap(), json!("true"));
    }
    
    #[test]
    fn test_filter_to_number() {
        assert_eq!(apply_filter(&json!("42"), "number", &[]).unwrap().as_f64().unwrap(), 42.0);
        assert_eq!(apply_filter(&json!(true), "number", &[]).unwrap(), json!(1));
    }
    
    #[test]
    fn test_filter_to_bool() {
        assert_eq!(apply_filter(&json!(1), "bool", &[]).unwrap(), json!(true));
        assert_eq!(apply_filter(&json!(0), "bool", &[]).unwrap(), json!(false));
        assert_eq!(apply_filter(&json!("hello"), "bool", &[]).unwrap(), json!(true));
    }
    
    // Math filter tests
    #[test]
    fn test_filter_abs() {
        assert_eq!(apply_filter(&json!(-5), "abs", &[]).unwrap().as_f64().unwrap(), 5.0);
    }
    
    #[test]
    fn test_filter_round() {
        assert_eq!(apply_filter(&json!(3.7), "round", &[]).unwrap().as_f64().unwrap(), 4.0);
    }
    
    #[test]
    fn test_filter_floor() {
        assert_eq!(apply_filter(&json!(3.7), "floor", &[]).unwrap().as_f64().unwrap(), 3.0);
    }
    
    #[test]
    fn test_filter_ceil() {
        assert_eq!(apply_filter(&json!(3.2), "ceil", &[]).unwrap().as_f64().unwrap(), 4.0);
    }
}
