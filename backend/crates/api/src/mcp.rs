//! MCP (Model Context Protocol) Server mode — Phase 1.
//!
//! Exposes every Worpen dynamic route as an MCP tool:
//!   POST /mcp  (JSON-RPC 2.0: initialize, tools/list, tools/call)
//!   GET  /mcp/tools (plain REST for debugging)
//!
//! STDIO mode (`worpen mcp serve-stdio`) speaks the same JSON-RPC over stdin/stdout
//! for Claude Desktop / agent harnesses.

use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

pub const MCP_VERSION: &str = "2024-11-05";
pub const WORPEN_VERSION: &str = env!("CARGO_PKG_VERSION");

/// JSON-RPC request envelope (minimal)
#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct RpcRequest {
    pub jsonrpc: Option<String>,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

fn rpc_ok(id: Option<Value>, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id.unwrap_or(Value::Null), "result": result})
}

fn rpc_err(id: Option<Value>, code: i32, message: String) -> Value {
    json!({"jsonrpc": "2.0", "id": id.unwrap_or(Value::Null), "error": {"code": code, "message": message}})
}

/// Build the tools/list array from registered routes + global functions.
pub async fn build_tools(state: &AppState) -> Vec<Value> {
    let mut tools = vec![];
    if let Ok(routes) = state.dynamic_route_service.list_routes().await {
        for r in routes {
            if !r.enabled {
                continue;
            }
            tools.push(json!({
                "name": r.name,
                "description": r.description,
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "payload": {"type": "object", "description": "Request body/payload"},
                        "query": {"type": "object", "description": "Query params"}
                    }
                },
                "_worpen": {"kind": "route", "route_id": r.id, "path": r.path, "method": format!("{:?}", r.method)}
            }));
        }
    }
    // Pipelines are first-class MCP tools: one multi-stage job per call.
    for def in state.pipeline_service.list_definitions().await {
        if !def.enabled {
            continue;
        }
        tools.push(json!({
            "name": format!("pipeline:{}", def.name),
            "description": format!("Run pipeline '{}'. {}", def.name, def.description),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "payload": {"type": "object", "description": "Initial input shared by all stages"}
                }
            },
            "_worpen": {"kind": "pipeline", "pipeline_id": def.id, "stages": def.stages.iter().map(|s| s.name.clone()).collect::<Vec<_>>()}
        }));
    }
    tools
}

/// GET /mcp — human-friendly info (browsers get this instead of a blank 405).
pub async fn mcp_info() -> axum::Json<Value> {
    axum::Json(json!({
        "name": "worpen",
        "version": WORPEN_VERSION,
        "protocol": "Model Context Protocol (JSON-RPC 2.0)",
        "protocolVersion": MCP_VERSION,
        "how_to_use": {
            "http": "POST JSON-RPC to /mcp with method initialize | tools/list | tools/call",
            "rest": "GET /mcp/tools lists all tools as plain JSON",
            "stdio": "worpen mcp serve-stdio  (for Claude Desktop / agents)",
            "example_call": {"jsonrpc": "2.0", "id": 1, "method": "tools/call",
                             "params": {"name": "<route-name>", "arguments": {"payload": {}}}}
        }
    }))
}

/// POST /mcp — JSON-RPC dispatcher
pub async fn mcp_handler(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<RpcRequest>,
) -> Json<Value> {
    let id = req.id.clone();
    match req.method.as_str() {
        "initialize" => Json(rpc_ok(id, json!({
            "protocolVersion": MCP_VERSION,
            "serverInfo": {"name": "worpen", "version": WORPEN_VERSION},
            "capabilities": {"tools": {}}
        }))),
        "notifications/initialized" => Json(rpc_ok(id, json!({}))),
        "ping" => Json(rpc_ok(id, json!({}))),
        "tools/list" => {
            let tools = build_tools(&state).await;
            Json(rpc_ok(id, json!({"tools": tools})))
        }
        "tools/call" => {
            let params = req.params.unwrap_or(Value::Null);
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match call_tool(&state, &headers, &name, args).await {
                Ok(v) => Json(rpc_ok(id, json!({"content": [{"type": "text", "text": v.to_string()}], "isError": false}))),
                Err(e) => Json(rpc_ok(id, json!({"content": [{"type": "text", "text": e}], "isError": true}))),
            }
        }
        _ => Json(rpc_err(id, -32601, format!("method not found: {}", req.method))),
    }
}

/// GET /mcp/tools — REST convenience
pub async fn mcp_tools_rest(State(state): State<AppState>) -> Json<Value> {
    let tools = build_tools(&state).await;
    Json(json!({"tools": tools, "count": tools.len()}))
}

async fn call_tool(state: &AppState, headers: &axum::http::HeaderMap, tool_name: &str, args: Value) -> Result<Value, String> {
    // Pipeline tools are namespaced as `pipeline:<name>`.
    if let Some(pipe_ref) = tool_name.strip_prefix("pipeline:") {
        let def = match state.pipeline_service.get_definition(pipe_ref).await {
            Some(d) => d,
            None => state
                .pipeline_service
                .find_by_name(pipe_ref)
                .await
                .ok_or_else(|| format!("tool '{}' not found", tool_name))?,
        };
        if def.auth_required {
            crate::auth::require_token(headers).map_err(|(_, body)| body.0.get("message").and_then(|m| m.as_str()).unwrap_or("unauthorized").to_string())?;
        }
        let payload = args.get("payload").cloned().or(Some(args.clone()));
        let run = state.pipeline_service.run_pipeline(&def.id, payload).await?;
        return Ok(serde_json::to_value(run).unwrap_or(Value::Null));
    }
    // Resolve tool name -> route (by route name first, then id)
    let routes = state.dynamic_route_service.list_routes().await?;
    let route = routes.iter().find(|r| r.name == tool_name || r.id == tool_name)
        .ok_or_else(|| format!("tool '{}' not found", tool_name))?;
    // Auth-gated tools require a token when one is configured
    if route.auth_required {
        crate::auth::require_token(headers).map_err(|(_, body)| body.0.get("message").and_then(|m| m.as_str()).unwrap_or("unauthorized").to_string())?;
    }
    let payload = args.get("payload").cloned()
        .or_else(|| if args.is_object() { Some(args.clone()) } else { None });
    let query: std::collections::HashMap<String, String> = args.get("query")
        .and_then(|q| q.as_object())
        .map(|m| m.iter().map(|(k, v)| (k.clone(), v.to_string().trim_matches('"').to_string())).collect())
        .unwrap_or_default();
    state.dynamic_route_service
        .execute_route(&route.id, payload, std::collections::HashMap::new(), query)
        .await
}

/// STDIO loop: reads JSON-RPC lines from stdin, writes responses to stdout.
/// Loads persisted routes from DATA_DIR (headless, no DB needed for pure-logic routes).
pub async fn serve_stdio() -> Result<(), String> {
    use tokio::io::{AsyncBufReadExt, BufReader};
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
    let svc = std::sync::Arc::new(worpen_core::services::DynamicRouteService::with_data_dir(data_dir));
    let pipes = std::sync::Arc::new(worpen_core::services::PipelineService::with_executor(svc.clone()));
    // Minimal AppState for tool execution (other services unused in stdio mode)
    // We reuse DynamicRouteService directly instead of full AppState.
    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();
    use tokio::io::AsyncWriteExt;
    let mut stdout = tokio::io::stdout();
    while let Some(line) = lines.next_line().await.map_err(|e| e.to_string())? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let req: RpcRequest = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(e) => {
                let resp = rpc_err(None, -32700, format!("parse error: {}", e));
                stdout.write_all(format!("{}\n", resp).as_bytes()).await.map_err(|e| e.to_string())?;
                stdout.flush().await.map_err(|e| e.to_string())?;
                continue;
            }
        };
        let id = req.id.clone();
        let resp = match req.method.as_str() {
            "initialize" => rpc_ok(id, json!({
                "protocolVersion": MCP_VERSION,
                "serverInfo": {"name": "worpen", "version": WORPEN_VERSION},
                "capabilities": {"tools": {}}
            })),
            "ping" => rpc_ok(id, json!({})),
            "tools/list" => {
                let routes = svc.list_routes().await.unwrap_or_default();
                let mut tools: Vec<Value> = routes.iter().filter(|r| r.enabled).map(|r| json!({
                    "name": r.name, "description": r.description,
                    "inputSchema": {"type": "object"},
                    "_worpen": {"kind": "route", "route_id": r.id, "path": r.path}
                })).collect();
                for def in pipes.list_definitions().await {
                    if !def.enabled {
                        continue;
                    }
                    tools.push(json!({
                        "name": format!("pipeline:{}", def.name),
                        "description": format!("Run pipeline '{}'. {}", def.name, def.description),
                        "inputSchema": {"type": "object"},
                        "_worpen": {"kind": "pipeline", "pipeline_id": def.id}
                    }));
                }
                rpc_ok(id, json!({"tools": tools}))
            }
            "tools/call" => {
                let params = req.params.unwrap_or(Value::Null);
                let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                // Pipelines first (namespace `pipeline:<name>`).
                if let Some(pipe_ref) = name.strip_prefix("pipeline:") {
                    let def = match pipes.get_definition(pipe_ref).await {
                        Some(d) => Some(d),
                        None => pipes.find_by_name(pipe_ref).await,
                    };
                    match def {
                        Some(d) => {
                            let payload = args.get("payload").cloned().or(Some(args.clone()));
                            match pipes.run_pipeline(&d.id, payload).await {
                                Ok(run) => rpc_ok(id, json!({"content": [{"type": "text", "text": serde_json::to_string(&run).unwrap_or_default()}], "isError": false})),
                                Err(e) => rpc_ok(id, json!({"content": [{"type": "text", "text": e}], "isError": true})),
                            }
                        }
                        None => rpc_ok(id, json!({"content": [{"type": "text", "text": format!("tool '{}' not found", name)}], "isError": true})),
                    }
                } else {
                    let routes = svc.list_routes().await.unwrap_or_default();
                    if let Some(route) = routes.iter().find(|r| r.name == name || r.id == name) {
                        let payload = args.get("payload").cloned().or(Some(args.clone()));
                        match svc.execute_route(&route.id, payload, Default::default(), Default::default()).await {
                            Ok(v) => rpc_ok(id, json!({"content": [{"type": "text", "text": v.to_string()}], "isError": false})),
                            Err(e) => rpc_ok(id, json!({"content": [{"type": "text", "text": e}], "isError": true})),
                        }
                    } else {
                        rpc_ok(id, json!({"content": [{"type": "text", "text": format!("tool '{}' not found", name)}], "isError": true}))
                    }
                }
            }
            _ => rpc_err(id, -32601, format!("method not found: {}", req.method)),
        };
        stdout.write_all(format!("{}\n", resp).as_bytes()).await.map_err(|e| e.to_string())?;
        stdout.flush().await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

