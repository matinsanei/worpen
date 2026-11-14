use proto::models::ExecutionMode;
use std::fs;
use std::sync::Arc;
use worpen_core::secrets::{FileSecretSource, SecretStore};
use worpen_core::services::DynamicRouteService;
use worpen_core::workflow::service::WorkflowService;

#[tokio::test]
async fn test_canary_secrets_isolation_and_redaction() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    // 1. Plant canary secrets across multiple sources
    let mem_canary = "sk_live_canary_super_secret_token_12345";
    let env_canary = "canary_env_secret_val_67890";
    let file_canary = "canary_file_secret_val_443322";

    // Setup file secret
    let secrets_dir = temp_dir.join("secrets");
    fs::create_dir_all(&secrets_dir).unwrap();
    fs::write(secrets_dir.join("db_password.txt"), file_canary).unwrap();

    // Setup env secret
    std::env::set_var("WORPEN_SECRET_CANARY_ENV", env_canary);

    // Initialize SecretStore and DynamicRouteService
    let mut secret_store = SecretStore::new();
    secret_store.set_secret("stripe_key", mem_canary);
    secret_store.add_source(Box::new(FileSecretSource::new(&secrets_dir)));

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    let store_arc = Arc::new(secret_store);
    route_service.set_secret_store(store_arc.clone());

    // Setup SQLite DB pool
    let db_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE audit_vault (id INTEGER PRIMARY KEY, secret_value TEXT, notes TEXT);",
    )
    .execute(&db_pool)
    .await
    .unwrap();

    route_service.set_db_pool(db_pool.clone());
    let route_service_arc = Arc::new(route_service);

    // Initialize WorkflowService
    let workflow_service = WorkflowService::new(data_dir.clone(), route_service_arc.clone());

    // 2. Define workflow that inserts secrets into SQL and echoes transformed values
    let yaml = r#"
name: secret_vault_test
version: "1.0"
description: Canary secret isolation test
steps:
  - id: step_insert
    op: sql
    query: "INSERT INTO audit_vault (secret_value, notes) VALUES ($1, $2)"
    args:
      - "{{secret:stripe_key}}"
      - "{{secret:db_password}}"
    output_var: insert_res

  - id: step_read
    op: sql
    query: "SELECT id, secret_value, notes FROM audit_vault WHERE id = 1"
    args: []
    output_var: db_records

return:
  status: 200
  value:
    inserted: "{{insert_res}}"
    records: "{{db_records}}"
    direct_ref: "{{secret:CANARY_ENV}}"
"#;

    let registered = workflow_service.register_workflow(yaml).await;
    assert!(
        registered.is_ok(),
        "Workflow registration failed: {:?}",
        registered.err()
    );

    // 3. Execute workflow
    let result = workflow_service
        .execute("secret_vault_test", None, ExecutionMode::Live)
        .await;
    assert!(result.is_ok(), "Execution failed: {:?}", result.err());

    let output_val = result.unwrap();
    let output_str = serde_json::to_string_pretty(&output_val).unwrap();

    // 4. Verify that the SQLite DB received the ACTUAL unredacted secrets
    let row: (String, String) =
        sqlx::query_as("SELECT secret_value, notes FROM audit_vault WHERE id = 1")
            .fetch_one(&db_pool)
            .await
            .unwrap();
    assert_eq!(
        row.0, mem_canary,
        "Database did not receive actual memory secret!"
    );
    assert_eq!(
        row.1, file_canary,
        "Database did not receive actual file secret!"
    );

    // 5. Verify that output returned to the agent has been automatically REDACTED
    assert!(
        output_str.contains("[REDACTED]"),
        "Output did not contain [REDACTED] marker: {}",
        output_str
    );
    assert!(
        !output_str.contains(mem_canary),
        "Memory canary secret leaked into agent output!"
    );
    assert!(
        !output_str.contains(file_canary),
        "File canary secret leaked into agent output!"
    );
    assert!(
        !output_str.contains(env_canary),
        "Env canary secret leaked into agent output!"
    );

    // 6. Check audit log file and in-memory ring
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let audit_file = format!("{}/audit.log", data_dir);
    let audit_file_content = fs::read_to_string(&audit_file).unwrap_or_default();
    let recent_audits = route_service_arc.recent_audit(10).await;
    let recent_audit_str = serde_json::to_string(&recent_audits).unwrap();

    // Grep combined string of all logs, audit trails, and outputs
    let combined_trail = format!(
        "{}\n{}\n{}",
        output_str, audit_file_content, recent_audit_str
    );

    assert!(
        !combined_trail.contains(mem_canary),
        "Memory canary leaked into audit trail or logs!"
    );
    assert!(
        !combined_trail.contains(file_canary),
        "File canary leaked into audit trail or logs!"
    );
    assert!(
        !combined_trail.contains(env_canary),
        "Env canary leaked into audit trail or logs!"
    );
}

#[tokio::test]
async fn test_canary_secrets_redacted_in_errors() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let canary_err_token = "secret_canary_error_token_999";
    let secret_store = SecretStore::new();
    secret_store.set_secret("error_token", canary_err_token);

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_secret_store(Arc::new(secret_store));

    let db_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();
    route_service.set_db_pool(db_pool);
    let route_service_arc = Arc::new(route_service);

    let workflow_service = WorkflowService::new(data_dir.clone(), route_service_arc.clone());

    // Workflow with an intentional SQL error that echoes the secret in the query string
    let yaml = r#"
name: secret_error_test
version: "1.0"
description: Secret error redaction test
steps:
  - id: step_fail
    op: sql
    query: "SELECT 1 FROM \"missing_table_{{secret:error_token}}\""
    args: []
    output_var: res

return:
  status: 200
  value: "{{res}}"
"#;

    workflow_service.register_workflow(yaml).await.unwrap();

    let err_result = workflow_service
        .execute("secret_error_test", None, ExecutionMode::Live)
        .await;

    assert!(err_result.is_err(), "Expected workflow to fail with error");
    let err_msg = err_result.err().unwrap();

    // Verify error message is redacted and contains zero secret leakage
    assert!(
        err_msg.contains("[REDACTED]"),
        "Error message did not redact secret: {}",
        err_msg
    );
    assert!(
        !err_msg.contains(canary_err_token),
        "Secret leaked in error message: {}",
        err_msg
    );

    // Verify audit log also redacted the error
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let audit_file = format!("{}/audit.log", data_dir);
    let audit_file_content = fs::read_to_string(&audit_file).unwrap_or_default();

    assert!(
        !audit_file_content.contains(canary_err_token),
        "Secret leaked into audit log error field!"
    );
}

#[tokio::test]
async fn test_canary_secret_http_injection_and_reflection_redaction() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let canary_api_token = "sk_live_outbound_http_canary_token_778899";

    // Start a lightweight mock HTTP server
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let server_handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0u8; 2048];
        let n = socket.read(&mut buf).await.unwrap();
        let req_text = String::from_utf8_lossy(&buf[..n]).to_string();

        // 1. Verify that the outbound HTTP request DOES contain the real unredacted secret
        assert!(
            req_text.contains("Bearer sk_live_outbound_http_canary_token_778899"),
            "Outbound HTTP request was missing unredacted secret header! Received:\n{}",
            req_text
        );

        // Echo the secret back in HTTP response body to test redaction of server responses
        let body = format!(r#"{{"status":"ok","echo_secret":"{}"}}"#, canary_api_token);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        socket.write_all(response.as_bytes()).await.unwrap();
    });

    let temp_dir = std::env::temp_dir().join(format!("worpen_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let secret_store = SecretStore::new();
    secret_store.set_secret("http_token", canary_api_token);

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_secret_store(Arc::new(secret_store));
    let route_service_arc = Arc::new(route_service);

    let workflow_service = WorkflowService::new(data_dir.clone(), route_service_arc.clone());

    let yaml = format!(
        r#"
name: http_secret_test
version: "1.0"
description: HTTP secret injection and redaction test
steps:
  - id: step_http
    op: http
    method: POST
    url: "http://127.0.0.1:{}/api/charge"
    headers:
      Authorization: "Bearer {{{{secret:http_token}}}}"
    output_var: http_res

return:
  status: 200
  value:
    res: "{{{{http_res}}}}"
"#,
        port
    );

    workflow_service.register_workflow(&yaml).await.unwrap();

    let result = workflow_service
        .execute("http_secret_test", None, ExecutionMode::Live)
        .await;

    server_handle.await.unwrap();

    assert!(
        result.is_ok(),
        "HTTP workflow execution failed: {:?}",
        result.err()
    );
    let output_val = result.unwrap();
    let output_str = serde_json::to_string(&output_val).unwrap();

    // 2. Verify that the agent NEVER sees the secret, even when echoed by third-party server
    assert!(
        !output_str.contains(canary_api_token),
        "Secret leaked in output string: {}",
        output_str
    );
    assert!(
        output_str.contains("[REDACTED]"),
        "Output string missing [REDACTED]: {}",
        output_str
    );

    // 3. Verify audit log has 0 leakage
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let audit_file = format!("{}/audit.log", data_dir);
    let audit_file_content = fs::read_to_string(&audit_file).unwrap_or_default();
    assert!(
        !audit_file_content.contains(canary_api_token),
        "Secret leaked in audit log: {}",
        audit_file_content
    );
}
