use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use proto::models::{ExecutionMode, WorkflowSpecV1};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ExecuteParams {
    #[serde(default)]
    pub dry_run: Option<bool>,
}

#[derive(Deserialize)]
pub struct RollbackParams {
    pub version: String,
}

/// GET /api/v1/workflows — List all registered workflows
pub async fn list_workflows(State(state): State<AppState>) -> Json<Vec<WorkflowSpecV1>> {
    Json(state.workflow_service.list_workflows())
}

/// POST /api/v1/workflows — Register a new WorkflowSpecV1 (YAML or JSON)
pub async fn register_workflow(
    State(state): State<AppState>,
    body: String,
) -> Result<(StatusCode, Json<WorkflowSpecV1>), (StatusCode, Json<Value>)> {
    match state.workflow_service.register_workflow(&body).await {
        Ok(spec) => Ok((StatusCode::CREATED, Json(spec))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "validation_failed", "message": e })),
        )),
    }
}

/// GET /api/v1/workflows/:name — Get a workflow by name
pub async fn get_workflow(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<WorkflowSpecV1>, (StatusCode, Json<Value>)> {
    match state.workflow_service.get_workflow(&name) {
        Some(spec) => Ok(Json(spec)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(
                json!({ "error": "not_found", "message": format!("Workflow '{}' not found", name) }),
            ),
        )),
    }
}

/// POST /api/v1/workflows/:name/execute — Execute a workflow (Live or DryRun)
pub async fn execute_workflow(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(params): Query<ExecuteParams>,
    payload: Option<Json<Value>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mode = if params.dry_run.unwrap_or(false) {
        ExecutionMode::DryRun
    } else {
        ExecutionMode::Live
    };

    let body = payload.map(|p| p.0);
    match state.workflow_service.execute(&name, body, mode).await {
        Ok(val) => Ok(Json(val)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "execution_failed", "message": e })),
        )),
    }
}

/// GET /api/v1/workflows/:name/versions — List immutable versions
pub async fn list_versions(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Json<HashMap<String, Vec<String>>> {
    let versions = state.workflow_service.list_versions(&name);
    let mut map = HashMap::new();
    map.insert("versions".to_string(), versions);
    Json(map)
}

/// POST /api/v1/workflows/:name/rollback — Rollback workflow to specified version
pub async fn rollback_workflow(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(params): Query<RollbackParams>,
) -> Result<Json<WorkflowSpecV1>, (StatusCode, Json<Value>)> {
    match state
        .workflow_service
        .rollback_workflow(&name, &params.version)
        .await
    {
        Ok(spec) => Ok(Json(spec)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "rollback_failed", "message": e })),
        )),
    }
}
