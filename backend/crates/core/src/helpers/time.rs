use chrono::{DateTime, Utc};

/// Get current timestamp in ISO 8601 format
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
