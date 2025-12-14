use axum::{extract::State, Json, http::StatusCode};
use crate::state::AppState;
use crate::dtos::RegisterAgentRequest;

pub async fn register_agent(
    State(state): State<AppState>,
    Json(payload): Json<RegisterAgentRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    match state.agent_service.register_agent(payload.hostname, payload.ip_address, payload.version).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(serde_json::json!({ "error": e }))
        ),
    }
}

use crate::dtos::TelemetryRequest;

pub async fn receive_telemetry(
    State(state): State<AppState>,
    Json(payload): Json<TelemetryRequest>,
) -> StatusCode {
    tracing::info!(
        "Telemetry from {}: CPU: {:.2}%, MEM: {}/{} bytes", 
        payload.agent_id, 
        payload.cpu_usage, 
        payload.memory_usage, 
        payload.total_memory
    );

    match state.agent_service.process_heartbeat(payload.agent_id).await {
        Ok(_) => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    }
}

