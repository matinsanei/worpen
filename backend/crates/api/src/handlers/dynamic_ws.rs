use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use worpen_core::websocket::WebSocketManager;
use worpen_core::vm::memory::ExecutionMemory;
use worpen_core::vm::machine::VirtualMachine;
use worpen_core::compiler::lowerer::LogicCompiler;
use futures::{sink::SinkExt, stream::StreamExt};
use proto::models::RouteType;
use serde_json::Value;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::state::AppState;

/// Handle WebSocket upgrade for dynamic routes
pub async fn dynamic_ws_handler(
    ws: WebSocketUpgrade,
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Find the WebSocket route
    let routes = match state.dynamic_route_service.list_routes().await {
        Ok(routes) => routes,
        Err(_) => {
            return ws.on_upgrade(|mut socket| async move {
                let _ = socket
                    .send(Message::Text("Failed to list routes".to_string()))
                    .await;
                let _ = socket.close().await;
            });
        }
    };
    
    // Find matching WebSocket route
    let mut found_route: Option<proto::models::RouteDefinition> = None;
    for route in routes {
        if route.route_type == RouteType::WebSocket 
            && route.enabled 
            && path.starts_with(route.path.trim_start_matches("/api/")) 
        {
            found_route = Some(route);
            break;
        }
    }

    if let Some(route) = found_route {
        ws.on_upgrade(move |socket| handle_websocket(socket, route, state))
    } else {
        // Return 404-like response
        ws.on_upgrade(|mut socket| async move {
            let _ = socket
                .send(Message::Text("WebSocket route not found".to_string()))
                .await;
            let _ = socket.close().await;
        })
    }
}

/// Handle individual WebSocket connection
async fn handle_websocket(
    socket: WebSocket,
    route: proto::models::RouteDefinition,
    state: AppState,
) {
    let (mut sender, mut receiver) = socket.split();
    let connection_id = Uuid::new_v4().to_string();

    // Create WebSocket manager if not exists
    let ws_manager = state
        .dynamic_route_service
        .get_ws_manager()
        .unwrap_or_else(WebSocketManager::new);

    // Create channel for outgoing messages
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Register connection
    ws_manager.register(connection_id.clone(), tx);

    // Spawn task to forward messages from channel to WebSocket
    let ws_manager_clone = ws_manager.clone();
    let connection_id_clone = connection_id.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
        // Cleanup on disconnect
        ws_manager_clone.unregister(&connection_id_clone);
    });

    // Execute on_connect hook if exists
    if let Some(ref hooks) = route.ws_hooks {
        if !hooks.on_connect.is_empty() {
            let _ = execute_ws_logic(
                &hooks.on_connect,
                &state,
                &ws_manager,
                &connection_id,
                None,
            )
            .await;
        }
    }

    // Main message loop
    while let Some(msg_result) = receiver.next().await {
        match msg_result {
            Ok(Message::Text(text)) => {
                // Execute on_message hook
                if let Some(ref hooks) = route.ws_hooks {
                    if !hooks.on_message.is_empty() {
                        let _ = execute_ws_logic(
                            &hooks.on_message,
                            &state,
                            &ws_manager,
                            &connection_id,
                            Some(text),
                        )
                        .await;
                    }
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Err(_) => {
                break;
            }
            _ => {}
        }
    }

    // Execute on_disconnect hook
    if let Some(ref hooks) = route.ws_hooks {
        if !hooks.on_disconnect.is_empty() {
            let _ = execute_ws_logic(
                &hooks.on_disconnect,
                &state,
                &ws_manager,
                &connection_id,
                None,
            )
            .await;
        }
    }

    // Unregister connection
    ws_manager.unregister(&connection_id);
}

/// Execute WebSocket logic operations
async fn execute_ws_logic(
    logic: &[proto::models::LogicOperation],
    state: &AppState,
    ws_manager: &WebSocketManager,
    connection_id: &str,
    incoming_message: Option<String>,
) -> Result<Value, String> {
    // Compile logic
    let mut compiler = LogicCompiler::new();
    let program = compiler.compile(logic);
    let symbol_table = compiler.get_symbol_table().clone();

    // Create execution memory
    let mut memory = ExecutionMemory::new();

    // Inject {{message}} variable if provided
    if let Some(msg) = incoming_message {
        if let Some(msg_index) = symbol_table.get_index("message") {
            memory.set(msg_index, Value::String(msg));
        }
    }

    // Inject {{connection_id}} variable
    if let Some(conn_index) = symbol_table.get_index("connection_id") {
        memory.set(conn_index, Value::String(connection_id.to_string()));
    }

    // Create VM with WebSocket support
    let db_pool = state.dynamic_route_service.get_db_pool();
    let redis_pool = state.dynamic_route_service.get_redis_pool();

    let mut vm = VirtualMachine::with_all(
        memory,
        symbol_table,
        db_pool,
        redis_pool,
        Some(ws_manager.clone()),
        Some(connection_id.to_string()),
    );

    // Execute
    vm.execute(&program).await
}
