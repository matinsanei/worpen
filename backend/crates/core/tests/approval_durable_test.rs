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
