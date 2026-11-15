use super::utils::{resolve_string, resolve_variables};
use proto::models::DynamicRouteExecutionContext;
use serde_json::Value;
use std::collections::HashMap;

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
    let initial_url = resolve_string(url, context);

    let mut current_url = initial_url;
    let mut current_method = method.to_uppercase();
    let mut current_body = body.clone();
    let mut hops = 0;
    const MAX_HOPS: usize = 5;

    let (final_status, final_body, final_url) = loop {
        let mut client_builder =
            reqwest::Client::builder().redirect(reqwest::redirect::Policy::none());

        if let Some(timeout) = timeout_ms {
            client_builder = client_builder.timeout(std::time::Duration::from_millis(*timeout));
        }

        // SSRF & DNS rebinding defense
        if let Some(ref policy) = context.sandbox {
            let (host, port, pinned_ip) =
                crate::sandbox::resolve_and_validate_host(&current_url, policy)
                    .await
                    .map_err(|e| {
                        if hops > 0 {
                            format!("sandbox: redirect to '{}' denied: {}", current_url, e)
                        } else {
                            e
                        }
                    })?;
            client_builder =
                client_builder.resolve(&host, std::net::SocketAddr::new(pinned_ip, port));
        }

        let client = client_builder
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let mut request_builder = match current_method.as_str() {
            "GET" => client.get(&current_url),
            "POST" => client.post(&current_url),
            "PUT" => client.put(&current_url),
            "DELETE" => client.delete(&current_url),
            "PATCH" => client.patch(&current_url),
            "HEAD" => client.head(&current_url),
            _ => return Err(format!("Unsupported HTTP method: {}", current_method)),
        };

        // Add headers if present
        if let Some(hdrs) = headers {
            for (key, value) in hdrs {
                let resolved_value = resolve_string(value, context);
                request_builder = request_builder.header(key, resolved_value);
            }
        }

        // Add body if present
        if let Some(ref b) = current_body {
            let resolved_body = resolve_variables(b, context);
            let body_string = match resolved_body {
                Value::String(s) => s,
                _ => serde_json::to_string(&resolved_body)
                    .map_err(|e| format!("Failed to serialize body: {}", e))?,
            };
            request_builder = request_builder.body(body_string);
        }

        // Send request and await response
        let response = request_builder
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;
        let status_code = response.status().as_u16();

        // Check for redirect status
        if [301, 302, 303, 307, 308].contains(&status_code) {
            if hops >= MAX_HOPS {
                return Err(format!(
                    "sandbox: redirect limit exceeded (max {} hops)",
                    MAX_HOPS
                ));
            }
            if let Some(loc_header) = response.headers().get(reqwest::header::LOCATION) {
                let loc_str = loc_header
                    .to_str()
                    .map_err(|_| "sandbox: invalid Location header in redirect".to_string())?;
                let base = reqwest::Url::parse(&current_url)
                    .map_err(|e| format!("sandbox: invalid URL '{}': {}", current_url, e))?;
                let next_url = base.join(loc_str).map_err(|e| {
                    format!("sandbox: invalid redirect target '{}': {}", loc_str, e)
                })?;
                current_url = next_url.to_string();

                if status_code == 301 || status_code == 302 || status_code == 303 {
                    current_method = "GET".to_string();
                    current_body = None;
                }
                hops += 1;
                continue;
            }
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        let response_body = match serde_json::from_str::<Value>(&response_text) {
            Ok(json_value) => json_value,
            Err(_) => Value::String(response_text),
        };

        break (status_code, response_body, current_url);
    };

    let result = serde_json::json!({
        "status": final_status,
        "body": final_body,
        "url": final_url,
        "method": method,
    });

    context
        .variables
        .insert("http_response".to_string(), result.clone());
    Ok(result)
}

pub fn handle_query_db(
    query: &str,
    _params: &[Value],
    context: &mut DynamicRouteExecutionContext,
    steps: &mut Vec<String>,
) -> Value {
    let resolved_query = resolve_string(query, context);
    steps.push(format!("Execute DB query: {}", resolved_query));

    // Sandbox policy validation for dynamic route query execution
    if let Some(ref policy) = context.sandbox {
        if let Err(e) = crate::sandbox::validate_sql_policy(&resolved_query, policy) {
            let err_obj = serde_json::json!({
                "error": e,
                "rows": [],
                "count": 0,
                "query": resolved_query,
            });
            context
                .variables
                .insert("db_result".to_string(), err_obj.clone());
            return err_obj;
        }
    }

    let result = serde_json::json!({
        "rows": [],
        "count": 0,
        "query": resolved_query,
    });
    context
        .variables
        .insert("db_result".to_string(), result.clone());
    result
}

pub fn handle_log(
    level: &str,
    message: &str,
    context: &DynamicRouteExecutionContext,
    steps: &mut Vec<String>,
) {
    let resolved_msg = resolve_string(message, context);
    steps.push(format!("[{}] {}", level.to_uppercase(), resolved_msg));
    println!("[{}] {}", level.to_uppercase(), resolved_msg);
}

pub async fn handle_sleep(duration_ms: u64, steps: &mut Vec<String>) {
    steps.push(format!("Sleep for {}ms", duration_ms));
    tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms)).await;
}
