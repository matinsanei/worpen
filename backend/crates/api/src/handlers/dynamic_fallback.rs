use axum::{
    extract::State,
    response::{IntoResponse, Response},
    http::{StatusCode, Method, Request},
    Json,
    body::Body,
};
use crate::state::AppState;
use serde_json::Value;

/// Fallback handler برای dynamic routes
/// این handler همه request های ثبت‌نشده رو میگیره و چک میکنه آیا dynamic route هست
#[axum::debug_handler]
pub async fn dynamic_route_fallback(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Response {
    let path = req.uri().path().to_string();
    let method = req.method().clone();
    
    tracing::info!("Dynamic fallback: {} {}", method, path);
    
    // پیدا کردن route با path و method
    match find_dynamic_route(&state, &path, &method).await {
        Some(route) => {
            // اجرای route
            match execute_dynamic_route(&state, route, req).await {
                Ok(response) => response,
                Err(e) => {
                    tracing::error!("Error executing dynamic route: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": "Route execution failed",
                            "message": e
                        }))
                    ).into_response()
                }
            }
        }
        None => {
            // اگه route پیدا نشد، 404 برگردون
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Not Found",
                    "message": format!("No route found for {} {}", method, path)
                }))
            ).into_response()
        }
    }
}

async fn find_dynamic_route(
    state: &AppState,
    path: &str,
    method: &Method,
) -> Option<proto::models::RouteDefinition> {
    // لیست گرفتن همه route ها
    let routes = state.dynamic_route_service.list_routes().await.ok()?;
    
    // پیدا کردن route با path و method مطابق
    routes.into_iter().find(|route| {
        let route_method = match &route.method {
            proto::models::HttpMethod::GET => "GET",
            proto::models::HttpMethod::POST => "POST",
            proto::models::HttpMethod::PUT => "PUT",
            proto::models::HttpMethod::DELETE => "DELETE",
            proto::models::HttpMethod::PATCH => "PATCH",
        };
        route.path == path && 
        route_method == method.as_str() &&
        route.enabled
    })
}

async fn execute_dynamic_route(
    state: &AppState,
    route: proto::models::RouteDefinition,
    req: Request<Body>,
) -> Result<Response, String> {
    // Extract request data
    let (parts, body) = req.into_parts();
    
    // Parse body
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| format!("Failed to read body: {}", e))?;
    
    let request_data: Value = if body_bytes.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&body_bytes)
            .unwrap_or_else(|_| serde_json::json!({}))
    };
    
    // Extract query parameters
    let query_params: std::collections::HashMap<String, String> = parts
        .uri
        .query()
        .map(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_default();
    
    // Extract headers
    let headers: std::collections::HashMap<String, String> = parts
        .headers
        .iter()
        .map(|(name, value)| {
            (name.to_string(), value.to_str().unwrap_or("").to_string())
        })
        .collect();
    
    // Extract path parameters by parsing route path
    let path_params = extract_path_params(&route.path, parts.uri.path());
    
    // Create request object for variable resolution
    let request_object = serde_json::json!({
        "payload": request_data,
        "headers": headers,
        "params": path_params
    });
    
    // ساخت execution context
    let mut context = proto::models::DynamicRouteExecutionContext {
        route_id: route.id.clone(),
        variables: std::collections::HashMap::new(),
        request_payload: Some(request_data),
        path_params,
        query_params,
        functions: std::collections::HashMap::new(),
        loop_control: proto::models::LoopControl::default(),
        error_context: None,
    };
    
    // Inject request object into variables for template resolution
    context.variables.insert("request".to_string(), request_object);
    
    // اجرای logic
    let result = state.dynamic_route_service
        .execute_route_logic(&route.logic, &mut context)
        .await
        .map_err(|e| format!("Execution error: {}", e))?;
    
    // تبدیل نتیجه به response
    let response_json = result.as_object()
        .and_then(|obj| {
            if obj.contains_key("status") || obj.contains_key("message") || obj.contains_key("data") {
                Some(result.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| serde_json::json!({
            "status": 200,
            "data": result
        }));
    
    // Extract status code
    let status_code = response_json
        .get("status")
        .and_then(|s| s.as_u64())
        .unwrap_or(200) as u16;
    
    let status = StatusCode::from_u16(status_code)
        .unwrap_or(StatusCode::OK);
    
    // اگه html field داره، به صورت HTML برگردون
    if let Some(html) = response_json.get("html").and_then(|h| h.as_str()) {
        use axum::response::Html;
        Ok((status, Html(html.to_string())).into_response())
    } else {
        Ok((status, Json(response_json)).into_response())
    }
}

/// Extract path parameters from route path and actual request path
fn extract_path_params(route_path: &str, request_path: &str) -> std::collections::HashMap<String, String> {
    let mut params = std::collections::HashMap::new();
    
    // Split paths into segments
    let route_segments: Vec<&str> = route_path.trim_start_matches('/').split('/').collect();
    let request_segments: Vec<&str> = request_path.trim_start_matches('/').split('/').collect();
    
    // If lengths don't match, return empty params
    if route_segments.len() != request_segments.len() {
        return params;
    }
    
    // Compare segments
    for (route_seg, req_seg) in route_segments.iter().zip(request_segments.iter()) {
        if route_seg.starts_with('{') && route_seg.ends_with('}') {
            // This is a parameter
            let param_name = &route_seg[1..route_seg.len()-1];
            params.insert(param_name.to_string(), req_seg.to_string());
        } else if route_seg != req_seg {
            // Static segments don't match, return empty params
            return std::collections::HashMap::new();
        }
    }
    
    params
}
