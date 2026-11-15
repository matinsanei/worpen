use proto::models::{ExecutionMode, RunStatus};
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs;
use std::sync::Arc;
use worpen_core::services::DynamicRouteService;
use worpen_core::workflow::service::WorkflowService;

#[tokio::test]
async fn test_approval_gate_pause_and_approve_cycle() {
    let temp_dir =
        std::env::temp_dir().join(format!("worpen_durable_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // Create business table
    sqlx::query(
        "CREATE TABLE refunds (id INTEGER PRIMARY KEY, customer TEXT, amount REAL, status TEXT);",
    )
    .execute(&db_pool)
    .await
    .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool.clone());
    let route_service_arc = Arc::new(route_service);

    let workflow_service = WorkflowService::new(data_dir.clone(), route_service_arc.clone());

    // Workflow with an approval gate between step 1 and step 2
    let yaml = r#"
name: refund_workflow
version: "1.0"
description: Refund demo with human approval gate
policy:
  allow_sql: true
  read_only_sql: false
input_schema:
  type: object
  properties:
    customer:
      type: string
    amount:
      type: number
steps:
  - id: step_calc
    op: transform
    expr: "{{amount}} * 1.0"
    output_var: formatted_amount

  - id: gate_manager_approval
    op: approval
    summary: "Approve refund of ${{formatted_amount}} for customer {{customer}}"
    timeout_seconds: 3600

  - id: step_record_refund
    op: sql
    query: "INSERT INTO refunds (customer, amount, status) VALUES ($1, $2, 'approved')"
    args:
      - "{{customer}}"
      - "{{formatted_amount}}"
    output_var: insert_res

return:
  status: 200
  value:
    refund_status: "processed"
    customer: "{{customer}}"
    amount: "{{formatted_amount}}"
    approved_by: "{{__approved_by}}"
"#;

    workflow_service.register_workflow(yaml).await.unwrap();

    // 1. Start workflow with an initial payload
    let payload = json!({
        "customer": "alice@corp.com",
        "amount": 250
    });

    let run_res = workflow_service
        .start_run("refund_workflow", Some(payload), ExecutionMode::Live)
        .await;

    assert!(run_res.is_ok(), "Failed to start run: {:?}", run_res.err());
    let run = run_res.unwrap();

    // 2. Assert workflow paused at the approval gate
    assert_eq!(run.status, RunStatus::WaitingForApproval);
    assert_eq!(run.step_index, 1);
    assert!(run.pending_approval.is_some());

    let pending = run.pending_approval.as_ref().unwrap();
    assert_eq!(pending.step_id, "gate_manager_approval");
    assert_eq!(
        pending.summary,
        "Approve refund of $250 for customer alice@corp.com"
    );

    // Assert database insert has NOT yet executed
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM refunds")
        .fetch_one(&db_pool)
        .await
        .unwrap();
    assert_eq!(
        count.0, 0,
        "SQL step executed before approval gate sign-off!"
    );

    // 3. Query list_pending_approvals
    let pending_runs = workflow_service.list_pending_approvals().await.unwrap();
    assert_eq!(pending_runs.len(), 1);
    assert_eq!(pending_runs[0].run_id, run.run_id);

    // 4. Human approves the run
    let approved_res = workflow_service
        .approve_run(
            &run.run_id,
            "finance_lead_carol",
            "Customer verified on Zendesk",
        )
        .await;

    assert!(
        approved_res.is_ok(),
        "Failed to approve run: {:?}",
        approved_res.err()
    );
    let approved_run = approved_res.unwrap();

    // 5. Assert workflow resumed and completed
    assert_eq!(approved_run.status, RunStatus::Completed);
    assert_eq!(approved_run.step_index, 3);
    assert!(approved_run.result.is_some());

    let result_obj = approved_run.result.unwrap();
    assert_eq!(result_obj["refund_status"], "processed");
    assert_eq!(result_obj["customer"], "alice@corp.com");
    assert_eq!(result_obj["amount"], 250);
    assert_eq!(result_obj["approved_by"], "finance_lead_carol");

    // 6. Verify SQL step executed after approval
    let refund_row: (String, f64, String) =
        sqlx::query_as("SELECT customer, amount, status FROM refunds WHERE id = 1")
            .fetch_one(&db_pool)
            .await
            .unwrap();
    assert_eq!(refund_row.0, "alice@corp.com");
    assert_eq!(refund_row.1, 250.0);
    assert_eq!(refund_row.2, "approved");

    // 7. Assert pending list is now empty
    let remaining_pending = workflow_service.list_pending_approvals().await.unwrap();
    assert_eq!(remaining_pending.len(), 0);
}

#[tokio::test]
async fn test_approval_gate_rejection() {
    let temp_dir =
        std::env::temp_dir().join(format!("worpen_durable_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool.clone());
    let workflow_service = WorkflowService::new(data_dir.clone(), Arc::new(route_service));

    let yaml = r#"
name: dangerous_action
version: "1.0"
steps:
  - id: gate
    op: approval
    summary: "Approve dangerous action"
return:
  status: 200
  value: "done"
"#;

    workflow_service.register_workflow(yaml).await.unwrap();

    let run = workflow_service
        .start_run("dangerous_action", None, ExecutionMode::Live)
        .await
        .unwrap();

    assert_eq!(run.status, RunStatus::WaitingForApproval);

    // Reject the run
    let rejected = workflow_service
        .reject_run(&run.run_id, "sec_ops", "Unusual IP address detected")
        .await
        .unwrap();

    assert_eq!(rejected.status, RunStatus::Rejected);
    assert!(rejected.error.unwrap().contains("Unusual IP address"));

    // Attempting to approve a rejected run must fail
    let approve_attempt = workflow_service
        .approve_run(&run.run_id, "someone", "retry")
        .await;
    assert!(approve_attempt.is_err());
}

#[tokio::test]
async fn test_durable_resumption_across_restart() {
    let temp_dir =
        std::env::temp_dir().join(format!("worpen_durable_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_file = temp_dir.join("test_durable.db");
    let db_url = format!("sqlite://{}?mode=rwc", db_file.to_str().unwrap());

    let db_pool = SqlitePoolOptions::new().connect(&db_url).await.unwrap();

    let yaml = r#"
name: restart_test
version: "1.0"
steps:
  - id: step1
    op: set
    var: persistent_value
    value: "saved_in_sqlite"
  - id: approval_gate
    op: approval
    summary: "Sign-off"
return:
  status: 200
  value:
    persisted: "{{persistent_value}}"
"#;

    let run_id = {
        let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
        route_service.set_db_pool(db_pool.clone());
        let wf_svc = WorkflowService::new(data_dir.clone(), Arc::new(route_service));
        wf_svc.register_workflow(yaml).await.unwrap();

        let run = wf_svc
            .start_run("restart_test", None, ExecutionMode::Live)
            .await
            .unwrap();
        assert_eq!(run.status, RunStatus::WaitingForApproval);
        run.run_id
    };

    // Simulate complete process restart: open new connections and instances
    {
        let new_pool = SqlitePoolOptions::new().connect(&db_url).await.unwrap();

        let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
        route_service.set_db_pool(new_pool);
        let wf_svc_restored = WorkflowService::new(data_dir.clone(), Arc::new(route_service));
        wf_svc_restored.load_persisted_workflows().await.unwrap();

        // 1. Run state successfully retrieved across restart
        let retrieved = wf_svc_restored.get_run(&run_id).await.unwrap().unwrap();
        assert_eq!(retrieved.status, RunStatus::WaitingForApproval);
        assert_eq!(retrieved.variables["persistent_value"], "saved_in_sqlite");

        // 2. Approve and finish execution seamlessly
        let completed = wf_svc_restored
            .approve_run(&run_id, "admin", "Restart verified")
            .await
            .unwrap();

        assert_eq!(completed.status, RunStatus::Completed);
        assert_eq!(completed.result.unwrap()["persisted"], "saved_in_sqlite");
    }
}

#[tokio::test]
async fn test_durable_idempotency_deduplication() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool.clone());
    let wf_svc = WorkflowService::new(data_dir.clone(), Arc::new(route_service));

    let yaml = r#"
name: idempotent_test
version: "1.0"
description: Idempotency deduplication test
steps:
  - id: step1
    op: transform
    expr: "100 + 50"
    output_var: sum
return:
  status: 200
  value:
    sum: "{{sum}}"
"#;
    wf_svc.register_workflow(yaml).await.unwrap();

    let payload = Some(serde_json::json!({
        "idempotency_key": "payment_tx_998877",
        "customer": "bob"
    }));

    // First call: executes and completes
    let run1 = wf_svc
        .start_run_with_meta(
            "idempotent_test",
            payload.clone(),
            ExecutionMode::Live,
            Some("agent:order_worker".to_string()),
            Some("payment_tx_998877".to_string()),
        )
        .await
        .unwrap();

    assert_eq!(run1.status, RunStatus::Completed);
    assert_eq!(run1.idempotency_key.as_deref(), Some("payment_tx_998877"));

    // Second call with same idempotency key: returns existing run without re-running!
    let run2 = wf_svc
        .start_run_with_meta(
            "idempotent_test",
            payload,
            ExecutionMode::Live,
            Some("agent:order_worker".to_string()),
            Some("payment_tx_998877".to_string()),
        )
        .await
        .unwrap();

    assert_eq!(run2.run_id, run1.run_id);
    assert_eq!(run2.status, RunStatus::Completed);
    assert_eq!(run2.result, run1.result);
}

#[tokio::test]
async fn test_durable_sql_execution_and_read_only_enforcement() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_sql_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // 1. Create table and populate sample rows
    sqlx::query(
        "CREATE TABLE products (id INTEGER PRIMARY KEY, sku TEXT NOT NULL, price REAL NOT NULL, active INTEGER NOT NULL);",
    )
    .execute(&db_pool)
    .await
    .unwrap();

    sqlx::query("INSERT INTO products (sku, price, active) VALUES ('PROD-A', 49.99, 1);")
        .execute(&db_pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO products (sku, price, active) VALUES ('PROD-B', 19.99, 1);")
        .execute(&db_pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO products (sku, price, active) VALUES ('PROD-C', 89.50, 0);")
        .execute(&db_pool)
        .await
        .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool.clone());
    let wf_svc = WorkflowService::new(data_dir.clone(), Arc::new(route_service));

    // 2. Read-only SQL workflow: Executes SELECT query and populates output_var
    let read_yaml = r#"
name: get_active_products
version: "1.0"
policy:
  allow_sql: true
  read_only_sql: true
  allowed_tables:
    - "products"
input_schema:
  type: object
  properties:
    min_price:
      type: number
steps:
  - id: step_query
    op: sql
    query: "SELECT sku, price FROM products WHERE active = 1 AND price >= $1 ORDER BY price ASC"
    args:
      - "{{min_price}}"
    output_var: active_products
return:
  status: 200
  value:
    products: "{{active_products}}"
"#;

    wf_svc.register_workflow(read_yaml).await.unwrap();

    let payload = json!({ "min_price": 20.0 });
    let run = wf_svc
        .start_run("get_active_products", Some(payload), ExecutionMode::Live)
        .await
        .unwrap();

    assert_eq!(run.status, RunStatus::Completed);
    // output_var active_products must be populated in run.variables
    let active_products = run
        .variables
        .get("active_products")
        .expect("active_products populated");
    assert!(active_products.is_array());
    let prods = active_products.as_array().unwrap();
    assert_eq!(prods.len(), 1, "Only PROD-A is active and >= 20.0");
    assert_eq!(prods[0]["sku"], "PROD-A");
    assert_eq!(prods[0]["price"], 49.99);

    // 3. Mutating SQL statement (INSERT) under read_only_sql: true must FAIL
    let mutate_yaml = r#"
name: attack_insert_products
version: "1.0"
policy:
  allow_sql: true
  read_only_sql: true
  allowed_tables:
    - "products"
steps:
  - id: step_insert
    op: sql
    query: "INSERT INTO products (sku, price, active) VALUES ('PROD-EVIL', 0.0, 1)"
    args: []
    output_var: insert_res
return:
  status: 200
  value: "done"
"#;

    wf_svc.register_workflow(mutate_yaml).await.unwrap();
    let err_run = wf_svc
        .start_run("attack_insert_products", None, ExecutionMode::Live)
        .await;

    assert!(
        err_run.is_err(),
        "Mutating INSERT query must fail under read_only_sql: true"
    );
    let err_msg = err_run.unwrap_err();
    assert!(
        err_msg.contains("read_only_sql=true") || err_msg.contains("SQL write operation denied"),
        "Error message must specify read_only restriction: {}",
        err_msg
    );

    // Verify row count in DB has not changed
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&db_pool)
        .await
        .unwrap();
    assert_eq!(
        count.0, 3,
        "No rows should have been inserted into products"
    );

    // 4. Mutating SQL statement (UPDATE) under read_only_sql: true must also FAIL
    let update_yaml = r#"
name: attack_update_products
version: "1.0"
policy:
  allow_sql: true
  read_only_sql: true
  allowed_tables:
    - "products"
steps:
  - id: step_update
    op: sql
    query: "UPDATE products SET price = 0.0 WHERE sku = 'PROD-A'"
    args: []
    output_var: update_res
return:
  status: 200
  value: "done"
"#;
    wf_svc.register_workflow(update_yaml).await.unwrap();
    let err_update = wf_svc
        .start_run("attack_update_products", None, ExecutionMode::Live)
        .await;
    assert!(err_update.is_err());

    // 5. Mutating SQL succeeds when read_only_sql: false
    let write_allowed_yaml = r#"
name: allowed_insert_product
version: "1.0"
policy:
  allow_sql: true
  read_only_sql: false
  allowed_tables:
    - "products"
steps:
  - id: step_insert_ok
    op: sql
    query: "INSERT INTO products (sku, price, active) VALUES ($1, $2, $3)"
    args:
      - "PROD-NEW"
      - 34.50
      - 1
    output_var: insert_ok_res
return:
  status: 200
  value:
    inserted: true
"#;
    wf_svc.register_workflow(write_allowed_yaml).await.unwrap();
    let ok_run = wf_svc
        .start_run("allowed_insert_product", None, ExecutionMode::Live)
        .await
        .unwrap();
    assert_eq!(ok_run.status, RunStatus::Completed);

    let count_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&db_pool)
        .await
        .unwrap();
    assert_eq!(
        count_after.0, 4,
        "Row count must be 4 after legitimate insert"
    );
}

#[tokio::test]
async fn test_anti_self_approval_payload_spoofing_prevented() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_anti_self_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool);
    let wf_svc = WorkflowService::new(data_dir.clone(), Arc::new(route_service));

    let yaml = r#"
name: wire_transfer
version: "1.0"
description: Wire transfer requiring distinct operator sign-off
input_schema:
  type: object
  properties:
    __initiator:
      type: string
    amount:
      type: number
    recipient:
      type: string
steps:
  - id: gate_manager
    op: approval
    summary: "Approve transfer of ${{amount}} to {{recipient}}"
return:
  status: 200
  value:
    status: "dispatched"
    approved_by: "{{__approved_by}}"
"#;

    wf_svc.register_workflow(yaml).await.unwrap();

    // 1. Attacker (caller identity "evil_alice") initiates run, but injects "__initiator": "innocent_bob" in payload
    let spoof_payload = json!({
        "__initiator": "innocent_bob",
        "amount": 50000,
        "recipient": "offshore_account"
    });

    let run = wf_svc
        .start_run_with_meta(
            "wire_transfer",
            Some(spoof_payload),
            ExecutionMode::Live,
            Some("evil_alice".to_string()),
            None,
        )
        .await
        .unwrap();

    assert_eq!(run.status, RunStatus::WaitingForApproval);
    // Initiator must be "evil_alice", payload.__initiator MUST BE COMPLETELY IGNORED!
    assert_eq!(
        run.initiator.as_deref(),
        Some("evil_alice"),
        "Workflow run initiator must strictly reflect caller identity, not spoofed payload!"
    );

    // 2. Real initiator ("evil_alice") attempts to approve her own run
    let self_approve_err = wf_svc
        .approve_run(&run.run_id, "evil_alice", "Approved by manager")
        .await
        .unwrap_err();

    assert!(
        self_approve_err.contains("Self-approval forbidden"),
        "Initiator 'evil_alice' must be blocked from self-approval: {}",
        self_approve_err
    );
    assert!(
        self_approve_err.contains("evil_alice"),
        "Error message should mention initiator 'evil_alice': {}",
        self_approve_err
    );

    // 3. Obfuscation bypass attempts: case sensitivity & whitespace tricks
    let caps_approve_err = wf_svc
        .approve_run(&run.run_id, "EVIL_ALICE", "Caps bypass attempt")
        .await
        .unwrap_err();
    assert!(caps_approve_err.contains("Self-approval forbidden"));

    let space_approve_err = wf_svc
        .approve_run(&run.run_id, " evil_alice ", "Whitespace bypass attempt")
        .await
        .unwrap_err();
    assert!(space_approve_err.contains("Self-approval forbidden"));

    // 4. Authorized distinct human operator can approve the run
    let ok_approve = wf_svc
        .approve_run(&run.run_id, "supervisor_carol", "Legitimate wire verified")
        .await
        .expect("Distinct supervisor approval must succeed");

    assert_eq!(ok_approve.status, RunStatus::Completed);
    assert_eq!(
        ok_approve.result.unwrap()["approved_by"],
        "supervisor_carol"
    );

    // 5. Payload spoofing when caller identity is None: defaults to "unknown_agent"
    let unauthed_payload = json!({
        "__initiator": "trusted_admin",
        "amount": 100
    });
    let run2 = wf_svc
        .start_run_with_meta(
            "wire_transfer",
            Some(unauthed_payload),
            ExecutionMode::Live,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(run2.initiator.as_deref(), Some("unknown_agent"));
    let unauth_approve_err = wf_svc
        .approve_run(&run2.run_id, "unknown_agent", "Self approval")
        .await
        .unwrap_err();
    assert!(unauth_approve_err.contains("Self-approval forbidden"));
}

#[tokio::test]
async fn test_concurrent_idempotency_key_deduplication() {
    let temp_dir =
        std::env::temp_dir().join(format!("worpen_concurrent_idem_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool.clone());
    let wf_svc = Arc::new(WorkflowService::new(
        data_dir.clone(),
        Arc::new(route_service),
    ));

    let yaml = r#"
name: concurrent_tx_test
version: "1.0"
steps:
  - id: step_math
    op: transform
    expr: "50 * 2"
    output_var: val
return:
  status: 200
  value:
    result: "{{val}}"
"#;
    wf_svc.register_workflow(yaml).await.unwrap();

    let idem_key = format!("tx_race_{}", uuid::Uuid::new_v4());
    let payload = json!({ "account": "acc_123" });

    // Spawn 10 concurrent requests with the identical idempotency key
    let mut handles = Vec::new();
    for i in 0..10 {
        let svc = wf_svc.clone();
        let key = idem_key.clone();
        let p = payload.clone();
        let handle = tokio::spawn(async move {
            svc.start_run_with_meta(
                "concurrent_tx_test",
                Some(p),
                ExecutionMode::Live,
                Some(format!("worker_{}", i)),
                Some(key),
            )
            .await
        });
        handles.push(handle);
    }

    let mut run_ids = Vec::new();
    for h in handles {
        let res = h.await.unwrap();
        assert!(res.is_ok(), "Concurrent start_run failed: {:?}", res.err());
        let run = res.unwrap();
        // A concurrent caller either initiates the run (transitioning to Completed) or
        // deduplicates against the in-flight/completed run record without raising an error.
        assert!(
            run.status == RunStatus::Running || run.status == RunStatus::Completed,
            "Run status must be Running or Completed, got: {:?}",
            run.status
        );
        run_ids.push(run.run_id);
    }

    // All 10 concurrent requests must resolve to the EXACT SAME run_id
    let first_id = &run_ids[0];
    for id in &run_ids {
        assert_eq!(
            id, first_id,
            "Concurrent idempotency race produced distinct run_ids!"
        );
    }

    // Verify terminal completion and output of the deduplicated run
    let final_run = wf_svc.get_run(first_id).await.unwrap().unwrap();
    assert_eq!(final_run.status, RunStatus::Completed);
    assert_eq!(final_run.result.unwrap()["result"], 100);

    // Verify in database that exactly ONE row was created with this idempotency key
    let _engine = wf_svc.ensure_durable_engine().await.unwrap();
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM workflow_runs WHERE idempotency_key = $1")
            .bind(&idem_key)
            .fetch_one(&db_pool)
            .await
            .unwrap();

    assert_eq!(
        count.0, 1,
        "Exactly 1 database row should exist for idempotency key"
    );
}
