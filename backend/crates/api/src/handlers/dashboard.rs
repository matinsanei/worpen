use axum::{
    extract::State,
    Json,
};
use crate::state::AppState;
use serde_json::Value;

#[utoipa::path(
    get,
    path = "/api/v1/dashboard/stats",
    responses(
        (status = 200, description = "Get dashboard statistics", body = Value)
    )
)]
pub async fn get_stats(State(state): State<AppState>) -> Json<Value> {
    match state.dashboard_service.get_stats().await {
        Ok(stats) => Json(stats),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/dashboard/network",
    responses(
        (status = 200, description = "Get network traffic data", body = Value)
    )
)]
pub async fn get_network_stats(State(_state): State<AppState>) -> Json<Value> {
    // Mock network data
    Json(serde_json::json!({
        "inbound": [120, 150, 180, 200, 220, 250],
        "outbound": [80, 90, 110, 130, 140, 160],
        "timestamps": ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"]
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/dashboard/services",
    responses(
        (status = 200, description = "Get service health matrix", body = Value)
    )
)]
pub async fn get_services_health(State(_state): State<AppState>) -> Json<Value> {
    // Mock service health data
    Json(serde_json::json!([
        {"name": "api-gateway", "status": "HEALTHY", "latency": 45, "load": 60},
        {"name": "auth-service", "status": "HEALTHY", "latency": 30, "load": 40},
        {"name": "database", "status": "HEALTHY", "latency": 15, "load": 75},
        {"name": "cache", "status": "WARNING", "latency": 80, "load": 85}
    ]))
}

#[utoipa::path(
    get,
    path = "/api/v1/dashboard/load",
    responses(
        (status = 200, description = "Get cluster load distribution", body = Value)
    )
)]
pub async fn get_load_distribution(State(_state): State<AppState>) -> Json<Value> {
    // Mock load distribution data
    Json(serde_json::json!({
        "predicted": [40, 50, 55, 60, 65, 70],
        "actual": [38, 52, 57, 58, 68, 72],
        "timestamps": ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"]
    }))
}
