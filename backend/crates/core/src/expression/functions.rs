use serde_json::{Value, Number};
use std::collections::HashMap;
use std::sync::Arc;
use crate::helpers;

// Use Arc for shared ownership to allow cheap cloning of the function registry
pub type EvaluatorFunction = Arc<dyn Fn(Vec<Value>) -> Result<Value, String> + Send + Sync>;

use once_cell::sync::Lazy;

// Static cache of built-in functions
static BUILTIN_FUNCTIONS: Lazy<HashMap<String, EvaluatorFunction>> = Lazy::new(|| {
    let mut functions: HashMap<String, EvaluatorFunction> = HashMap::new();

    // Math functions
    functions.insert("abs".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("abs requires 1 argument".to_string());
        }
        let n = args[0].as_f64().ok_or("abs requires number")?;
        Ok(Value::Number(Number::from_f64(n.abs()).ok_or("Invalid number")?))
    }));
    
    functions.insert("max".to_string(), Arc::new(|args| {
        if args.is_empty() {
            return Err("max requires at least 1 argument".to_string());
        }
        let mut max = args[0].as_f64().ok_or("max requires numbers")?;
        for arg in &args[1..] {
            let n = arg.as_f64().ok_or("max requires numbers")?;
            if n > max {
                max = n;
            }
        }
        Ok(Value::Number(Number::from_f64(max).ok_or("Invalid number")?))
    }));
    
    functions.insert("min".to_string(), Arc::new(|args| {
        if args.is_empty() {
            return Err("min requires at least 1 argument".to_string());
        }
        let mut min = args[0].as_f64().ok_or("min requires numbers")?;
        for arg in &args[1..] {
            let n = arg.as_f64().ok_or("min requires numbers")?;
            if n < min {
                min = n;
            }
        }
        Ok(Value::Number(Number::from_f64(min).ok_or("Invalid number")?))
    }));
    
    // String functions
    functions.insert("len".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("len requires 1 argument".to_string());
        }
        let len = match &args[0] {
            Value::String(s) => s.len(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            _ => return Err("len requires string, array, or object".to_string()),
        };
        Ok(Value::Number(Number::from(len)))
    }));
    
    // UUID functions
    functions.insert("uuid".to_string(), Arc::new(|args| {
        if !args.is_empty() {
            return Err("uuid requires no arguments".to_string());
        }
        Ok(Value::String(helpers::generate_uuid()))
    }));
    
    // Hashing functions
    functions.insert("hash_password".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("hash_password requires 1 argument".to_string());
        }
        let password = args[0].as_str().ok_or("hash_password requires string")?;
        Ok(Value::String(helpers::hash_password(password)))
    }));
    
    functions.insert("md5".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("md5 requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("md5 requires string")?;
        Ok(Value::String(helpers::md5_hash(text)))
    }));
    
    // DateTime functions
    functions.insert("now".to_string(), Arc::new(|args| {
        if !args.is_empty() {
            return Err("now requires no arguments".to_string());
        }
        Ok(Value::String(helpers::now_iso()))
    }));
    
    functions.insert("now_unix".to_string(), Arc::new(|args| {
        if !args.is_empty() {
            return Err("now_unix requires no arguments".to_string());
        }
        Ok(Value::Number(Number::from(helpers::now_unix())))
    }));
    
    functions.insert("today".to_string(), Arc::new(|args| {
        if !args.is_empty() {
            return Err("today requires no arguments".to_string());
        }
        Ok(Value::String(helpers::today()))
    }));
    
    functions.insert("now_time".to_string(), Arc::new(|args| {
        if !args.is_empty() {
            return Err("now_time requires no arguments".to_string());
        }
        Ok(Value::String(helpers::now_time()))
    }));
    
    functions.insert("add_days".to_string(), Arc::new(|args| {
        if args.len() != 2 {
            return Err("add_days requires 2 arguments".to_string());
        }
        let timestamp = args[0].as_str().ok_or("add_days requires string timestamp")?;
        let days = args[1].as_i64().ok_or("add_days requires integer days")? as i32;
        helpers::add_days(timestamp, days)
            .map(Value::String)
            .map_err(|e| e.to_string())
    }));
    
    functions.insert("add_hours".to_string(), Arc::new(|args| {
        if args.len() != 2 {
            return Err("add_hours requires 2 arguments".to_string());
        }
        let timestamp = args[0].as_str().ok_or("add_hours requires string timestamp")?;
        let hours = args[1].as_i64().ok_or("add_hours requires integer hours")? as i32;
        helpers::add_hours(timestamp, hours)
            .map(Value::String)
            .map_err(|e| e.to_string())
    }));
    
    functions.insert("format_timestamp".to_string(), Arc::new(|args| {
        if args.len() != 2 {
            return Err("format_timestamp requires 2 arguments".to_string());
        }
        let timestamp = args[0].as_str().ok_or("format_timestamp requires string timestamp")?;
        let format = args[1].as_str().ok_or("format_timestamp requires string format")?;
        helpers::format_timestamp(timestamp, format)
            .map(Value::String)
            .map_err(|e| e.to_string())
    }));
    
    functions.insert("parse_timestamp".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("parse_timestamp requires 1 argument".to_string());
        }
        let timestamp = args[0].as_str().ok_or("parse_timestamp requires string")?;
        helpers::parse_iso_timestamp(timestamp)
            .map(|ts| Value::Number(Number::from(ts)))
            .map_err(|e| e.to_string())
    }));
    
    // Random functions
    functions.insert("random_int".to_string(), Arc::new(|args| {
        if args.len() != 2 {
            return Err("random_int requires 2 arguments".to_string());
        }
        let min = args[0].as_i64().ok_or("random_int requires integer min")?;
        let max = args[1].as_i64().ok_or("random_int requires integer max")?;
        Ok(Value::Number(Number::from(helpers::random_int(min, max))))
    }));
    
    functions.insert("random_float".to_string(), Arc::new(|args| {
        if args.len() != 2 {
            return Err("random_float requires 2 arguments".to_string());
        }
        let min = args[0].as_f64().ok_or("random_float requires number min")?;
        let max = args[1].as_f64().ok_or("random_float requires number max")?;
        Ok(Value::Number(Number::from_f64(helpers::random_float(min, max)).ok_or("Invalid number")?))
    }));
    
    functions.insert("random_string".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("random_string requires 1 argument".to_string());
        }
        let length = args[0].as_u64().ok_or("random_string requires positive integer")? as usize;
        Ok(Value::String(helpers::random_string(length)))
    }));
    
    // Encoding functions
    functions.insert("base64_encode".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("base64_encode requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("base64_encode requires string")?;
        Ok(Value::String(helpers::base64_encode(text)))
    }));
    
    functions.insert("base64_decode".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("base64_decode requires 1 argument".to_string());
        }
        let encoded = args[0].as_str().ok_or("base64_decode requires string")?;
        helpers::base64_decode(encoded)
            .map(Value::String)
            .map_err(|e| e.to_string())
    }));
    
    functions.insert("url_encode".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("url_encode requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("url_encode requires string")?;
        Ok(Value::String(helpers::url_encode(text)))
    }));
    
    functions.insert("url_decode".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("url_decode requires 1 argument".to_string());
        }
        let encoded = args[0].as_str().ok_or("url_decode requires string")?;
        helpers::url_decode(encoded)
            .map(Value::String)
            .map_err(|e| e.to_string())
    }));
    
    functions.insert("html_escape".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("html_escape requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("html_escape requires string")?;
        Ok(Value::String(helpers::html_escape(text)))
    }));
    
    functions.insert("html_unescape".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("html_unescape requires 1 argument".to_string());
        }
        let escaped = args[0].as_str().ok_or("html_unescape requires string")?;
        Ok(Value::String(helpers::html_unescape(escaped)))
    }));
    
    // String utility functions
    functions.insert("slugify".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("slugify requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("slugify requires string")?;
        Ok(Value::String(helpers::slugify(text)))
    }));
    
    functions.insert("truncate".to_string(), Arc::new(|args| {
        if args.len() != 2 {
            return Err("truncate requires 2 arguments".to_string());
        }
        let text = args[0].as_str().ok_or("truncate requires string")?;
        let max_len = args[1].as_u64().ok_or("truncate requires positive integer")? as usize;
        Ok(Value::String(helpers::truncate(text, max_len)))
    }));
    
    functions.insert("word_count".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("word_count requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("word_count requires string")?;
        Ok(Value::Number(Number::from(helpers::word_count(text))))
    }));
    
    functions.insert("initials".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("initials requires 1 argument".to_string());
        }
        let name = args[0].as_str().ok_or("initials requires string")?;
        Ok(Value::String(helpers::initials(name)))
    }));
    
    // Email utility functions
    functions.insert("email_domain".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("email_domain requires 1 argument".to_string());
        }
        let email = args[0].as_str().ok_or("email_domain requires string")?;
        helpers::email_domain(email)
            .map(Value::String)
            .ok_or_else(|| "Invalid email format".to_string())
    }));
    
    functions.insert("email_username".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("email_username requires 1 argument".to_string());
        }
        let email = args[0].as_str().ok_or("email_username requires string")?;
        helpers::email_username(email)
            .map(Value::String)
            .ok_or_else(|| "Invalid email format".to_string())
    }));
    
    functions.insert("is_email".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("is_email requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("is_email requires string")?;
        Ok(Value::Bool(helpers::is_email(text)))
    }));
    
    functions.insert("is_url".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("is_url requires 1 argument".to_string());
        }
        let text = args[0].as_str().ok_or("is_url requires string")?;
        Ok(Value::Bool(helpers::is_url(text)))
    }));
    
    // JSON utility functions
    functions.insert("json_parse".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("json_parse requires 1 argument".to_string());
        }
        let json_str = args[0].as_str().ok_or("json_parse requires string")?;
        helpers::json_parse(json_str)
            .map_err(|e| e.to_string())
    }));
    
    functions.insert("json_stringify".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("json_stringify requires 1 argument".to_string());
        }
        helpers::json_stringify(&args[0])
            .map(Value::String)
            .map_err(|e| e.to_string())
    }));
    
    functions.insert("json_pretty".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("json_pretty requires 1 argument".to_string());
        }
        helpers::json_pretty(&args[0])
            .map(Value::String)
            .map_err(|e| e.to_string())
    }));
    
    // Formatting functions
    functions.insert("format_number".to_string(), Arc::new(|args| {
        if args.is_empty() || args.len() > 2 {
            return Err("format_number requires 1 or 2 arguments".to_string());
        }
        let number = args[0].as_f64().ok_or("format_number requires number")?;
        let decimals = if args.len() == 2 {
            args[1].as_u64().ok_or("format_number decimals requires positive integer")? as usize
        } else {
            2
        };
        Ok(Value::String(helpers::format_number(number, decimals)))
    }));
    
    functions.insert("format_bytes".to_string(), Arc::new(|args| {
        if args.len() != 1 {
            return Err("format_bytes requires 1 argument".to_string());
        }
        let bytes = args[0].as_u64().ok_or("format_bytes requires positive integer")?;
        Ok(Value::String(helpers::format_bytes(bytes)))
    }));
    
    functions
});

pub fn get_builtin_functions() -> HashMap<String, EvaluatorFunction> {
    // Zero-cost clone of the map containing Arcs
    BUILTIN_FUNCTIONS.clone()
}
