use axum::{
    extract::{State, WebSocketUpgrade},
    response::{IntoResponse, Response},
    http::{StatusCode, Method, Request},
    Json,
    body::Body,
};
use crate::state::AppState;
use serde_json::Value;
use proto::models::RouteType;

/// Temporary constant to control dynamic fallback logging
/// Set to false to disable logging, true to enable
const ENABLE_DYNAMIC_FALLBACK_LOGGING: bool = true;

/// Fallback handler برای dynamic routes
/// این handler همه request های ثبت‌نشده رو میگیره و چک میکنه آیا dynamic route هست
#[axum::debug_handler]
pub async fn dynamic_route_fallback(
    State(state): State<AppState>,
    ws: Option<WebSocketUpgrade>,
    req: Request<Body>,
) -> Response {
    let path = req.uri().path().to_string();
    let method = req.method().clone();
    
    if ENABLE_DYNAMIC_FALLBACK_LOGGING {
        tracing::info!("Dynamic fallback: {} {}", method, path);
    }
    
    // پیدا کردن route با path و method
    match find_dynamic_route(&state, &path, &method).await {
        Some(route) => {
            // ✅ CHECK ROUTE TYPE FIRST
            match route.route_type {
                RouteType::WebSocket => {
                    // Dispatch to WebSocket handler
                    if let Some(ws_upgrade) = ws {
                        tracing::info!("Upgrading to WebSocket for route: {}", route.name);
                        return handle_websocket_route(ws_upgrade, route, state).await;
                    } else {
                        tracing::error!("WebSocket route called without upgrade header");
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(serde_json::json!({
                                "error": "WebSocket upgrade required",
                                "message": "This route requires WebSocket protocol"
                            }))
                        ).into_response();
                    }
                }
                RouteType::Http => {
                    // Continue with HTTP logic
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
    
    // Check if result contains enhanced return metadata (value, status, headers, raw)
    if let Some(obj) = result.as_object() {
        if obj.contains_key("value") {
            // Enhanced return with custom response fields
            let value = obj.get("value").unwrap_or(&Value::Null);
            let status_code = obj.get("status")
                .and_then(|s| s.as_u64())
                .unwrap_or(200) as u16;
            let headers = obj.get("headers")
                .and_then(|h| h.as_object())
                .cloned();
            let is_raw = obj.get("raw")
                .and_then(|r| r.as_bool())
                .unwrap_or(false);
            
            let status = StatusCode::from_u16(status_code)
                .unwrap_or(StatusCode::OK);
            
            // If raw=true, send the value directly without wrapping
            if is_raw {
                // Extract raw string content
                let raw_content = match value {
                    Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };
                
                // Build response with custom headers
                let mut response = (status, raw_content).into_response();
                if let Some(header_map) = headers {
                    for (key, val) in header_map {
                        if let Value::String(header_value) = val {
                            if let Ok(header_name) = axum::http::HeaderName::from_bytes(key.as_bytes()) {
                                if let Ok(header_val) = axum::http::HeaderValue::from_str(&header_value) {
                                    response.headers_mut().insert(header_name, header_val);
                                }
                            }
                        }
                    }
                }
                return Ok(response);
            } else {
                // Normal JSON response with custom status and headers
                let mut response = (status, Json(value.clone())).into_response();
                if let Some(header_map) = headers {
                    for (key, val) in header_map {
                        if let Value::String(header_value) = val {
                            if let Ok(header_name) = axum::http::HeaderName::from_bytes(key.as_bytes()) {
                                if let Ok(header_val) = axum::http::HeaderValue::from_str(&header_value) {
                                    response.headers_mut().insert(header_name, header_val);
                                }
                            }
                        }
                    }
                }
                return Ok(response);
            }
        }
    }
    
    // Legacy path: تبدیل نتیجه به response
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

/// Handle WebSocket route upgrade
async fn handle_websocket_route(
    ws: WebSocketUpgrade,
    route: proto::models::RouteDefinition,
    state: AppState,
) -> Response {
    use axum::extract::ws::Message;
    use futures::{sink::SinkExt, stream::StreamExt};
    use tokio::sync::mpsc;
    use uuid::Uuid;
    
    ws.on_upgrade(move |socket| async move {
        let (mut sender, mut receiver) = socket.split();
        let connection_id = Uuid::new_v4().to_string();

        // Get WebSocket manager
        let ws_manager = state
            .dynamic_route_service
            .get_ws_manager()
            .unwrap_or_default();

        // Create channel for outgoing messages
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        // Register connection
        ws_manager.register(connection_id.clone(), tx);

        // Spawn task to forward messages from channel to WebSocket
        let ws_manager_clone = ws_manager.clone();
        let connection_id_clone = connection_id.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
            // Cleanup on disconnect
            ws_manager_clone.unregister(&connection_id_clone);
        });

        // Execute on_connect hook if exists
        if let Some(ref hooks) = route.ws_hooks {
            if !hooks.on_connect.is_empty() {
                let _ = execute_ws_logic(
                    &hooks.on_connect,
                    &state,
                    &ws_manager,
                    &connection_id,
                    None,
                )
                .await;
            }
        }

        // Main message loop
        while let Some(msg_result) = receiver.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    // Execute on_message hook
                    if let Some(ref hooks) = route.ws_hooks {
                        if !hooks.on_message.is_empty() {
                            let _ = execute_ws_logic(
                                &hooks.on_message,
                                &state,
                                &ws_manager,
                                &connection_id,
                                Some(text),
                            )
                            .await;
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(_) => {
                    break;
                }
                _ => {}
            }
        }

        // Execute on_disconnect hook
        if let Some(ref hooks) = route.ws_hooks {
            if !hooks.on_disconnect.is_empty() {
                let _ = execute_ws_logic(
                    &hooks.on_disconnect,
                    &state,
                    &ws_manager,
                    &connection_id,
                    None,
                )
                .await;
            }
        }

        // Unregister connection
        ws_manager.unregister(&connection_id);
    })
    .into_response()
}

/// Execute WebSocket logic operations
async fn execute_ws_logic(
    logic: &[proto::models::LogicOperation],
    state: &AppState,
    ws_manager: &worpen_core::websocket::WebSocketManager,
    connection_id: &str,
    incoming_message: Option<String>,
) -> Result<Value, String> {
    use worpen_core::vm::memory::ExecutionMemory;
    use worpen_core::vm::machine::VirtualMachine;
    use worpen_core::compiler::lowerer::LogicCompiler;
    
    // Compile logic
    let mut compiler = LogicCompiler::new();
    let program = compiler.compile(logic);
    let symbol_table = compiler.get_symbol_table().clone();

    // Create execution memory
    let mut memory = ExecutionMemory::new();

    // Inject {{message}} variable if provided
    if let Some(msg) = incoming_message {
        if let Some(msg_index) = symbol_table.get_index("message") {
            memory.set(msg_index, Value::String(msg));
        }
    }

    // Inject {{connection_id}} variable
    if let Some(conn_index) = symbol_table.get_index("connection_id") {
        memory.set(conn_index, Value::String(connection_id.to_string()));
    }

    // Create VM with WebSocket support
    let db_pool = state.dynamic_route_service.get_db_pool();
    let redis_pool = state.dynamic_route_service.get_redis_pool();

    let mut vm = VirtualMachine::with_all(
        memory,
        symbol_table,
        db_pool,
        redis_pool,
        Some(ws_manager.clone()),
        Some(connection_id.to_string()),
    );

    // Execute
    vm.execute(&program).await
}
