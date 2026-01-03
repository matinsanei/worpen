use worpen_core::compiler::lowerer::LogicCompiler;
use worpen_core::vm::machine::VirtualMachine;
use worpen_core::vm::memory::ExecutionMemory;
use proto::models::LogicOperation;
use serde_json::{json, Value};
use tokio::runtime::Runtime;

#[test]
fn test_redis_set_and_get() {
    // Skip if Redis not available
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    
    // Create connection pool
    let redis_pool = match deadpool_redis::Pool::builder(
        deadpool_redis::Manager::new(redis_url.as_str()).expect("Failed to create Redis manager")
    ).build() {
        Ok(pool) => pool,
        Err(_) => {
            println!("Failed to create Redis pool, skipping test");
            return;
        }
    };
    
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Test Redis SET and GET
        let logic = vec![
            LogicOperation::RedisOp {
                command: "SET".to_string(),
                key: "test_key_simple".to_string(),
                value: Some("test_value_123".to_string()),
                ttl_seconds: None,
                output_var: Some("status".to_string()),
            },
            LogicOperation::RedisOp {
                command: "GET".to_string(),
                key: "test_key_simple".to_string(),
                value: None,
                ttl_seconds: None,
                output_var: Some("result".to_string()),
            },
            LogicOperation::Return { value: Value::String("{{result}}".to_string()) },
        ];
        
        // Compile and execute
        let mut compiler = LogicCompiler::new();
        let optimized = compiler.compile(&logic);
        let symbol_table = compiler.get_symbol_table();
        
        let memory = ExecutionMemory::new();
        let mut vm = VirtualMachine::with_redis_pool(memory, symbol_table.clone(), redis_pool.clone());
        
        let result = vm.execute(&optimized).await;
        match result {
            Ok(val) => assert_eq!(val, Value::String("test_value_123".to_string())),
            Err(e) if e.contains("Failed to get Redis connection") => {
                println!("Redis not available: {}", e);
            },
            Err(e) => panic!("Unexpected error: {}", e),
        }
    });
}

#[test]
fn test_redis_with_variables() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    
    let redis_pool = match deadpool_redis::Pool::builder(
        deadpool_redis::Manager::new(redis_url.as_str()).expect("Failed to create Redis manager")
    ).build() {
        Ok(pool) => pool,
        Err(_) => {
            println!("Failed to create Redis pool, skipping test");
            return;
        }
    };
    
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Test Redis with variable interpolation
        let logic = vec![
            LogicOperation::Set { var: "user_id".to_string(), value: json!(42) },
            LogicOperation::Set { var: "user_name".to_string(), value: json!("Alice") },
            LogicOperation::RedisOp {
                command: "SET".to_string(),
                key: "user:{{user_id}}:name".to_string(),
                value: Some("{{user_name}}".to_string()),
                ttl_seconds: None,
                output_var: Some("status".to_string()),
            },
            LogicOperation::RedisOp {
                command: "GET".to_string(),
                key: "user:42:name".to_string(),
                value: None,
                ttl_seconds: None,
                output_var: Some("result".to_string()),
            },
            LogicOperation::Return { value: Value::String("{{result}}".to_string()) },
        ];
        
        let mut compiler = LogicCompiler::new();
        let optimized = compiler.compile(&logic);
        let symbol_table = compiler.get_symbol_table();
        
        let memory = ExecutionMemory::new();
        let mut vm = VirtualMachine::with_redis_pool(memory, symbol_table.clone(), redis_pool.clone());
        
        let result = vm.execute(&optimized).await;
        match result {
            Ok(val) => assert_eq!(val, Value::String("Alice".to_string())),
            Err(e) if e.contains("Failed to get Redis connection") => {
                println!("Redis not available: {}", e);
            },
            Err(e) => panic!("Unexpected error: {}", e),
        }
    });
}

#[test]
fn test_redis_incr() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    
    let redis_pool = match deadpool_redis::Pool::builder(
        deadpool_redis::Manager::new(redis_url.as_str()).expect("Failed to create Redis manager")
    ).build() {
        Ok(pool) => pool,
        Err(_) => {
            println!("Failed to create Redis pool, skipping test");
            return;
        }
    };
    
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Test Redis INCR/DECR
        let logic = vec![
            LogicOperation::RedisOp {
                command: "SET".to_string(),
                key: "counter_key".to_string(),
                value: Some("10".to_string()),
                ttl_seconds: None,
                output_var: Some("status".to_string()),
            },
            LogicOperation::RedisOp {
                command: "INCR".to_string(),
                key: "counter_key".to_string(),
                value: None,
                ttl_seconds: None,
                output_var: Some("counter".to_string()),
            },
            LogicOperation::RedisOp {
                command: "INCR".to_string(),
                key: "counter_key".to_string(),
                value: None,
                ttl_seconds: None,
                output_var: Some("counter".to_string()),
            },
            LogicOperation::RedisOp {
                command: "DECR".to_string(),
                key: "counter_key".to_string(),
                value: None,
                ttl_seconds: None,
                output_var: Some("counter".to_string()),
            },
            LogicOperation::Return { value: Value::String("{{counter}}".to_string()) },
        ];
        
        let mut compiler = LogicCompiler::new();
        let optimized = compiler.compile(&logic);
        let symbol_table = compiler.get_symbol_table();
        
        let memory = ExecutionMemory::new();
        let mut vm = VirtualMachine::with_redis_pool(memory, symbol_table.clone(), redis_pool.clone());
        
        let result = vm.execute(&optimized).await;
        match result {
            Ok(val) => assert_eq!(val, json!(11)),
            Err(e) if e.contains("Failed to get Redis connection") => {
                println!("Redis not available: {}", e);
            },
            Err(e) => panic!("Unexpected error: {}", e),
        }
    });
}
