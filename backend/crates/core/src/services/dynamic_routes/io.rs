use serde_json::Value;
use proto::models::DynamicRouteExecutionContext;
use std::collections::HashMap;
use super::utils::{resolve_string, resolve_variables};

pub async fn handle_http_request(
    url: &str,
    method: &str,
    body: &Option<Value>,
    headers: &Option<HashMap<String, String>>,
    timeout_ms: &Option<u64>,
    context: &mut DynamicRouteExecutionContext,
    steps: &mut Vec<String>,
) -> Result<Value, String> {
    steps.push(format!("HTTP {} request to: {}", method, url));
    
    // Resolve URL with variables
    let resolved_url = resolve_string(url, context);
    
    // Create HTTP client
    let mut client_builder = reqwest::Client::builder();
    
    // Set timeout if specified
    if let Some(timeout) = timeout_ms {
        client_builder = client_builder.timeout(std::time::Duration::from_millis(*timeout));
    }
    
    let client = client_builder.build().map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Build request
    let mut request_builder = match method {
        "GET" => client.get(&resolved_url),
        "POST" => client.post(&resolved_url),
        "PUT" => client.put(&resolved_url),
        "DELETE" => client.delete(&resolved_url),
        "PATCH" => client.patch(&resolved_url),
        "HEAD" => client.head(&resolved_url),
        _ => return Err(format!("Unsupported HTTP method: {}", method)),
    };
    
    // Add headers if present
    if let Some(hdrs) = headers {
        for (key, value) in hdrs {
            let resolved_value = resolve_string(value, context);
            request_builder = request_builder.header(key, resolved_value);
        }
    }
    
    // Add body if present
    if let Some(b) = body {
        let resolved_body = resolve_variables(b, context);
        let body_string = match resolved_body {
            Value::String(s) => s,
            _ => serde_json::to_string(&resolved_body).map_err(|e| format!("Failed to serialize body: {}", e))?,
        };
        request_builder = request_builder.body(body_string);
    }
    
    // Send request and await response
    let response = request_builder.send().await.map_err(|e| format!("HTTP request failed: {}", e))?;
    let status_code = response.status().as_u16();
    let response_text = response.text().await.map_err(|e| format!("Failed to read response body: {}", e))?;
    
    // Try to parse response as JSON, otherwise return as string
    let response_body = match serde_json::from_str::<Value>(&response_text) {
        Ok(json_value) => json_value,
        Err(_) => Value::String(response_text),
    };
    
    let result = serde_json::json!({
        "status": status_code,
        "body": response_body,
        "url": resolved_url,
        "method": method,
    });
    
    context.variables.insert("http_response".to_string(), result.clone());
    Ok(result)
}

pub fn handle_query_db(query: &str, _params: &[Value], context: &mut DynamicRouteExecutionContext, steps: &mut Vec<String>) -> Value {
    steps.push(format!("Execute DB query: {}", query));
    // TODO: Implement real database connection
    let result = serde_json::json!({
        "rows": [],
        "count": 0,
        "query": query,
    });
    context.variables.insert("db_result".to_string(), result.clone());
    result
}

pub fn handle_log(level: &str, message: &str, context: &DynamicRouteExecutionContext, steps: &mut Vec<String>) {
    let resolved_msg = resolve_string(message, context);
    steps.push(format!("[{}] {}", level.to_uppercase(), resolved_msg));
    println!("[{}] {}", level.to_uppercase(), resolved_msg);
}

pub async fn handle_sleep(duration_ms: u64, steps: &mut Vec<String>) {
    steps.push(format!("Sleep for {}ms", duration_ms));
    tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms)).await;
}
