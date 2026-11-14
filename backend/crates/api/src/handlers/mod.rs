use crate::dtos::RegisterAgentRequest;
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};

#[allow(dead_code, unused_imports)]
pub mod automation;
#[allow(dead_code, unused_imports)]
pub mod dashboard;
#[allow(dead_code, unused_imports)]
pub mod docker;
pub mod dynamic_fallback;
pub mod dynamic_routes;
pub mod dynamic_routes_yaml;
#[allow(dead_code, unused_imports)]
pub mod dynamic_ws;
#[allow(dead_code, unused_imports)]
pub mod fleet;
#[allow(dead_code, unused_imports)]
pub mod incident;
#[allow(dead_code, unused_imports)]
pub mod logs;
pub mod pipelines;
#[allow(dead_code, unused_imports)]
pub mod terminal;
#[allow(dead_code, unused_imports)]
pub mod ws;

#[allow(unused_imports)]
pub use automation::*;
#[allow(unused_imports)]
pub use dashboard::*;
#[allow(unused_imports)]
pub use docker::*;
pub use dynamic_fallback::*;
pub use dynamic_routes::*;
pub use dynamic_routes_yaml::*;
#[allow(unused_imports)]
pub use dynamic_ws::*;
#[allow(unused_imports)]
pub use fleet::*;
#[allow(unused_imports)]
pub use incident::*;
#[allow(unused_imports)]
pub use logs::*;
pub use pipelines::*;
#[allow(unused_imports)]
pub use terminal::*;
#[allow(unused_imports)]
pub use ws::ws_handler;

/// Register a new agent in the Hive
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/agents/register",
    request_body = RegisterAgentRequest,
    responses(
        (status = 201, description = "Agent successfully registered", body = serde_json::Value),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn register_agent(
    State(state): State<AppState>,
    Json(payload): Json<RegisterAgentRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    match state
        .agent_service
        .register_agent(payload.hostname, payload.ip_address, payload.version)
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })), // Ensure e is converted to string or json safe
        ),
    }
}

use crate::dtos::TelemetryRequest;

/// Receive telemetry data from an agent
#[allow(dead_code)]
#[utoipa::path(
    post,
    path = "/telemetry",
    request_body = TelemetryRequest,
    responses(
        (status = 200, description = "Telemetry processed successfully"),
        (status = 404, description = "Agent not found")
    )
)]
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

    match state
        .agent_service
        .process_heartbeat(payload.agent_id)
        .await
    {
        Ok(_) => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    }
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "System is healthy")
    )
)]
pub async fn health_check() -> &'static str {
    "OK"
}
