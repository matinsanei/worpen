// WebSocket VM operation tests
use worpen_core::websocket::WebSocketManager;
use worpen_core::vm::memory::ExecutionMemory;
use worpen_core::vm::machine::VirtualMachine;
use worpen_core::compiler::lowerer::LogicCompiler;
use proto::models::{LogicOperation, RouteDefinition, WebSocketHooks, RouteType, HttpMethod};
use serde_json::json;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_websocket_send_operation() {
    // Create WebSocket manager
    let ws_manager = WebSocketManager::new();
    let connection_id = "test-conn-1".to_string();
    
    // Create channel for receiving messages
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    // Register connection
    ws_manager.register(connection_id.clone(), tx);
    
    // Create test logic with WsOp
    let logic = vec![
        LogicOperation::WsOp {
            command: "send".to_string(),
            message: "Hello from VM!".to_string(),
            channel: None,
        }
    ];
    
    // Compile and execute
    let mut compiler = LogicCompiler::new();
    let program = compiler.compile(&logic);
    let symbol_table = compiler.get_symbol_table().clone();
    
    let memory = ExecutionMemory::new();
    let mut vm = VirtualMachine::with_websocket(
        memory,
        symbol_table,
        ws_manager.clone(),
        connection_id.clone(),
    );
    
    // Execute in background
    tokio::spawn(async move {
        let result = vm.execute(&program).await;
        assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
    });
    
    // Receive message
    if let Some(msg) = rx.recv().await {
        assert_eq!(msg, "Hello from VM!");
    } else {
        panic!("No message received");
    }
    
    ws_manager.unregister(&connection_id);
}

#[tokio::test]
async fn test_websocket_broadcast_operation() {
    // Create WebSocket manager
    let ws_manager = WebSocketManager::new();
    
    // Register multiple connections
    let (tx1, mut rx1) = mpsc::unbounded_channel();
    let (tx2, mut rx2) = mpsc::unbounded_channel();
    
    ws_manager.register("conn1".to_string(), tx1);
    ws_manager.register("conn2".to_string(), tx2);
    
    // Create test logic with broadcast
    let logic = vec![
        LogicOperation::WsOp {
            command: "broadcast".to_string(),
            message: "Broadcast message".to_string(),
            channel: None,
        }
    ];
    
    // Compile and execute
    let mut compiler = LogicCompiler::new();
    let program = compiler.compile(&logic);
    let symbol_table = compiler.get_symbol_table().clone();
    
    let memory = ExecutionMemory::new();
    let mut vm = VirtualMachine::with_websocket(
        memory,
        symbol_table,
        ws_manager.clone(),
        "conn1".to_string(),
    );
    
    // Execute
    let result = vm.execute(&program).await;
    assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
    
    // Both connections should receive the message
    let msg1 = rx1.recv().await;
    let msg2 = rx2.recv().await;
    
    assert_eq!(msg1, Some("Broadcast message".to_string()));
    assert_eq!(msg2, Some("Broadcast message".to_string()));
    
    ws_manager.unregister("conn1");
    ws_manager.unregister("conn2");
}

#[tokio::test]
async fn test_websocket_variable_interpolation() {
    // Create WebSocket manager
    let ws_manager = WebSocketManager::new();
    let connection_id = "test-conn-var".to_string();
    
    let (tx, mut rx) = mpsc::unbounded_channel();
    ws_manager.register(connection_id.clone(), tx);
    
    // Create logic that uses variables
    let logic = vec![
        LogicOperation::Set {
            var: "username".to_string(),
            value: json!("Alice"),
        },
        LogicOperation::Set {
            var: "score".to_string(),
            value: json!(100),
        },
        LogicOperation::WsOp {
            command: "send".to_string(),
            message: "User {{username}} scored {{score}} points!".to_string(),
            channel: None,
        }
    ];
    
    // Compile and execute
    let mut compiler = LogicCompiler::new();
    let program = compiler.compile(&logic);
    let symbol_table = compiler.get_symbol_table().clone();
    
    let memory = ExecutionMemory::new();
    let mut vm = VirtualMachine::with_websocket(
        memory,
        symbol_table,
        ws_manager.clone(),
        connection_id.clone(),
    );
    
    let result = vm.execute(&program).await;
    assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
    
    // Check received message
    if let Some(msg) = rx.recv().await {
        assert!(msg.contains("Alice"));
        assert!(msg.contains("100"));
    } else {
        panic!("No message received");
    }
    
    ws_manager.unregister(&connection_id);
}

#[tokio::test]
async fn test_websocket_channel_broadcast() {
    // Create WebSocket manager
    let ws_manager = WebSocketManager::new();
    
    // Register connections in different channels
    let (tx1, mut rx1) = mpsc::unbounded_channel();
    let (tx2, mut rx2) = mpsc::unbounded_channel();
    let (tx3, mut rx3) = mpsc::unbounded_channel();
    
    ws_manager.register("conn1".to_string(), tx1);
    ws_manager.register("conn2".to_string(), tx2);
    ws_manager.register("conn3".to_string(), tx3);
    
    // Subscribe conn1 and conn2 to "gameroom"
    ws_manager.subscribe("conn1".to_string(), "gameroom".to_string());
    ws_manager.subscribe("conn2".to_string(), "gameroom".to_string());
    // conn3 not subscribed
    
    // Broadcast to channel
    let logic = vec![
        LogicOperation::WsOp {
            command: "broadcast".to_string(),
            message: "Game starting!".to_string(),
            channel: Some("gameroom".to_string()),
        }
    ];
    
    // Compile and execute
    let mut compiler = LogicCompiler::new();
    let program = compiler.compile(&logic);
    let symbol_table = compiler.get_symbol_table().clone();
    
    let memory = ExecutionMemory::new();
    let mut vm = VirtualMachine::with_websocket(
        memory,
        symbol_table,
        ws_manager.clone(),
        "conn1".to_string(),
    );
    
    let result = vm.execute(&program).await;
    assert!(result.is_ok());
    
    // conn1 and conn2 should receive, conn3 should not
    assert_eq!(rx1.recv().await, Some("Game starting!".to_string()));
    assert_eq!(rx2.recv().await, Some("Game starting!".to_string()));
    
    // rx3 should timeout (no message)
    let rx3_result = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        rx3.recv()
    ).await;
    assert!(rx3_result.is_err(), "conn3 should not receive message");
    
    ws_manager.unregister("conn1");
    ws_manager.unregister("conn2");
    ws_manager.unregister("conn3");
}

/// Test a complete WebSocket route with hooks
#[tokio::test]
async fn test_websocket_route_with_hooks() {
    let route = RouteDefinition {
        id: "ws-test-1".to_string(),
        name: "Test WebSocket Chat".to_string(),
        description: "Test WS route with hooks".to_string(),
        path: "/ws/chat".to_string(),
        method: HttpMethod::GET,
        route_type: RouteType::WebSocket,
        logic: vec![],
        ws_hooks: Some(WebSocketHooks {
            on_connect: vec![
                LogicOperation::WsOp {
                    command: "send".to_string(),
                    message: "Welcome to chat!".to_string(),
                    channel: None,
                }
            ],
            on_message: vec![
                LogicOperation::Set {
                    var: "response".to_string(),
                    value: json!("Echo: {{message}}"),
                },
                LogicOperation::WsOp {
                    command: "send".to_string(),
                    message: "{{response}}".to_string(),
                    channel: None,
                }
            ],
            on_disconnect: vec![
                LogicOperation::WsOp {
                    command: "broadcast".to_string(),
                    message: "User disconnected".to_string(),
                    channel: None,
                }
            ],
        }),
        parameters: vec![],
        response_schema: None,
        auth_required: false,
        rate_limit: None,
        enabled: true,
        version: "1.0.0".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        created_by: "test".to_string(),
    };
    
    assert_eq!(route.route_type, RouteType::WebSocket);
    assert!(route.ws_hooks.is_some());
    
    let hooks = route.ws_hooks.unwrap();
    assert!(!hooks.on_connect.is_empty());
    assert!(!hooks.on_message.is_empty());
    assert!(!hooks.on_disconnect.is_empty());
}


