use api::mcp::{mcp_handler, RpcRequest};
use api::state::AppState;
use axum::{extract::State, http::HeaderMap, Json};
use serde_json::json;
use std::fs;

async fn setup_test_app() -> (AppState, String) {
    let temp_dir = std::env::temp_dir().join(format!("worpen_mcp_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();
    let db_path = temp_dir.join("test.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = infra::initialize_db(&db_url).await.unwrap();
    let state = AppState::new_with_pool_and_data_dir(pool, data_dir.clone()).await;

    // Ensure durable tables are created
    let engine = state
        .workflow_service
        .ensure_durable_engine()
        .await
        .unwrap();
    engine.store().init().await.unwrap();

    (state, data_dir)
}

#[tokio::test]
async fn test_mcp_initialize_and_ping() {
    let (state, _data_dir) = setup_test_app().await;

    let init_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: None,
    };
    let Json(resp) = mcp_handler(State(state.clone()), HeaderMap::new(), Json(init_req)).await;
    assert_eq!(resp["result"]["protocolVersion"], "2024-11-05");
    assert_eq!(resp["result"]["serverInfo"]["name"], "worpen");

    let ping_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(2)),
        method: "ping".to_string(),
        params: None,
    };
    let Json(ping_resp) = mcp_handler(State(state), HeaderMap::new(), Json(ping_req)).await;
    assert_eq!(ping_resp["id"], 2);
    assert!(ping_resp["result"].is_object());
}

#[tokio::test]
async fn test_mcp_tools_list_annotations_and_builtins() {
    let (state, _data_dir) = setup_test_app().await;

    let list_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(10)),
        method: "tools/list".to_string(),
        params: None,
    };
    let Json(resp) = mcp_handler(State(state), HeaderMap::new(), Json(list_req)).await;
    let tools = resp["result"]["tools"].as_array().expect("tools array");

    // Assert built-in tools are present
    let tool_names: Vec<String> = tools
        .iter()
        .filter_map(|t| t["name"].as_str().map(|s| s.to_string()))
        .collect();
    assert!(tool_names.contains(&"get_run".to_string()));
    assert!(tool_names.contains(&"list_pending_approvals".to_string()));
    assert!(tool_names.contains(&"approve_run".to_string()));
    assert!(tool_names.contains(&"reject_run".to_string()));
    assert!(tool_names.contains(&"verify_audit".to_string()));

    // Verify all tools have proper annotations
    for t in tools {
        let name = t["name"].as_str().unwrap();
        let annotations = &t["annotations"];
        assert!(
            annotations.is_object(),
            "Tool '{}' must have annotations object",
            name
        );
        assert!(annotations.get("readOnly").is_some());
        assert!(annotations.get("destructive").is_some());
        assert!(annotations.get("idempotent").is_some());

        if name == "get_run" || name == "list_pending_approvals" || name == "verify_audit" {
            assert_eq!(annotations["readOnly"], true);
            assert_eq!(annotations["idempotent"], true);
        } else if name == "approve_run" || name == "reject_run" {
            assert_eq!(annotations["readOnly"], false);
            assert_eq!(annotations["idempotent"], false);
        }
    }
}

#[tokio::test]
async fn test_mcp_identity_filtering() {
    let (state, _data_dir) = setup_test_app().await;

    // Caller with X-Agent-Role: read_only_analyst
    let mut headers = HeaderMap::new();
    headers.insert("x-agent-role", "read_only_analyst".parse().unwrap());

    let list_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(20)),
        method: "tools/list".to_string(),
        params: None,
    };
    let Json(resp) = mcp_handler(State(state), headers, Json(list_req)).await;
    let tools = resp["result"]["tools"].as_array().expect("tools array");
    let tool_names: Vec<String> = tools
        .iter()
        .filter_map(|t| t["name"].as_str().map(|s| s.to_string()))
        .collect();

    // Read-only tools must be present
    assert!(tool_names.contains(&"get_run".to_string()));
    assert!(tool_names.contains(&"list_pending_approvals".to_string()));
    assert!(tool_names.contains(&"verify_audit".to_string()));

    // Mutating tools must be filtered out for read_only_analyst!
    assert!(!tool_names.contains(&"approve_run".to_string()));
    assert!(!tool_names.contains(&"reject_run".to_string()));

    // All exposed tools must be readOnly: true
    for t in tools {
        assert_eq!(t["annotations"]["readOnly"], true);
    }
}

#[tokio::test]
async fn test_mcp_structured_errors() {
    let (state, _data_dir) = setup_test_app().await;

    let call_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(30)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "non_existing_tool_xyz",
            "arguments": {}
        })),
    };
    let Json(resp) = mcp_handler(State(state), HeaderMap::new(), Json(call_req)).await;
    let result = &resp["result"];
    assert_eq!(result["isError"], true);

    let structured = &result["structuredError"];
    assert_eq!(structured["code"], "TOOL_NOT_FOUND");
    assert!(structured["message"]
        .as_str()
        .unwrap()
        .contains("does not exist"));
    assert!(structured["hint"].as_str().unwrap().contains("tools/list"));
    assert_eq!(structured["retryable"], false);
}

#[tokio::test]
async fn test_mcp_e2e_workflow_and_approval_cycle() {
    let (state, _data_dir) = setup_test_app().await;

    // Register a workflow with an approval step via YAML
    let spec_yaml = r#"
name: mcp_refund
version: "1.0.0"
description: Process a refund requiring manager approval
input_schema:
  type: object
  properties:
    amount:
      type: number
steps:
  - id: gate_check
    op: approval
    summary: "Approve refund of ${{amount}}"
    timeout_seconds: 3600
  - id: calc_fee
    op: transform
    expr: "2.5"
    output_var: fee
return:
  status: 200
  value:
    status: refunded
    fee: "{{fee}}"
    approved_by: "{{__approved_by}}"
"#;
    state
        .workflow_service
        .register_workflow(spec_yaml)
        .await
        .unwrap();

    // 1. Agent calls workflow tool -> pauses at Approval Gate
    let call_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(40)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "workflow:mcp_refund",
            "arguments": {
                "amount": 75.0
            }
        })),
    };
    let Json(resp1) = mcp_handler(State(state.clone()), HeaderMap::new(), Json(call_req)).await;
    let res1 = &resp1["result"];
    assert_eq!(res1["isError"], true);

    let err1 = &res1["structuredError"];
    assert_eq!(err1["code"], "APPROVAL_REQUIRED");
    assert_eq!(err1["retryable"], true);
    assert!(err1["message"]
        .as_str()
        .unwrap()
        .contains("Approve refund of $75"));
    let run_id = err1["fields"]["run_id"].as_str().unwrap().to_string();
    assert!(!run_id.is_empty());

    // 2. Supervisor checks pending approvals via MCP tool
    let list_approvals_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(41)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "list_pending_approvals",
            "arguments": {}
        })),
    };
    let Json(resp2) = mcp_handler(
        State(state.clone()),
        HeaderMap::new(),
        Json(list_approvals_req),
    )
    .await;
    let pending_list = &resp2["result"]["result"];
    assert!(pending_list.is_array());
    let pending_runs = pending_list.as_array().unwrap();
    assert_eq!(pending_runs.len(), 1);
    assert_eq!(pending_runs[0]["run_id"], run_id);

    // 3. Supervisor approves run via MCP tool
    let approve_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(42)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "approve_run",
            "arguments": {
                "run_id": run_id,
                "approver": "supervisor_alice",
                "reason": "Legitimate claim confirmed"
            }
        })),
    };
    let Json(resp3) = mcp_handler(State(state.clone()), HeaderMap::new(), Json(approve_req)).await;
    let res3 = &resp3["result"];
    assert_eq!(res3["isError"], false);
    let run_res = &res3["result"];
    assert_eq!(run_res["status"], "completed");
    assert_eq!(run_res["result"]["status"], "refunded");
    assert_eq!(run_res["result"]["approved_by"], "supervisor_alice");

    // 4. Verify cryptographic audit chain via MCP tool
    let audit_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(43)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "verify_audit",
            "arguments": {}
        })),
    };
    let Json(resp4) = mcp_handler(State(state), HeaderMap::new(), Json(audit_req)).await;
    let audit_res = &resp4["result"]["result"];
    assert_eq!(audit_res["valid"], true);
    assert!(audit_res["total_entries"].as_u64().unwrap() >= 2);
}
