use axum::{
    extract::{State, Path},
    Json,
};
use crate::state::AppState;
use proto::models::Incident;
use serde_json::Value;

#[utoipa::path(
    get,
    path = "/api/v1/incidents",
    responses(
        (status = 200, description = "List incidents", body = Vec<Incident>)
    )
)]
pub async fn list_incidents(State(state): State<AppState>) -> Json<Vec<Incident>> {
    match state.incident_service.list_incidents().await {
        Ok(incidents) => Json(incidents),
        Err(_) => Json(vec![]),
    }
}

pub async fn resolve_incident(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let method = payload["method"].as_str().unwrap_or("MANUAL").to_string();
    let notes = payload["notes"].as_str().unwrap_or("").to_string();
    
    match state.incident_service.resolve_incident(id, method, notes).await {
        Ok(_) => Json(serde_json::json!({"status": "RESOLVED"})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}
