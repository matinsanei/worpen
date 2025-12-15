use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
};
use crate::state::AppState;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WsParams {
    agent_id: uuid::Uuid,
}

/// WebSocket handler for agent connection
/// 
/// Agents connect here to receive real-time commands.
/// Usage: `ws://localhost:3000/ws?agent_id=<uuid>`
#[utoipa::path(
    get,
    path = "/ws",
    params(
        ("agent_id" = uuid::Uuid, Query, description = "Agent ID")
    ),
    responses(
        (status = 101, description = "Switching Protocols")
    )
)]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<WsParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, params.agent_id))
}

async fn handle_socket(socket: WebSocket, state: AppState, agent_id: uuid::Uuid) {
    let (mut sender, mut receiver) = socket.split();

    // Create a channel for this agent
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    // Save sender to state
    state.connected_agents.insert(agent_id, tx);
    tracing::info!("Agent {} connected via WebSocket", agent_id);

    // Task to forward messages from channel to socket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Task to receive messages from socket
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // Log debug
            tracing::debug!("Received ws msg from {}: {:?}", agent_id, msg);
            // In a real system, we might handle ACKs or responses here
        }
    });

    // Wait for either to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Cleanup
    state.connected_agents.remove(&agent_id);
    tracing::info!("Agent {} disconnected", agent_id);
}
