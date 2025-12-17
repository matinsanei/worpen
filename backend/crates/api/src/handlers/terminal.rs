use axum::{
    extract::{Path, State, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::Response,
};
use crate::state::AppState;
use futures::{sink::SinkExt, stream::StreamExt};

/// WebSocket endpoint for interactive container terminal
pub async fn container_terminal_ws(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
    Path(container_id): Path<String>,
) -> Response {
    ws.on_upgrade(move |socket| handle_container_terminal(socket, container_id))
}

async fn handle_container_terminal(socket: WebSocket, container_id: String) {
    let (mut sender, mut receiver) = socket.split();
    
    let session_id = uuid::Uuid::new_v4().to_string();
    
    // Send welcome message
    let welcome = serde_json::json!({
        "type": "connected",
        "session_id": session_id,
        "container_id": container_id,
        "message": format!("Connected to container: {}", container_id)
    });
    
    if sender.send(Message::Text(welcome.to_string())).await.is_err() {
        return;
    }
    
    // Send mock prompt
    let _ = sender.send(Message::Text(serde_json::json!({
        "type": "output",
        "data": format!("root@{}:/# ", container_id)
    }).to_string())).await;
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    // Echo command
                    let _ = sender.send(Message::Text(serde_json::json!({
                        "type": "echo",
                        "data": text.clone()
                    }).to_string())).await;
                    
                    // Mock command execution
                    let response = match text.trim() {
                        "ls" => "bin  boot  dev  etc  home  lib  media  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var",
                        "pwd" => "/root",
                        "whoami" => "root",
                        "exit" => {
                            let _ = sender.send(Message::Text(serde_json::json!({
                                "type": "disconnected",
                                "message": "Session terminated"
                            }).to_string())).await;
                            break;
                        },
                        cmd if cmd.starts_with("echo ") => &cmd[5..],
                        _ => &format!("bash: {}: command not found", text.trim())
                    };
                    
                    // Send response
                    let _ = sender.send(Message::Text(serde_json::json!({
                        "type": "output",
                        "data": format!("{}\nroot@{}:/# ", response, container_id)
                    }).to_string())).await;
                },
                Message::Close(_) => break,
                _ => {}
            }
        }
    }
}

/// WebSocket endpoint for interactive agent terminal
pub async fn agent_terminal_ws(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
    Path(agent_id): Path<String>,
) -> Response {
    ws.on_upgrade(move |socket| handle_agent_terminal(socket, agent_id))
}

async fn handle_agent_terminal(socket: WebSocket, agent_id: String) {
    let (mut sender, mut receiver) = socket.split();
    
    let session_id = uuid::Uuid::new_v4().to_string();
    
    // Send welcome message
    let welcome = serde_json::json!({
        "type": "connected",
        "session_id": session_id,
        "agent_id": agent_id,
        "message": format!("Connected to agent: {}", agent_id)
    });
    
    if sender.send(Message::Text(welcome.to_string())).await.is_err() {
        return;
    }
    
    // Send mock prompt
    let _ = sender.send(Message::Text(serde_json::json!({
        "type": "output",
        "data": format!("worpen@agent-{}:~$ ", agent_id)
    }).to_string())).await;
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    // Echo command
                    let _ = sender.send(Message::Text(serde_json::json!({
                        "type": "echo",
                        "data": text.clone()
                    }).to_string())).await;
                    
                    // Mock command execution
                    let response = match text.trim() {
                        "ls" => "Desktop  Documents  Downloads  Pictures  Videos  worpen",
                        "pwd" => "/home/worpen",
                        "whoami" => "worpen",
                        "hostname" => &format!("agent-{}", agent_id),
                        "docker ps" => "CONTAINER ID   IMAGE          COMMAND       STATUS       PORTS\nabc123def456   nginx:latest   \"nginx\"       Up 2 hours   0.0.0.0:80->80/tcp",
                        "uptime" => "15 days, 7 hours, 32 minutes",
                        "free -h" => "              total        used        free\nMem:           16Gi        9.6Gi       6.4Gi\nSwap:          8Gi         512Mi       7.5Gi",
                        "exit" => {
                            let _ = sender.send(Message::Text(serde_json::json!({
                                "type": "disconnected",
                                "message": "Session terminated"
                            }).to_string())).await;
                            break;
                        },
                        cmd if cmd.starts_with("echo ") => &cmd[5..],
                        _ => &format!("bash: {}: command not found", text.trim())
                    };
                    
                    // Send response
                    let _ = sender.send(Message::Text(serde_json::json!({
                        "type": "output",
                        "data": format!("{}\nworpen@agent-{}:~$ ", response, agent_id)
                    }).to_string())).await;
                },
                Message::Close(_) => break,
                _ => {}
            }
        }
    }
}

/// List all active terminal sessions
pub async fn list_terminal_sessions(
    State(_state): State<AppState>,
) -> axum::Json<serde_json::Value> {
    // Mock active sessions
    axum::Json(serde_json::json!({
        "sessions": [
            {
                "session_id": "sess-001",
                "type": "container",
                "target_id": "container-abc123",
                "started_at": "2025-12-17T12:30:00Z",
                "last_activity": "2025-12-17T12:45:00Z",
                "is_active": true
            },
            {
                "session_id": "sess-002",
                "type": "agent",
                "target_id": "agent-xyz789",
                "started_at": "2025-12-17T12:35:00Z",
                "last_activity": "2025-12-17T12:46:00Z",
                "is_active": true
            }
        ]
    }))
}
