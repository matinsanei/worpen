use crate::auth::require_token;
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use proto::models::{PipelineListItem, PipelineRunResult, RegisterPipelineRequest};
use serde_json::Value;
use std::collections::HashMap;

/// List pipelines with last-run status (UI-facing).
#[utoipa::path(
    get,
    path = "/api/v1/pipelines",
    responses(
        (status = 200, description = "Pipelines with status", body = Vec<PipelineListItem>)
    )
)]
pub async fn list_pipelines(State(state): State<AppState>) -> Json<Vec<PipelineListItem>> {
    Json(state.pipeline_service.list_with_status().await)
}

/// Define a new pipeline (stages are inlined + validated at definition time).
#[utoipa::path(
    post,
    path = "/api/v1/pipelines",
    request_body = RegisterPipelineRequest,
    responses(
        (status = 201, description = "Pipeline defined", body = Value),
        (status = 400, description = "Invalid pipeline definition")
    )
)]
pub async fn define_pipeline(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RegisterPipelineRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    require_token(&headers)?;
    match state.pipeline_service.define_pipeline(req).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({"status": "DEFINED", "pipeline_id": id})),
        )),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )),
    }
}

/// Get one pipeline definition.
pub async fn get_pipeline(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    match state.pipeline_service.get_definition(&id).await {
        Some(def) => Ok(Json(serde_json::to_value(def).unwrap_or(Value::Null))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete a pipeline definition (run history is kept in memory).
pub async fn delete_pipeline(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    if require_token(&headers).is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if state.pipeline_service.delete_pipeline(&id).await {
        Ok(Json(
            serde_json::json!({"status": "DELETED", "pipeline_id": id}),
        ))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// Trigger a pipeline run by id or name. Accepts {pipelineId|pipeline_id|id|name} + optional {payload}.
#[utoipa::path(
    post,
    path = "/api/v1/pipelines/trigger",
    request_body = Value,
    responses(
        (status = 200, description = "Pipeline run result", body = PipelineRunResult),
        (status = 404, description = "Pipeline not found")
    )
)]
pub async fn trigger_pipeline(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<PipelineRunResult>, (StatusCode, Json<Value>)> {
    let id_or_name = payload
        .get("pipelineId")
        .or_else(|| payload.get("pipeline_id"))
        .or_else(|| payload.get("id"))
        .or_else(|| payload.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if id_or_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "pipelineId (or name) is required"})),
        ));
    }
    // Auth-gated pipelines require a token when one is configured.
    let auth_required = match state.pipeline_service.get_definition(&id_or_name).await {
        Some(d) => d.auth_required,
        None => state
            .pipeline_service
            .find_by_name(&id_or_name)
            .await
            .map(|d| d.auth_required)
            .unwrap_or(false),
    };
    if auth_required {
        require_token(&headers)?;
    }
    let run_payload = payload.get("payload").cloned();
    match state
        .pipeline_service
        .run_pipeline(&id_or_name, run_payload)
        .await
    {
        Ok(run) => Ok(Json(run)),
        Err(e) if e.contains("not found") => {
            Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": e}))))
        }
        Err(e) if e.contains("disabled") => {
            Err((StatusCode::FORBIDDEN, Json(serde_json::json!({"error": e}))))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        )),
    }
}

/// Run history, newest first. Optional ?pipeline_id=...&limit=20.
pub async fn pipeline_runs(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Value> {
    let limit: usize = params
        .get("limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20)
        .clamp(1, 100);
    let runs = state
        .pipeline_service
        .run_history(params.get("pipeline_id").map(|s| s.as_str()), limit)
        .await;
    Json(serde_json::json!({"count": runs.len(), "runs": runs}))
}

/// Run history for one pipeline: GET /api/v1/pipelines/:id/runs.
pub async fn pipeline_runs_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Value> {
    let limit: usize = params
        .get("limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20)
        .clamp(1, 100);
    let runs = state.pipeline_service.run_history(Some(&id), limit).await;
    Json(serde_json::json!({"count": runs.len(), "runs": runs}))
}
