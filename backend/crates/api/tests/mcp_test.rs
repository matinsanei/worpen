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

    // Assert built-in read-only verification tools are present
    let tool_names: Vec<String> = tools
        .iter()
        .filter_map(|t| t["name"].as_str().map(|s| s.to_string()))
        .collect();
    assert!(tool_names.contains(&"get_run".to_string()));
    assert!(tool_names.contains(&"list_pending_approvals".to_string()));
    assert!(tool_names.contains(&"verify_audit".to_string()));

    // Strict channel separation: approve_run and reject_run must NEVER be exposed as agent MCP tools
    assert!(!tool_names.contains(&"approve_run".to_string()));
    assert!(!tool_names.contains(&"reject_run".to_string()));

    // Verify all exposed tools have proper annotations
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

    // 3. Agent attempts to self-approve via MCP tool — MUST BE BLOCKED by channel separation!
    let agent_approve_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(42)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "approve_run",
            "arguments": {
                "run_id": run_id,
                "approver": "mcp_agent",
                "reason": "Agent attempting self-approval"
            }
        })),
    };
    let Json(resp3) = mcp_handler(
        State(state.clone()),
        HeaderMap::new(),
        Json(agent_approve_req),
    )
    .await;
    let res3 = &resp3["result"];
    assert_eq!(res3["isError"], true);
    assert_eq!(
        res3["structuredError"]["code"],
        "APPROVAL_CHANNEL_SEPARATION"
    );

    // 4. Operator channel: Anti-self-approval check
    // If the initiator attempts to approve through the operator channel, it must be rejected!
    let self_approve_res = state
        .workflow_service
        .approve_run(&run_id, "mcp_agent", "Self-approval attempt")
        .await;
    assert!(self_approve_res.is_err());
    assert!(self_approve_res
        .unwrap_err()
        .contains("Self-approval forbidden"));

    // 5. Authorized distinct human operator approves via operator channel
    let approved_run = state
        .workflow_service
        .approve_run(&run_id, "supervisor_alice", "Legitimate claim confirmed")
        .await
        .expect("Operator approval must succeed");
    assert_eq!(approved_run.status, proto::models::RunStatus::Completed);
    let run_res = approved_run.result.expect("completed result");
    assert_eq!(run_res["status"], "refunded");
    assert_eq!(run_res["approved_by"], "supervisor_alice");

    // 6. Verify cryptographic audit chain via MCP tool
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

#[tokio::test]
async fn test_mcp_secrets_redaction_in_get_run_and_tools_call() {
    let (state, _data_dir) = setup_test_app().await;

    // Register a secret in the secret store
    state
        .dynamic_route_service
        .secret_store()
        .set_secret("TEST_API_KEY", "super_secret_token_value_999");

    // Register a workflow that references this secret
    let wf_yaml = r#"
name: secret_leak_test
version: "1.0"
steps:
  - id: step1
    op: set
    var: leaked_var
    value: "{{secret:TEST_API_KEY}}"
return:
  status: 200
  value:
    token: "{{secret:TEST_API_KEY}}"
"#;
    state
        .workflow_service
        .register_workflow(wf_yaml)
        .await
        .unwrap();

    // Call workflow via MCP tools/call
    let call_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(101)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "workflow:secret_leak_test",
            "arguments": {}
        })),
    };

    let Json(resp) = mcp_handler(State(state.clone()), HeaderMap::new(), Json(call_req)).await;
    let resp_str = serde_json::to_string(&resp).unwrap();
    assert!(
        !resp_str.contains("super_secret_token_value_999"),
        "Raw secret must NEVER appear in MCP response: {}",
        resp_str
    );

    // Call get_run on the run_id
    let runs = state.workflow_service.list_runs(1).await.unwrap();
    let run_id = &runs[0].run_id;

    let get_run_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(102)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "get_run",
            "arguments": { "run_id": run_id }
        })),
    };

    let Json(get_run_resp) = mcp_handler(State(state), HeaderMap::new(), Json(get_run_req)).await;
    let get_run_str = serde_json::to_string(&get_run_resp).unwrap();
    assert!(
        !get_run_str.contains("super_secret_token_value_999"),
        "Raw secret must NEVER appear in get_run response: {}",
        get_run_str
    );
}

#[tokio::test]
async fn test_workflow_runs_endpoints_require_token() {
    let (state, _data_dir) = setup_test_app().await;

    // Set WORPEN_API_TOKEN
    let test_token = "secure_ops_token_42";
    std::env::set_var("WORPEN_API_TOKEN", test_token);

    let empty_headers = HeaderMap::new();

    // 1. list_runs without token -> 401 Unauthorized
    let res =
        api::handlers::workflows::list_runs(State(state.clone()), empty_headers.clone()).await;
    assert!(res.is_err(), "list_runs without token must fail");
    assert_eq!(res.unwrap_err().0, axum::http::StatusCode::UNAUTHORIZED);

    // 2. list_pending_approvals without token -> 401 Unauthorized
    let res = api::handlers::workflows::list_pending_approvals(
        State(state.clone()),
        empty_headers.clone(),
    )
    .await;
    assert!(
        res.is_err(),
        "list_pending_approvals without token must fail"
    );
    assert_eq!(res.unwrap_err().0, axum::http::StatusCode::UNAUTHORIZED);

    // 3. get_run without token -> 401 Unauthorized
    let res = api::handlers::workflows::get_run(
        State(state.clone()),
        empty_headers.clone(),
        axum::extract::Path("dummy-id".to_string()),
    )
    .await;
    assert!(res.is_err(), "get_run without token must fail");
    assert_eq!(res.unwrap_err().0, axum::http::StatusCode::UNAUTHORIZED);

    // 4. approve_run without token -> 401 Unauthorized
    let res = api::handlers::workflows::approve_run(
        State(state.clone()),
        empty_headers.clone(),
        axum::extract::Path("dummy-id".to_string()),
        Json(api::handlers::workflows::ApprovalActionPayload {
            approver: "ops_alice".to_string(),
            reason: None,
        }),
    )
    .await;
    assert!(res.is_err(), "approve_run without token must fail");
    assert_eq!(res.unwrap_err().0, axum::http::StatusCode::UNAUTHORIZED);

    // 5. cancel_run without token -> 401 Unauthorized
    let res = api::handlers::workflows::cancel_run(
        State(state.clone()),
        empty_headers.clone(),
        axum::extract::Path("dummy-id".to_string()),
    )
    .await;
    assert!(res.is_err(), "cancel_run without token must fail");
    assert_eq!(res.unwrap_err().0, axum::http::StatusCode::UNAUTHORIZED);

    // 6. With valid Authorization header -> Succeeds (authenticated)
    let mut authed_headers = HeaderMap::new();
    authed_headers.insert(
        "authorization",
        axum::http::HeaderValue::from_str(&format!("Bearer {}", test_token)).unwrap(),
    );

    let res =
        api::handlers::workflows::list_runs(State(state.clone()), authed_headers.clone()).await;
    assert!(
        res.is_ok(),
        "list_runs with valid Bearer token must succeed"
    );

    let res = api::handlers::workflows::list_pending_approvals(
        State(state.clone()),
        authed_headers.clone(),
    )
    .await;
    assert!(
        res.is_ok(),
        "list_pending_approvals with valid Bearer token must succeed"
    );

    // Clean up env var
    std::env::remove_var("WORPEN_API_TOKEN");
}

#[tokio::test]
async fn test_mcp_universal_secret_redaction_and_anti_spoofing() {
    let (state, _data_dir) = setup_test_app().await;

    let canary = "canary_ultra_secret_xyz_777";
    state
        .dynamic_route_service
        .secret_store()
        .set_secret("CANARY_KEY", canary);

    // 1. Workflow that triggers an error containing the secret
    let err_wf_yaml = r#"
name: error_leak_test
version: "1.0"
steps:
  - id: step_fail
    op: fail
    message: "Critical failure using secret {{secret:CANARY_KEY}}"
return:
  status: 500
  value: {}
"#;
    state
        .workflow_service
        .register_workflow(err_wf_yaml)
        .await
        .unwrap();

    let call_err_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(201)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "workflow:error_leak_test",
            "arguments": {}
        })),
    };
    let Json(err_resp) =
        mcp_handler(State(state.clone()), HeaderMap::new(), Json(call_err_req)).await;
    let err_resp_str = serde_json::to_string(&err_resp).unwrap();
    assert!(
        !err_resp_str.contains(canary),
        "Secret leaked in MCP error response: {}",
        err_resp_str
    );
    assert_eq!(err_resp["result"]["isError"], true);

    // 2. Workflow that pauses at approval with secret in summary
    let approval_wf_yaml = r#"
name: approval_secret_test
version: "1.0"
input_schema:
  type: object
  properties:
    __initiator:
      type: string
steps:
  - id: gate_step
    op: approval
    summary: "Approve deployment with token {{secret:CANARY_KEY}}"
return:
  status: 200
  value: "done"
"#;
    state
        .workflow_service
        .register_workflow(approval_wf_yaml)
        .await
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("x-agent-role", "financial_agent".parse().unwrap());

    // Caller attempts payload spoofing via arguments: {"__initiator": "spoofed_admin"}
    let call_approval_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(202)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "workflow:approval_secret_test",
            "arguments": {
                "__initiator": "spoofed_admin"
            }
        })),
    };

    let Json(resp_app) = mcp_handler(
        State(state.clone()),
        headers.clone(),
        Json(call_approval_req),
    )
    .await;
    let app_str = serde_json::to_string(&resp_app).unwrap();
    assert!(
        !app_str.contains(canary),
        "Secret leaked in MCP approval response: {}",
        app_str
    );

    let err_fields = &resp_app["result"]["structuredError"]["fields"];
    let run_id = err_fields["run_id"]
        .as_str()
        .expect("run_id in error fields");

    // Check list_pending_approvals via MCP
    let list_req = RpcRequest {
        jsonrpc: Some("2.0".to_string()),
        id: Some(json!(203)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "list_pending_approvals",
            "arguments": {}
        })),
    };
    let Json(list_resp) = mcp_handler(State(state.clone()), HeaderMap::new(), Json(list_req)).await;
    let list_str = serde_json::to_string(&list_resp).unwrap();
    assert!(
        !list_str.contains(canary),
        "Secret leaked in list_pending_approvals: {}",
        list_str
    );

    // 3. Verify initiator is 'agent:financial_agent', NOT 'spoofed_admin'
    let run = state
        .workflow_service
        .get_run(run_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        run.initiator.as_deref(),
        Some("agent:financial_agent"),
        "Payload spoofing must not overwrite authenticated agent identity!"
    );

    // Self-approval by 'agent:financial_agent' through operator channel must FAIL
    let self_approve_err = state
        .workflow_service
        .approve_run(run_id, "agent:financial_agent", "self sign-off")
        .await
        .unwrap_err();
    assert!(self_approve_err.contains("Self-approval forbidden"));

    // Distinct operator approves
    let approved = state
        .workflow_service
        .approve_run(run_id, "security_officer_bob", "legitimate")
        .await
        .unwrap();
    assert_eq!(approved.status, proto::models::RunStatus::Completed);
}
