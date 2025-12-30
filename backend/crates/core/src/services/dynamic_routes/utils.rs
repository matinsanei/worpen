use proto::models::DynamicRouteExecutionContext;
use serde_json::Value;
use regex::Regex;
use once_cell::sync::Lazy;

// Static Regex compilation to avoid overhead on every call
static EXPR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{([^}]+)\}").unwrap());
static VAR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{\{([^}]+)\}\}").unwrap());

// Helper to extract value from JSON path
pub fn get_json_path(value: &Value, path: &str) -> Value {
    if path.is_empty() {
        return value.clone();
    }
    
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;
    
    for part in parts {
        match current {
            Value::Object(map) => {
                if let Some(v) = map.get(part) {
                    current = v;
                } else {
                    return Value::Null;
                }
            },
            Value::Array(arr) => {
                if let Ok(idx) = part.parse::<usize>() {
                    if let Some(v) = arr.get(idx) {
                        current = v;
                    } else {
                        return Value::Null;
                    }
                } else {
                    return Value::Null;
                }
            },
            _ => return Value::Null,
        }
    }
    
    current.clone()
}

// Helper function to resolve variables in strings
pub fn resolve_string(template: &str, context: &DynamicRouteExecutionContext) -> String {
    // Optimization: Early return if no template patterns detected
    if !template.contains("{{") && !template.contains("${") {
        return template.to_string();
    }

    let mut result = template.to_string();
    
    // First resolve error context variables (error.message, error.code)
    if let Some(error_ctx) = &context.error_context {
        if result.contains("{{error.") {
            result = result.replace("{{error.message}}", &error_ctx.message);
            if let Some(code) = &error_ctx.code {
                result = result.replace("{{error.code}}", code);
            } else {
                result = result.replace("{{error.code}}", "UNKNOWN_ERROR");
            }
        }
    }
    
    // Then resolve from request payload (for top-level fields like {{order_id}})
    if let Some(payload) = &context.request_payload {
        if result.contains("{{") {
            if let Some(obj) = payload.as_object() {
                for (key, value) in obj {
                    let placeholder = format!("{{{{{}}}}}", key);
                    if result.contains(&placeholder) {
                        let replacement = match value {
                            Value::String(s) => s.clone(),
                            other => other.to_string(),
                        };
                        result = result.replace(&placeholder, &replacement);
                    }
                }
            }
        }
    }
    
    // Then resolve ${...} expressions (evaluate them)
    if result.contains("${") {
        result = EXPR_RE.replace_all(&result, |caps: &regex::Captures| {
            let expr = &caps[1];
            // Convert context variables to HashMap for expression evaluation
            // TODO: Optimize this conversion or allow expression engine to use context directly
            let mut expr_context = std::collections::HashMap::new();
            for (key, value) in &context.variables {
                expr_context.insert(key.clone(), value.clone());
            }
            // Try to evaluate the expression
            match crate::expression::template::evaluate_expression(expr, &expr_context) {
                Ok(value) => match value {
                    Value::String(s) => s,
                    other => other.to_string(),
                },
                Err(_) => caps[0].to_string(), // Keep original if evaluation fails
            }
        }).to_string();
    }
    
    // Then resolve regular variables (including nested paths like order.status)
    // Use regex to find all {{...}} patterns
    if result.contains("{{") {
        result = VAR_RE.replace_all(&result, |caps: &regex::Captures| {
            let path = &caps[1];
            
            // Check if it's a nested path (contains dot)
            if path.contains('.') {
                let parts: Vec<&str> = path.split('.').collect();
                if let Some(root_var) = context.variables.get(parts[0]) {
                    // Use get_json_path for nested access
                    let nested_value = get_json_path(root_var, &parts[1..].join("."));
                    return match nested_value {
                        Value::String(s) => s,
                        Value::Null => caps[0].to_string(), // Keep original if null
                        other => other.to_string(),
                    };
                }
            } else {
                // Simple variable lookup
                if let Some(value) = context.variables.get(path) {
                    return match value {
                        Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                }
            }
            
            // If not found, keep original
            caps[0].to_string()
        }).to_string();
    }
    
    result
}

// Helper function to resolve variables in JSON values
pub fn resolve_variables(value: &Value, context: &DynamicRouteExecutionContext) -> Value {
    match value {
        Value::String(s) => {
            // Optimization: check start/end chars before string ops
            // Check if it's a pure expression ${...}
            if s.starts_with("${") && s.ends_with("}") && s.len() > 3 {
                let expr = &s[2..s.len()-1];
                // Build context for expression evaluation
                let mut expr_context = std::collections::HashMap::new();
                for (key, val) in &context.variables {
                    expr_context.insert(key.clone(), val.clone());
                }
                // Try to evaluate the expression
                match crate::expression::template::evaluate_expression(expr, &expr_context) {
                    Ok(evaluated_value) => evaluated_value,
                    Err(_) => Value::String(resolve_string(s, context)),
                }
            } else {
                Value::String(resolve_string(s, context))
            }
        },
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| resolve_variables(v, context)).collect())
        },
        Value::Object(obj) => {
            let mut resolved_obj = serde_json::Map::new();
            for (k, v) in obj {
                resolved_obj.insert(k.clone(), resolve_variables(v, context));
            }
            Value::Object(resolved_obj)
        },
        other => other.clone(),
    }
}

// Enhanced condition evaluation
pub fn evaluate_condition(condition: &str, context: &DynamicRouteExecutionContext) -> bool {
    // Optimization: Check for simple boolean literals before resolving string
    if condition == "true" { return true; }
    if condition == "false" { return false; }

    let resolved = resolve_string(condition, context);
    
    // Support multiple operators
    if resolved.contains("==") {
        let parts: Vec<&str> = resolved.split("==").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            return parts[0] == parts[1];
        }
    }
    
    if resolved.contains("!=") {
        let parts: Vec<&str> = resolved.split("!=").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            return parts[0] != parts[1];
        }
    }
    
    if resolved.contains(">=") {
        let parts: Vec<&str> = resolved.split(">=").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left >= right;
            }
        }
    }
    
    if resolved.contains("<=") {
        let parts: Vec<&str> = resolved.split("<=").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left <= right;
            }
        }
    }
    
    if resolved.contains(">") {
        let parts: Vec<&str> = resolved.split(">").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left > right;
            }
        }
    }
    
    if resolved.contains("<") {
        let parts: Vec<&str> = resolved.split("<").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left < right;
            }
        }
    }
    
    // Boolean literals
    if resolved == "true" { return true; }
    if resolved == "false" { return false; }
    
    // Default
    true
}
