/// Helper functions library for YAML Dynamic Routes
/// 
/// Provides reusable utility functions for common operations:
/// - UUID generation
/// - Password hashing
/// - Date/time operations
/// - Random value generation
/// - Encoding/decoding
/// - JSON operations

use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

/// Generate a new UUID v4
/// 
/// # Example
/// ```
/// use core::helpers::generate_uuid;
/// 
/// let id = generate_uuid();
/// assert_eq!(id.len(), 36); // UUID format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
/// ```
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Hash a password using bcrypt-like simple hashing (for demo purposes)
/// 
/// Note: In production, use proper bcrypt/argon2 libraries
pub fn hash_password(password: &str) -> String {
    // Simple hash for demo (in production, use bcrypt/argon2)
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    format!("hashed_{:x}", hasher.finish())
}

/// Get current timestamp in ISO 8601 format
/// 
/// # Example
/// ```
/// use core::helpers::now_iso;
/// 
/// let timestamp = now_iso();
/// assert!(timestamp.contains("T"));
/// assert!(timestamp.contains("Z") || timestamp.contains("+"));
/// ```
pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

/// Get current Unix timestamp (seconds since epoch)
pub fn now_unix() -> i64 {
    Utc::now().timestamp()
}

/// Get current Unix timestamp in milliseconds
pub fn now_unix_ms() -> i64 {
    Utc::now().timestamp_millis()
}

/// Format a Unix timestamp to ISO 8601 string with custom format
pub fn format_timestamp(iso_string: &str, format: &str) -> Result<String, String> {
    let dt = DateTime::parse_from_rfc3339(iso_string)
        .map_err(|e| format!("Parse error: {}", e))?;
    Ok(dt.format(format).to_string())
}

/// Parse ISO 8601 string to Unix timestamp
pub fn parse_iso_timestamp(iso_string: &str) -> Result<i64, String> {
    let dt = DateTime::parse_from_rfc3339(iso_string)
        .map_err(|e| format!("Parse error: {}", e))?;
    Ok(dt.timestamp())
}

/// Get current date in YYYY-MM-DD format
pub fn today() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

/// Get current time in HH:MM:SS format
pub fn now_time() -> String {
    Utc::now().format("%H:%M:%S").to_string()
}

/// Generate random integer between min and max (inclusive)
pub fn random_int(min: i64, max: i64) -> i64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    
    let random_state = RandomState::new();
    let mut hasher = random_state.build_hasher();
    Utc::now().timestamp_nanos_opt().unwrap_or(0).hash(&mut hasher);
    let hash = hasher.finish();
    
    min + ((hash % ((max - min + 1) as u64)) as i64)
}

/// Generate random float between min and max
pub fn random_float(min: f64, max: f64) -> f64 {
    let rand = random_float_01();
    min + (rand * (max - min))
}

/// Generate random float between 0.0 and 1.0
fn random_float_01() -> f64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    
    let random_state = RandomState::new();
    let mut hasher = random_state.build_hasher();
    Utc::now().timestamp_nanos_opt().unwrap_or(0).hash(&mut hasher);
    let hash = hasher.finish();
    
    (hash as f64) / (u64::MAX as f64)
}

/// Base64 encode a string
pub fn base64_encode(input: &str) -> String {
    use std::fmt::Write;
    
    let bytes = input.as_bytes();
    let mut result = String::new();
    
    // Simple base64 encoding
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;
        
        write!(&mut result, "{}", CHARSET[b1] as char).unwrap();
        write!(&mut result, "{}", CHARSET[b2] as char).unwrap();
        
        if chunk.len() > 1 {
            write!(&mut result, "{}", CHARSET[b3] as char).unwrap();
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            write!(&mut result, "{}", CHARSET[b4] as char).unwrap();
        } else {
            result.push('=');
        }
    }
    
    result
}

/// URL encode a string
pub fn url_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}

/// URL decode a string
pub fn url_decode(input: &str) -> Result<String, String> {
    let bytes = input.as_bytes();
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                result.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i+1..i+3])
                    .map_err(|_| "Invalid UTF-8")?;
                let byte = u8::from_str_radix(hex, 16)
                    .map_err(|_| "Invalid hex")?;
                result.push(byte);
                i += 3;
            }
            _ => {
                result.push(bytes[i]);
                i += 1;
            }
        }
    }
    
    String::from_utf8(result).map_err(|_| "Invalid UTF-8".to_string())
}

/// Base64 decode a string
pub fn base64_decode(input: &str) -> Result<String, String> {
    // Simple base64 decoding (for demo purposes)
    // In production, use base64 crate
    let input = input.trim_end_matches('=');
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut bytes = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    
    for chunk in chars.chunks(4) {
        let mut values = Vec::new();
        for &ch in chunk {
            let val = charset.find(ch)
                .ok_or_else(|| format!("Invalid base64 character: {}", ch))?;
            values.push(val as u8);
        }
        
        while values.len() < 4 {
            values.push(0);
        }
        
        bytes.push((values[0] << 2) | (values[1] >> 4));
        if chunk.len() > 2 {
            bytes.push((values[1] << 4) | (values[2] >> 2));
        }
        if chunk.len() > 3 {
            bytes.push((values[2] << 6) | values[3]);
        }
    }
    
    String::from_utf8(bytes).map_err(|_| "Invalid UTF-8".to_string())
}

/// Slugify a string (convert to URL-friendly slug)
pub fn slugify(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            ' ' | '_' | '-' => '-',
            _ => '-',
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Generate a random string of given length
pub fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    (0..length)
        .map(|_| {
            let idx = random_int(0, (CHARSET.len() - 1) as i64) as usize;
            CHARSET[idx] as char
        })
        .collect()
}

/// Convert JSON string to Value
pub fn json_parse(json_string: &str) -> Result<Value, String> {
    serde_json::from_str(json_string)
        .map_err(|e| format!("JSON parse error: {}", e))
}

/// Convert Value to JSON string
pub fn json_stringify(value: &Value) -> Result<String, String> {
    serde_json::to_string(value)
        .map_err(|e| format!("JSON stringify error: {}", e))
}

/// Pretty print JSON with indentation
pub fn json_pretty(value: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(value)
        .map_err(|e| format!("JSON pretty error: {}", e))
}

/// Calculate MD5 hash of string (for demo purposes)
pub fn md5_hash(input: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Escape HTML entities
pub fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Unescape HTML entities
pub fn html_unescape(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
}

/// Extract domain from email
pub fn email_domain(email: &str) -> Option<String> {
    email
        .split('@')
        .nth(1)
        .map(|s| s.to_string())
}

/// Extract username from email
pub fn email_username(email: &str) -> Option<String> {
    if !email.contains('@') {
        return None;
    }
    email
        .split('@')
        .next()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

/// Check if string is valid email format
pub fn is_email(input: &str) -> bool {
    let parts: Vec<&str> = input.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let username = parts[0];
    let domain = parts[1];
    
    !username.is_empty() && !domain.is_empty() && domain.contains('.')
}

/// Check if string is valid URL format
pub fn is_url(input: &str) -> bool {
    input.starts_with("http://") || input.starts_with("https://")
}

/// Truncate string to max length with ellipsis
pub fn truncate(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else if max_length <= 3 {
        input.chars().take(max_length).collect()
    } else {
        format!("{}...", input.chars().take(max_length - 3).collect::<String>())
    }
}

/// Count words in string
pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}

/// Extract initials from name
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .collect()
}

/// Add days to a timestamp
pub fn add_days(iso_string: &str, days: i32) -> Result<String, String> {
    use chrono::Duration;
    
    let dt = DateTime::parse_from_rfc3339(iso_string)
        .map_err(|e| format!("Parse error: {}", e))?;
    let future = dt + Duration::days(days as i64);
    Ok(future.to_rfc3339())
}

/// Add hours to a timestamp
pub fn add_hours(iso_string: &str, hours: i32) -> Result<String, String> {
    use chrono::Duration;
    
    let dt = DateTime::parse_from_rfc3339(iso_string)
        .map_err(|e| format!("Parse error: {}", e))?;
    let future = dt + Duration::hours(hours as i64);
    Ok(future.to_rfc3339())
}

/// Format number with thousand separators
pub fn format_number(num: f64, decimals: usize) -> String {
    let formatted = format!("{:.prec$}", num, prec = decimals);
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer = parts[0];
    let decimal = if parts.len() > 1 { parts[1] } else { "" };
    
    let formatted_integer: String = integer
        .chars()
        .rev()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                vec![',', c]
            } else {
                vec![c]
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    
    if decimal.is_empty() {
        formatted_integer
    } else {
        format!("{}.{}", formatted_integer, decimal)
    }
}

/// Format bytes to human readable size
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes < 1024 {
        return format!("{} B", bytes);
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_hash_password() {
        let hash1 = hash_password("password123");
        let hash2 = hash_password("password123");
        assert!(hash1.starts_with("hashed_"));
        assert_eq!(hash1, hash2); // Same input = same hash
    }

    #[test]
    fn test_now_iso() {
        let timestamp = now_iso();
        assert!(timestamp.contains('T'));
        assert!(timestamp.ends_with('Z') || timestamp.contains('+'));
    }

    #[test]
    fn test_now_unix() {
        let timestamp = now_unix();
        assert!(timestamp > 1700000000); // After 2023
    }

    #[test]
    fn test_format_timestamp() {
        let now = now_iso();
        let formatted = format_timestamp(&now, "%Y-%m-%d").unwrap();
        assert_eq!(formatted.len(), 10); // YYYY-MM-DD
    }

    #[test]
    fn test_today() {
        let date = today();
        assert_eq!(date.len(), 10); // YYYY-MM-DD
        assert!(date.contains('-'));
    }

    #[test]
    fn test_now_time() {
        let time = now_time();
        assert_eq!(time.len(), 8); // HH:MM:SS
        assert!(time.contains(':'));
    }

    #[test]
    fn test_random_int() {
        let num = random_int(1, 10);
        assert!(num >= 1 && num <= 10);
    }

    #[test]
    fn test_random_float() {
        let num = random_float(0.0, 1.0);
        assert!(num >= 0.0 && num <= 1.0);
    }

    #[test]
    fn test_base64_encode() {
        let encoded = base64_encode("hello");
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_url_encode() {
        let encoded = url_encode("hello world");
        assert_eq!(encoded, "hello+world");
        
        let encoded2 = url_encode("test@email.com");
        assert!(encoded2.contains("%40"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Test_String 123"), "test-string-123");
        assert_eq!(slugify("Multiple   Spaces"), "multiple-spaces");
    }

    #[test]
    fn test_random_string() {
        let s = random_string(10);
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_json_parse() {
        let result = json_parse(r#"{"key": "value"}"#).unwrap();
        assert_eq!(result["key"], "value");
    }

    #[test]
    fn test_json_stringify() {
        let value = serde_json::json!({"key": "value"});
        let json = json_stringify(&value).unwrap();
        assert!(json.contains("key"));
        assert!(json.contains("value"));
    }

    #[test]
    fn test_md5_hash() {
        let hash = md5_hash("test");
        assert!(!hash.is_empty());
        assert_eq!(md5_hash("test"), md5_hash("test")); // Deterministic
    }

    #[test]
    fn test_html_escape() {
        let escaped = html_escape("<script>alert('xss')</script>");
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
    }

    #[test]
    fn test_email_domain() {
        assert_eq!(email_domain("user@example.com").unwrap(), "example.com");
        assert!(email_domain("invalid").is_none());
    }

    #[test]
    fn test_email_username() {
        assert_eq!(email_username("user@example.com").unwrap(), "user");
        assert!(email_username("invalid").is_none());
    }

    #[test]
    fn test_is_email() {
        assert!(is_email("user@example.com"));
        assert!(!is_email("invalid"));
        assert!(!is_email("@example.com"));
        assert!(!is_email("user@"));
    }

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com"));
        assert!(is_url("http://example.com"));
        assert!(!is_url("example.com"));
        assert!(!is_url("ftp://example.com"));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello world", 5), "he...");
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello", 5), "hello");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count("one"), 1);
        assert_eq!(word_count("  multiple   spaces  "), 2);
    }

    #[test]
    fn test_initials() {
        assert_eq!(initials("John Doe"), "JD");
        assert_eq!(initials("Alice Bob Charlie"), "ABC");
        assert_eq!(initials("single"), "S");
    }

    #[test]
    fn test_add_days() {
        let now = now_iso();
        let future = add_days(&now, 7).unwrap();
        assert!(future.contains('T'));
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1000.0, 2), "1,000.00");
        assert_eq!(format_number(1000000.0, 2), "1,000,000.00");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
    }
}
