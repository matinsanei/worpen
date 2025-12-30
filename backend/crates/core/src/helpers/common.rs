use serde_json::Value;

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
