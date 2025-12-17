use axum::{
    extract::State,
    Json,
};
use crate::state::AppState;
use proto::models::Pipeline;
use serde_json::Value;

#[utoipa::path(
    get,
    path = "/api/v1/pipelines",
    responses(
        (status = 200, description = "List pipelines", body = Vec<Pipeline>)
    )
)]
pub async fn list_pipelines(State(state): State<AppState>) -> Json<Vec<Pipeline>> {
    match state.pipeline_service.list_pipelines().await {
        Ok(pipelines) => Json(pipelines),
        Err(_) => Json(vec![]),
    }
}

pub async fn trigger_pipeline(
    State(_state): State<AppState>,
    Json(_payload): Json<Value>,
) -> Json<Value> {
    // Mock trigger
    Json(serde_json::json!({"status": "TRIGGERED", "pipelineId": "pipe-new-001"}))
}
