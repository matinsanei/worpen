use proto::models::ExecutionMode;
use sqlx::SqlitePool;
use std::sync::Arc;
use worpen_core::services::DynamicRouteService;
use worpen_core::workflow::service::WorkflowService;

#[tokio::test]
async fn test_workflow_spec_v1_validation_undefined_variable() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let data_dir = format!("target/test_wf_validation_{}", uuid::Uuid::new_v4());
    let mut route_svc = DynamicRouteService::with_data_dir(data_dir.clone());
    route_svc.set_db_pool(pool);
    let wf_svc = WorkflowService::new(data_dir, Arc::new(route_svc));

    let invalid_yaml = r#"
version: "1.0"
name: test_invalid
description: "Uses undefined variable"
steps:
  - id: step1
    op: transform
    expr: "{{ghost_variable}} * 2"
    output_var: result
return:
  status: 200
  value: "{{result}}"
"#;

    let res = wf_svc.register_workflow(invalid_yaml).await;
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("undefined variable 'ghost_variable'"));
}

#[tokio::test]
async fn test_workflow_spec_v1_validation_unbounded_loop() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let data_dir = format!("target/test_wf_loop_{}", uuid::Uuid::new_v4());
    let mut route_svc = DynamicRouteService::with_data_dir(data_dir.clone());
    route_svc.set_db_pool(pool);
    let wf_svc = WorkflowService::new(data_dir, Arc::new(route_svc));

    let invalid_yaml = r#"
version: "1.0"
name: test_loop_cap
steps:
  - id: loop1
    op: loop
    items: "{{payload.items}}"
    as_var: item
    max_iterations: 0
    body:
      - id: inner
        op: set
        var: x
        value: 1
return:
  status: 200
  value: "done"
"#;

    let res = wf_svc.register_workflow(invalid_yaml).await;
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("max_iterations must be greater than 0"));
}

#[tokio::test]
async fn test_workflow_spec_v1_dry_run_simulation() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let data_dir = format!("target/test_wf_dryrun_{}", uuid::Uuid::new_v4());
    let mut route_svc = DynamicRouteService::with_data_dir(data_dir.clone());
    route_svc.set_db_pool(pool);
    let wf_svc = WorkflowService::new(data_dir, Arc::new(route_svc));

    let yaml = r#"
version: "1.0"
name: refund_sim
description: "Simulate order refund with approval"
input_schema:
  type: object
  required: [order_id]
  properties:
    order_id:
      type: string
steps:
  - id: lookup
    op: sql
    query: "SELECT total FROM orders WHERE id = ?"
    args: ["{{order_id}}"]
    output_var: order
  - id: approve
    op: approval
    summary: "Approve refund for order #{{order_id}}"
  - id: refund_api
    op: http
    method: POST
    url: "https://api.stripe.com/v1/refunds"
    idempotency_key: "refund-{{order_id}}"
    output_var: stripe_res
return:
  status: 200
  value:
    order_id: "{{order_id}}"
    stripe: "{{stripe_res}}"
    approved: "{{approve_approved}}"
"#;

    let spec = wf_svc.register_workflow(yaml).await.unwrap();
    assert_eq!(spec.name, "refund_sim");

    let payload = serde_json::json!({
        "order_id": "ord_999"
    });

    let res = wf_svc
        .execute("refund_sim", Some(payload), ExecutionMode::DryRun)
        .await
        .unwrap();

    assert_eq!(res["_worpen_mode"], "dry_run");
    assert_eq!(res["status"], 200);

    let result = &res["result"];
    assert_eq!(result["order_id"], "ord_999");
    assert_eq!(result["stripe"]["_worpen_dry_run"], true);
    assert_eq!(result["stripe"]["idempotency_key"], "refund-ord_999");
    assert_eq!(result["approved"]["_worpen_dry_run"], true);
}

#[tokio::test]
async fn test_workflow_spec_v1_versioning_and_rollback() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let data_dir = format!("target/test_wf_version_{}", uuid::Uuid::new_v4());
    let mut route_svc = DynamicRouteService::with_data_dir(data_dir.clone());
    route_svc.set_db_pool(pool);
    let wf_svc = WorkflowService::new(data_dir, Arc::new(route_svc));

    let v1_yaml = r#"
version: "1.0.0"
name: pricing_calc
steps:
  - id: step1
    op: set
    var: tax_rate
    value: 0.10
return:
  status: 200
  value: "{{tax_rate}}"
"#;

    let v2_yaml = r#"
version: "1.0.1"
name: pricing_calc
steps:
  - id: step1
    op: set
    var: tax_rate
    value: 0.15
return:
  status: 200
  value: "{{tax_rate}}"
"#;

    wf_svc.register_workflow(v1_yaml).await.unwrap();
    assert_eq!(
        wf_svc.get_workflow("pricing_calc").unwrap().version,
        "1.0.0"
    );

    wf_svc.register_workflow(v2_yaml).await.unwrap();
    assert_eq!(
        wf_svc.get_workflow("pricing_calc").unwrap().version,
        "1.0.1"
    );

    let versions = wf_svc.list_versions("pricing_calc");
    assert_eq!(versions, vec!["1.0.0", "1.0.1"]);

    // Rollback to v1.0.0
    let rolled = wf_svc
        .rollback_workflow("pricing_calc", "1.0.0")
        .await
        .unwrap();
    assert_eq!(rolled.version, "1.0.0");
    assert_eq!(
        wf_svc.get_workflow("pricing_calc").unwrap().version,
        "1.0.0"
    );
}

#[tokio::test]
async fn test_workflow_spec_v1_live_execution() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE orders (id INTEGER PRIMARY KEY, customer TEXT, amount REAL)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO orders (id, customer, amount) VALUES (101, 'Acme Corp', 500.0)")
        .execute(&pool)
        .await
        .unwrap();

    let data_dir = format!("target/test_wf_live_{}", uuid::Uuid::new_v4());
    let mut route_svc = DynamicRouteService::with_data_dir(data_dir.clone());
    route_svc.set_db_pool(pool);
    let wf_svc = WorkflowService::new(data_dir, Arc::new(route_svc));

    let yaml = r#"
version: "1.0"
name: calculate_discount
input_schema:
  type: object
  required: [order_id]
  properties:
    order_id:
      type: integer
policy:
  allow_sql: true
steps:
  - id: lookup
    op: sql
    query: "SELECT amount FROM orders WHERE id = ?"
    args: ["{{order_id}}"]
    output_var: order_row
  - id: discount
    op: transform
    expr: "0.20"
    output_var: discount_rate
return:
  status: 200
  value:
    order_id: "{{order_id}}"
    orders: "{{order_row}}"
    discount: "{{discount_rate}}"
"#;

    wf_svc.register_workflow(yaml).await.unwrap();

    let payload = serde_json::json!({
        "order_id": 101
    });

    let res = wf_svc
        .execute("calculate_discount", Some(payload), ExecutionMode::Live)
        .await
        .unwrap();

    assert_eq!(res["status"], 200);
    let val = &res["value"];
    assert_eq!(val["order_id"], 101);
    assert_eq!(val["discount"], 0.20);
    let orders = val["orders"].as_array().unwrap();
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0]["amount"], 500.0);
}
