use core::compiler::lowerer::LogicCompiler;
use core::vm::machine::VirtualMachine;
use core::vm::memory::ExecutionMemory;
use proto::models::LogicOperation;
use serde_json::Value;
use tokio::runtime::Runtime;
use sqlx::SqlitePool;

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
            LogicOperation::Set { var: "user_id".to_string(), value: Value::Number(1.into()) },
            LogicOperation::Set { var: "user_name".to_string(), value: Value::String("Alice".to_string()) },
            LogicOperation::Set { var: "user_age".to_string(), value: Value::Number(30.into()) },
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
            LogicOperation::Set { var: "search_id".to_string(), value: Value::Number(1.into()) },
            LogicOperation::SqlOp { 
                query: "SELECT id, name, age FROM users WHERE id = ?".to_string(),
                args: vec![
                    Value::String("{{search_id}}".to_string()),
                ],
                output_var: "query_result".to_string(),
            },
            LogicOperation::Return { value: Value::String("{{query_result}}".to_string()) },
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
            LogicOperation::Set { var: "user2_id".to_string(), value: Value::Number(2.into()) },
            LogicOperation::Set { var: "user2_name".to_string(), value: Value::String("Bob".to_string()) },
            LogicOperation::Set { var: "user2_age".to_string(), value: Value::Number(25.into()) },
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
            LogicOperation::Return { value: Value::String("{{all_users}}".to_string()) },
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
