//! MCP (Model Context Protocol) Server mode — Phase 7 Quality.
//!
//! Exposes every Worpen dynamic route, pipeline, and workflow as an MCP tool,
//! along with built-in workflow run & approval gate tools:
//!   POST /mcp  (JSON-RPC 2.0: initialize, tools/list, tools/call)
//!   GET  /mcp/tools (plain REST for debugging)
//!
//! STDIO mode (`worpen mcp serve-stdio`) speaks the same JSON-RPC over stdin/stdout
//! for Claude Desktop / agent harnesses.

use crate::state::AppState;
use axum::{extract::State, http::HeaderMap, Json};
use proto::models::workflow::ExecutionMode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

pub const MCP_VERSION: &str = "2024-11-05";
pub const WORPEN_VERSION: &str = env!("CARGO_PKG_VERSION");

/// JSON-RPC request envelope
#[derive(serde::Deserialize, Debug)]
pub struct RpcRequest {
    #[allow(dead_code)]
    pub jsonrpc: Option<String>,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

pub fn rpc_ok(id: Option<Value>, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id.unwrap_or(Value::Null), "result": result})
}

pub fn rpc_err(id: Option<Value>, code: i32, message: String) -> Value {
    json!({"jsonrpc": "2.0", "id": id.unwrap_or(Value::Null), "error": {"code": code, "message": message}})
}

/// Structured agent-friendly error format (TASK-7.3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpAgentError {
    pub code: String,
    pub message: String,
    pub hint: String,
    pub retryable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
}

impl McpAgentError {
    pub fn not_found(name: &str) -> Self {
        Self {
            code: "TOOL_NOT_FOUND".to_string(),
            message: format!("Tool '{}' does not exist or is disabled", name),
            hint: "Call 'tools/list' to inspect available tools".to_string(),
            retryable: false,
            fields: None,
        }
    }

    pub fn unauthorized(msg: &str) -> Self {
        Self {
            code: "UNAUTHORIZED".to_string(),
            message: msg.to_string(),
            hint: "Provide a valid token in the Authorization Bearer or x-api-token header"
                .to_string(),
            retryable: false,
            fields: None,
        }
    }

    pub fn policy_violation(reason: &str) -> Self {
        Self {
            code: "POLICY_VIOLATION".to_string(),
            message: format!("Sandbox policy blocked action: {}", reason),
            hint: "Check workflow SandboxPolicy for allowed_hosts, read_only_sql, or blocked private IPs"
                .to_string(),
            retryable: false,
            fields: None,
        }
    }

    pub fn approval_required(run_id: &str, summary: &str) -> Self {
        Self {
            code: "APPROVAL_REQUIRED".to_string(),
            message: format!(
                "Action paused at approval gate for run '{}': {}",
                run_id, summary
            ),
            hint: "An authorized human operator must approve this run via CLI ('worpen workflow approve') or authenticated management REST API. Agents cannot self-approve via MCP."
                .to_string(),
            retryable: true,
            fields: Some(json!({ "run_id": run_id })),
        }
    }

    pub fn execution_error(err: &str) -> Self {
        Self {
            code: "EXECUTION_ERROR".to_string(),
            message: err.to_string(),
            hint: "Verify input payload arguments match schema and parameters".to_string(),
            retryable: false,
            fields: None,
        }
    }
}

/// Extract agent caller role from headers
pub fn extract_caller_role(headers: Option<&HeaderMap>) -> Option<String> {
    headers.and_then(|h| {
        h.get("x-agent-role")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
    })
}

/// Build the tools/list array from registered workflows, routes, pipelines, and built-ins.
/// Applies identity-based filtering and annotations.
pub async fn build_tools(state: &AppState, headers: Option<&HeaderMap>) -> Vec<Value> {
    let mut tools = vec![];
    let is_authenticated = match headers {
        Some(h) => crate::auth::require_token(h).is_ok(),
        None => crate::auth::configured_token().is_none(),
    };
    let caller_role = extract_caller_role(headers);
    let is_read_only_agent = caller_role.as_deref() == Some("read_only_analyst");

    // 1. Built-in Management & Verification Tools (Phase 5, 6 & 7)
    tools.push(json!({
        "name": "get_run",
        "description": "Retrieve the execution details, status, step index, variables, or pending approval of a workflow run.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "run_id": {"type": "string", "description": "UUID of the workflow run"}
            },
            "required": ["run_id"]
        },
        "annotations": {
            "readOnly": true,
            "destructive": false,
            "idempotent": true
        },
        "_worpen": {"kind": "builtin", "tool": "get_run"}
    }));

    tools.push(json!({
        "name": "list_pending_approvals",
        "description": "List all workflow runs paused at an approval gate waiting for human operator approval.",
        "inputSchema": {
            "type": "object",
            "properties": {}
        },
        "annotations": {
            "readOnly": true,
            "destructive": false,
            "idempotent": true
        },
        "_worpen": {"kind": "builtin", "tool": "list_pending_approvals"}
    }));

    tools.push(json!({
        "name": "verify_audit",
        "description": "Verify the cryptographic HMAC-SHA256 hash chain integrity of the on-disk audit log.",
        "inputSchema": {
            "type": "object",
            "properties": {}
        },
        "annotations": {
            "readOnly": true,
            "destructive": false,
            "idempotent": true
        },
        "_worpen": {"kind": "builtin", "tool": "verify_audit"}
    }));

    // 2. Registered Workflows (Spec v1)
    for wf in state.workflow_service.list_workflows() {
        if is_read_only_agent {
            continue;
        }
        let schema = wf
            .input_schema
            .clone()
            .unwrap_or_else(|| json!({"type": "object"}));
        tools.push(json!({
            "name": format!("workflow:{}", wf.name),
            "description": wf.description,
            "inputSchema": schema,
            "annotations": {
                "readOnly": false,
                "destructive": false,
                "idempotent": false
            },
            "_worpen": {"kind": "workflow", "workflow_name": wf.name, "version": wf.version}
        }));
    }

    // 3. Registered Pipelines
    for def in state.pipeline_service.list_definitions().await {
        if !def.enabled {
            continue;
        }
        if def.auth_required && !is_authenticated {
            continue;
        }
        if is_read_only_agent {
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
            "annotations": {
                "readOnly": false,
                "destructive": false,
                "idempotent": false
            },
            "_worpen": {"kind": "pipeline", "pipeline_id": def.id, "stages": def.stages.iter().map(|s| s.name.clone()).collect::<Vec<_>>()}
        }));
    }

    // 4. Dynamic routes
    if let Ok(routes) = state.dynamic_route_service.list_routes().await {
        for r in routes {
            if !r.enabled {
                continue;
            }
            if r.auth_required && !is_authenticated {
                continue;
            }

            let is_get = format!("{:?}", r.method).contains("GET");
            let is_delete = format!("{:?}", r.method).contains("DELETE");
            let is_put = format!("{:?}", r.method).contains("PUT");

            let read_only = is_get;
            let destructive = is_delete;
            let idempotent = is_get || is_delete || is_put;

            if is_read_only_agent && !read_only {
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
                "annotations": {
                    "readOnly": read_only,
                    "destructive": destructive,
                    "idempotent": idempotent
                },
                "_worpen": {"kind": "route", "route_id": r.id, "path": r.path, "method": format!("{:?}", r.method)}
            }));
        }
    }

    tools
}

/// GET /mcp — human-friendly info
pub async fn mcp_info() -> axum::Json<Value> {
    axum::Json(json!({
        "name": "worpen",
        "version": WORPEN_VERSION,
        "protocol": "Model Context Protocol (JSON-RPC 2.0)",
        "protocolVersion": MCP_VERSION,
        "capabilities": {
            "tools": true,
            "annotations": true,
            "structuredErrors": true
        },
        "how_to_use": {
            "http": "POST JSON-RPC to /mcp with method initialize | tools/list | tools/call",
            "rest": "GET /mcp/tools lists all tools as plain JSON",
            "stdio": "worpen mcp serve-stdio  (for Claude Desktop / Cursor / agents)",
            "example_call": {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/call",
                "params": {"name": "workflow:refund", "arguments": {"order_id": "ord_123", "amount": 49.99}}
            }
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
        "initialize" => Json(rpc_ok(
            id,
            json!({
                "protocolVersion": MCP_VERSION,
                "serverInfo": {"name": "worpen", "version": WORPEN_VERSION},
                "capabilities": {
                    "tools": {
                        "listChanged": false
                    }
                }
            }),
        )),
        "notifications/initialized" => Json(rpc_ok(id, json!({}))),
        "ping" => Json(rpc_ok(id, json!({}))),
        "tools/list" => {
            let tools = build_tools(&state, Some(&headers)).await;
            Json(rpc_ok(id, json!({ "tools": tools })))
        }
        "tools/call" => {
            let params = req.params.unwrap_or(Value::Null);
            let name = params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match call_tool(&state, &headers, &name, args).await {
                Ok(v) => {
                    let formatted_text =
                        serde_json::to_string_pretty(&v).unwrap_or_else(|_| v.to_string());
                    Json(rpc_ok(
                        id,
                        json!({
                            "content": [{"type": "text", "text": formatted_text}],
                            "isError": false,
                            "result": v
                        }),
                    ))
                }
                Err(err) => {
                    let err_json = serde_json::to_string_pretty(&err).unwrap_or_default();
                    Json(rpc_ok(
                        id,
                        json!({
                            "content": [{"type": "text", "text": err_json}],
                            "isError": true,
                            "structuredError": err
                        }),
                    ))
                }
            }
        }
        _ => Json(rpc_err(
            id,
            -32601,
            format!("method not found: {}", req.method),
        )),
    }
}

/// GET /mcp/tools — REST convenience
pub async fn mcp_tools_rest(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Json<Value> {
    let tools = build_tools(&state, Some(&headers)).await;
    Json(json!({ "tools": tools, "count": tools.len() }))
}

pub async fn call_tool(
    state: &AppState,
    headers: &axum::http::HeaderMap,
    tool_name: &str,
    args: Value,
) -> Result<Value, McpAgentError> {
    // 1. Built-in tools
    match tool_name {
        "get_run" | "worpen:get_run" => {
            let run_id = args.get("run_id").and_then(|v| v.as_str()).ok_or_else(|| {
                McpAgentError::execution_error("Missing required parameter 'run_id'")
            })?;
            let run_opt = state
                .workflow_service
                .get_run(run_id)
                .await
                .map_err(|e| McpAgentError::execution_error(&e))?;
            match run_opt {
                Some(run) => Ok(json!(run)),
                None => Err(McpAgentError::not_found(&format!("Run '{}'", run_id))),
            }
        }
        "list_pending_approvals" | "worpen:list_pending_approvals" => {
            let pending = state
                .workflow_service
                .list_pending_approvals()
                .await
                .map_err(|e| McpAgentError::execution_error(&e))?;
            Ok(json!(pending))
        }
        "approve_run" | "worpen:approve_run" | "reject_run" | "worpen:reject_run" => {
            Err(McpAgentError {
                code: "APPROVAL_CHANNEL_SEPARATION".to_string(),
                message: format!(
                    "Tool '{}' is strictly forbidden via MCP. Approvals and rejections must occur through the dedicated operator channel (CLI 'worpen workflow approve' or authenticated REST API) with distinct operator credentials.",
                    tool_name
                ),
                hint: "Contact an authorized human operator to approve or reject this run via CLI or the authenticated management console.".to_string(),
                retryable: false,
                fields: Some(json!({ "forbidden_tool": tool_name })),
            })
        }
        "verify_audit" | "worpen:verify_audit" => {
            let chain = state.dynamic_route_service.audit_chain();
            match chain.verify() {
                Ok(report) => Ok(json!(report)),
                Err(err) => Err(McpAgentError::policy_violation(&err.to_string())),
            }
        }
        _ => {
            // Check workflow spec v1
            let wf_name = tool_name.strip_prefix("workflow:").unwrap_or(tool_name);
            if let Some(_spec) = state.workflow_service.get_workflow(wf_name) {
                let caller = extract_caller_role(Some(headers))
                    .map(|r| format!("agent:{}", r))
                    .unwrap_or_else(|| "mcp_agent".to_string());
                let payload = args.get("payload").cloned().or(Some(args.clone()));
                let run_res = state
                    .workflow_service
                    .execute_with_meta(wf_name, payload, ExecutionMode::Live, Some(caller), None)
                    .await;
                match run_res {
                    Ok(val) => {
                        // Check if it's a WorkflowRun paused for approval
                        if let Some(status) = val.get("status").and_then(|s| s.as_str()) {
                            if status == "waiting_for_approval" || status == "WaitingForApproval" {
                                let run_id =
                                    val.get("run_id").and_then(|r| r.as_str()).unwrap_or("");
                                let summary = val
                                    .get("pending_approval")
                                    .and_then(|p| p.get("summary"))
                                    .and_then(|s| s.as_str())
                                    .unwrap_or("Human approval required");
                                return Err(McpAgentError::approval_required(run_id, summary));
                            }
                        }
                        return Ok(val);
                    }
                    Err(e) => {
                        if e.contains("blocked") || e.contains("policy") || e.contains("SSRF") {
                            return Err(McpAgentError::policy_violation(&e));
                        }
                        return Err(McpAgentError::execution_error(&e));
                    }
                }
            }

            // Check pipeline
            if let Some(pipe_ref) = tool_name.strip_prefix("pipeline:") {
                let def = match state.pipeline_service.get_definition(pipe_ref).await {
                    Some(d) => d,
                    None => state
                        .pipeline_service
                        .find_by_name(pipe_ref)
                        .await
                        .ok_or_else(|| McpAgentError::not_found(tool_name))?,
                };
                if def.auth_required {
                    crate::auth::require_token(headers).map_err(|(_, body)| {
                        McpAgentError::unauthorized(
                            body.0
                                .get("message")
                                .and_then(|m| m.as_str())
                                .unwrap_or("unauthorized"),
                        )
                    })?;
                }
                let payload = args.get("payload").cloned().or(Some(args.clone()));
                let run = state
                    .pipeline_service
                    .run_pipeline(&def.id, payload)
                    .await
                    .map_err(|e| McpAgentError::execution_error(&e))?;
                return Ok(serde_json::to_value(run).unwrap_or(Value::Null));
            }

            // Check dynamic routes
            let routes = state
                .dynamic_route_service
                .list_routes()
                .await
                .map_err(|e| McpAgentError::execution_error(&e))?;
            let route = routes
                .iter()
                .find(|r| r.name == tool_name || r.id == tool_name)
                .ok_or_else(|| McpAgentError::not_found(tool_name))?;

            if route.auth_required {
                crate::auth::require_token(headers).map_err(|(_, body)| {
                    McpAgentError::unauthorized(
                        body.0
                            .get("message")
                            .and_then(|m| m.as_str())
                            .unwrap_or("unauthorized"),
                    )
                })?;
            }

            let payload = args.get("payload").cloned().or_else(|| {
                if args.is_object() {
                    Some(args.clone())
                } else {
                    None
                }
            });
            let query: std::collections::HashMap<String, String> = args
                .get("query")
                .and_then(|q| q.as_object())
                .map(|m| {
                    m.iter()
                        .map(|(k, v)| (k.clone(), v.to_string().trim_matches('"').to_string()))
                        .collect()
                })
                .unwrap_or_default();

            state
                .dynamic_route_service
                .execute_route(&route.id, payload, std::collections::HashMap::new(), query)
                .await
                .map_err(|e| {
                    if e.contains("blocked") || e.contains("policy") || e.contains("SSRF") {
                        McpAgentError::policy_violation(&e)
                    } else {
                        McpAgentError::execution_error(&e)
                    }
                })
        }
    }
}

/// STDIO loop: reads JSON-RPC lines from stdin, writes responses to stdout.
pub async fn serve_stdio() -> Result<(), String> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string());
    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:worpen.db?mode=rwc".to_string());
    let mut svc_raw = worpen_core::services::DynamicRouteService::with_data_dir(data_dir.clone());
    if let Ok(pool) = infra::initialize_db(&db_url).await {
        svc_raw.set_db_pool(pool);
    }
    let svc = Arc::new(svc_raw);
    let pipes = Arc::new(worpen_core::services::PipelineService::with_executor(
        svc.clone(),
    ));
    let wf_svc = Arc::new(worpen_core::workflow::WorkflowService::new(
        data_dir,
        svc.clone(),
    ));

    let stdin = tokio::io::stdin();
    let mut lines = BufReader::new(stdin).lines();
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
                stdout
                    .write_all(format!("{}\n", resp).as_bytes())
                    .await
                    .map_err(|e| e.to_string())?;
                stdout.flush().await.map_err(|e| e.to_string())?;
                continue;
            }
        };
        let id = req.id.clone();
        let resp = match req.method.as_str() {
            "initialize" => rpc_ok(
                id,
                json!({
                    "protocolVersion": MCP_VERSION,
                    "serverInfo": {"name": "worpen", "version": WORPEN_VERSION},
                    "capabilities": {"tools": {}}
                }),
            ),
            "ping" => rpc_ok(id, json!({})),
            "tools/list" => {
                let mut tools = vec![
                    json!({
                        "name": "get_run",
                        "description": "Retrieve workflow run details.",
                        "inputSchema": {"type": "object", "properties": {"run_id": {"type": "string"}}, "required": ["run_id"]},
                        "annotations": {"readOnly": true, "destructive": false, "idempotent": true}
                    }),
                    json!({
                        "name": "list_pending_approvals",
                        "description": "List workflow runs paused for approval.",
                        "inputSchema": {"type": "object"},
                        "annotations": {"readOnly": true, "destructive": false, "idempotent": true}
                    }),
                    json!({
                        "name": "verify_audit",
                        "description": "Verify audit log integrity.",
                        "inputSchema": {"type": "object"},
                        "annotations": {"readOnly": true, "destructive": false, "idempotent": true}
                    }),
                ];
                for wf in wf_svc.list_workflows() {
                    tools.push(json!({
                        "name": format!("workflow:{}", wf.name),
                        "description": wf.description,
                        "inputSchema": wf.input_schema.unwrap_or_else(|| json!({"type": "object"})),
                        "annotations": {"readOnly": false, "destructive": false, "idempotent": false}
                    }));
                }
                for def in pipes.list_definitions().await {
                    if def.enabled {
                        tools.push(json!({
                            "name": format!("pipeline:{}", def.name),
                            "description": def.description,
                            "inputSchema": {"type": "object"},
                            "annotations": {"readOnly": false, "destructive": false, "idempotent": false}
                        }));
                    }
                }
                let routes = svc.list_routes().await.unwrap_or_default();
                for r in routes {
                    if r.enabled {
                        tools.push(json!({
                            "name": r.name,
                            "description": r.description,
                            "inputSchema": {"type": "object"},
                            "annotations": {"readOnly": false, "destructive": false, "idempotent": false}
                        }));
                    }
                }
                rpc_ok(id, json!({ "tools": tools }))
            }
            "tools/call" => {
                let params = req.params.unwrap_or(Value::Null);
                let name = params
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                match name.as_str() {
                    "get_run" => {
                        let run_id = args.get("run_id").and_then(|v| v.as_str()).unwrap_or("");
                        match wf_svc.get_run(run_id).await {
                            Ok(Some(run)) => rpc_ok(
                                id,
                                json!({ "content": [{"type": "text", "text": serde_json::to_string_pretty(&run).unwrap_or_default()}], "isError": false }),
                            ),
                            Ok(None) => rpc_ok(
                                id,
                                json!({ "content": [{"type": "text", "text": format!("Run '{}' not found", run_id)}], "isError": true }),
                            ),
                            Err(e) => rpc_ok(
                                id,
                                json!({ "content": [{"type": "text", "text": e}], "isError": true }),
                            ),
                        }
                    }
                    "list_pending_approvals" => match wf_svc.list_pending_approvals().await {
                        Ok(pending) => rpc_ok(
                            id,
                            json!({ "content": [{"type": "text", "text": serde_json::to_string_pretty(&pending).unwrap_or_default()}], "isError": false }),
                        ),
                        Err(e) => rpc_ok(
                            id,
                            json!({ "content": [{"type": "text", "text": e}], "isError": true }),
                        ),
                    },
                    "approve_run" | "reject_run" => rpc_ok(
                        id,
                        json!({
                            "content": [{"type": "text", "text": "Approvals and rejections are strictly forbidden via MCP stdio. Use operator CLI: 'worpen workflow approve'"}],
                            "isError": true
                        }),
                    ),
                    "verify_audit" => match svc.audit_chain().verify() {
                        Ok(rep) => rpc_ok(
                            id,
                            json!({ "content": [{"type": "text", "text": serde_json::to_string_pretty(&rep).unwrap_or_default()}], "isError": false }),
                        ),
                        Err(e) => rpc_ok(
                            id,
                            json!({ "content": [{"type": "text", "text": e.to_string()}], "isError": true }),
                        ),
                    },
                    _ => {
                        // Check workflow
                        let wf_name = name.strip_prefix("workflow:").unwrap_or(&name);
                        if wf_svc.get_workflow(wf_name).is_some() {
                            let payload = args.get("payload").cloned().or(Some(args.clone()));
                            match wf_svc
                                .execute_with_meta(
                                    wf_name,
                                    payload,
                                    ExecutionMode::Live,
                                    Some("agent:stdio".to_string()),
                                    None,
                                )
                                .await
                            {
                                Ok(val) => rpc_ok(
                                    id,
                                    json!({ "content": [{"type": "text", "text": serde_json::to_string_pretty(&val).unwrap_or_default()}], "isError": false }),
                                ),
                                Err(e) => rpc_ok(
                                    id,
                                    json!({ "content": [{"type": "text", "text": e}], "isError": true }),
                                ),
                            }
                        } else if let Some(pipe_ref) = name.strip_prefix("pipeline:") {
                            let def = match pipes.get_definition(pipe_ref).await {
                                Some(d) => Some(d),
                                None => pipes.find_by_name(pipe_ref).await,
                            };
                            match def {
                                Some(d) => {
                                    let payload =
                                        args.get("payload").cloned().or(Some(args.clone()));
                                    match pipes.run_pipeline(&d.id, payload).await {
                                        Ok(run) => rpc_ok(
                                            id,
                                            json!({"content": [{"type": "text", "text": serde_json::to_string(&run).unwrap_or_default()}], "isError": false}),
                                        ),
                                        Err(e) => rpc_ok(
                                            id,
                                            json!({"content": [{"type": "text", "text": e}], "isError": true}),
                                        ),
                                    }
                                }
                                None => rpc_ok(
                                    id,
                                    json!({"content": [{"type": "text", "text": format!("tool '{}' not found", name)}], "isError": true}),
                                ),
                            }
                        } else {
                            let routes = svc.list_routes().await.unwrap_or_default();
                            if let Some(route) =
                                routes.iter().find(|r| r.name == name || r.id == name)
                            {
                                let payload = args.get("payload").cloned().or(Some(args.clone()));
                                match svc
                                    .execute_route(
                                        &route.id,
                                        payload,
                                        Default::default(),
                                        Default::default(),
                                    )
                                    .await
                                {
                                    Ok(v) => rpc_ok(
                                        id,
                                        json!({"content": [{"type": "text", "text": v.to_string()}], "isError": false}),
                                    ),
                                    Err(e) => rpc_ok(
                                        id,
                                        json!({"content": [{"type": "text", "text": e}], "isError": true}),
                                    ),
                                }
                            } else {
                                rpc_ok(
                                    id,
                                    json!({"content": [{"type": "text", "text": format!("tool '{}' not found", name)}], "isError": true}),
                                )
                            }
                        }
                    }
                }
            }
            _ => rpc_err(id, -32601, format!("method not found: {}", req.method)),
        };
        stdout
            .write_all(format!("{}\n", resp).as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        stdout.flush().await.map_err(|e| e.to_string())?;
    }
    Ok(())
}
