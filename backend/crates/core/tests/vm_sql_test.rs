use proto::models::LogicOperation;
use serde_json::Value;
use sqlx::SqlitePool;
use tokio::runtime::Runtime;
use worpen_core::compiler::lowerer::LogicCompiler;
use worpen_core::vm::machine::VirtualMachine;
use worpen_core::vm::memory::ExecutionMemory;

#[test]
fn test_vm_sql() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        // Create in-memory SQLite database
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        // Create table
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
            .execute(&pool)
            .await
            .unwrap();

        // Test 1: Insert data using SqlOp with parameterized query
        let logic = vec![
            LogicOperation::Set {
                var: "user_id".to_string(),
                value: Value::Number(1.into()),
            },
            LogicOperation::Set {
                var: "user_name".to_string(),
                value: Value::String("Alice".to_string()),
            },
            LogicOperation::Set {
                var: "user_age".to_string(),
                value: Value::Number(30.into()),
            },
            LogicOperation::SqlOp {
                query: "INSERT INTO users (id, name, age) VALUES (?, ?, ?)".to_string(),
                args: vec![
                    Value::String("{{user_id}}".to_string()),
                    Value::String("{{user_name}}".to_string()),
                    Value::String("{{user_age}}".to_string()),
                ],
                output_var: "insert_result".to_string(),
            },
        ];

        // Compile and execute
        let mut compiler = LogicCompiler::new();
        let optimized = compiler.compile(&logic);
        let symbol_table = compiler.get_symbol_table();

        let memory = ExecutionMemory::new();
        let mut vm = VirtualMachine::with_db_pool(memory, symbol_table.clone(), pool.clone());

        let _result = vm.execute(&optimized).await.unwrap();

        // Test 2: Select data using SqlOp
        let logic = vec![
            LogicOperation::Set {
                var: "search_id".to_string(),
                value: Value::Number(1.into()),
            },
            LogicOperation::SqlOp {
                query: "SELECT id, name, age FROM users WHERE id = ?".to_string(),
                args: vec![Value::String("{{search_id}}".to_string())],
                output_var: "query_result".to_string(),
            },
            LogicOperation::Return {
                value: Value::String("{{query_result}}".to_string()),
                status: None,
                headers: None,
                raw: None,
            },
        ];

        // Compile and execute
        let mut compiler = LogicCompiler::new();
        let optimized = compiler.compile(&logic);
        let symbol_table = compiler.get_symbol_table();

        let memory = ExecutionMemory::new();
        let mut vm = VirtualMachine::with_db_pool(memory, symbol_table.clone(), pool.clone());

        let result = vm.execute(&optimized).await.unwrap();

        // Assert that we got back an array with one user
        if let Value::Array(rows) = result {
            assert_eq!(rows.len(), 1);
            if let Value::Object(row) = &rows[0] {
                assert_eq!(row.get("id").and_then(|v| v.as_i64()), Some(1));
                assert_eq!(row.get("name").and_then(|v| v.as_str()), Some("Alice"));
                assert_eq!(row.get("age").and_then(|v| v.as_i64()), Some(30));
            } else {
                panic!("Expected row to be an object");
            }
        } else {
            panic!("Expected result to be an array, got: {:?}", result);
        }

        // Test 3: Insert multiple users and select all
        let logic = vec![
            LogicOperation::Set {
                var: "user2_id".to_string(),
                value: Value::Number(2.into()),
            },
            LogicOperation::Set {
                var: "user2_name".to_string(),
                value: Value::String("Bob".to_string()),
            },
            LogicOperation::Set {
                var: "user2_age".to_string(),
                value: Value::Number(25.into()),
            },
            LogicOperation::SqlOp {
                query: "INSERT INTO users (id, name, age) VALUES (?, ?, ?)".to_string(),
                args: vec![
                    Value::String("{{user2_id}}".to_string()),
                    Value::String("{{user2_name}}".to_string()),
                    Value::String("{{user2_age}}".to_string()),
                ],
                output_var: "insert_result2".to_string(),
            },
            LogicOperation::SqlOp {
                query: "SELECT id, name, age FROM users ORDER BY id".to_string(),
                args: vec![],
                output_var: "all_users".to_string(),
            },
            LogicOperation::Return {
                value: Value::String("{{all_users}}".to_string()),
                status: None,
                headers: None,
                raw: None,
            },
        ];

        // Compile and execute
        let mut compiler = LogicCompiler::new();
        let optimized = compiler.compile(&logic);
        let symbol_table = compiler.get_symbol_table();

        let memory = ExecutionMemory::new();
        let mut vm = VirtualMachine::with_db_pool(memory, symbol_table.clone(), pool.clone());

        let result = vm.execute(&optimized).await.unwrap();

        // Assert that we got back two users
        if let Value::Array(rows) = result {
            assert_eq!(rows.len(), 2);

            // Check first user
            if let Value::Object(row) = &rows[0] {
                assert_eq!(row.get("id").and_then(|v| v.as_i64()), Some(1));
                assert_eq!(row.get("name").and_then(|v| v.as_str()), Some("Alice"));
            } else {
                panic!("Expected first row to be an object");
            }

            // Check second user
            if let Value::Object(row) = &rows[1] {
                assert_eq!(row.get("id").and_then(|v| v.as_i64()), Some(2));
                assert_eq!(row.get("name").and_then(|v| v.as_str()), Some("Bob"));
            } else {
                panic!("Expected second row to be an object");
            }
        } else {
            panic!("Expected result to be an array");
        }
    });
}

#[tokio::test]
async fn test_dynamic_route_service_with_sql() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE orders (id INTEGER PRIMARY KEY, total REAL)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO orders (id, total) VALUES (42, 99.50)")
        .execute(&pool)
        .await
        .unwrap();

    let data_dir = format!("target/test_data_sql_{}", uuid::Uuid::new_v4());
    let mut service = worpen_core::services::DynamicRouteService::with_data_dir(data_dir);
    service.set_db_pool(pool);

    let route = proto::models::RouteDefinition {
        id: "refund_lookup".to_string(),
        name: "refund_lookup".to_string(),
        description: "Lookup order total".to_string(),
        path: "/api/orders/lookup".to_string(),
        method: proto::models::HttpMethod::POST,
        route_type: proto::models::RouteType::Http,
        parameters: vec![],
        response_schema: None,
        auth_required: false,
        rate_limit: None,
        enabled: true,
        version: "1.0.0".to_string(),
        created_at: "2026-11-14T00:00:00Z".to_string(),
        updated_at: "2026-11-14T00:00:00Z".to_string(),
        created_by: "system".to_string(),
        ws_hooks: None,
        sandbox: Some(proto::models::SandboxPolicy {
            allow_sql: true,
            ..Default::default()
        }),
        logic: vec![
            LogicOperation::Set {
                var: "order_id".to_string(),
                value: serde_json::json!(42),
            },
            LogicOperation::SqlOp {
                query: "SELECT total FROM orders WHERE id = ?".to_string(),
                args: vec![serde_json::json!("{{order_id}}")],
                output_var: "order_rows".to_string(),
            },
            LogicOperation::Return {
                value: serde_json::json!("{{order_rows}}"),
                status: None,
                headers: None,
                raw: None,
            },
        ],
    };

    service.register_route(route).await.unwrap();
    let res = service
        .execute_route(
            "refund_lookup",
            None,
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        )
        .await
        .unwrap();

    assert!(res.is_array());
    let arr = res.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["total"], 99.5);
}
