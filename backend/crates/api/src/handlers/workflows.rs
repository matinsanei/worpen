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
    headers: axum::http::HeaderMap,
    Path(name): Path<String>,
    Query(params): Query<ExecuteParams>,
    payload: Option<Json<Value>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mode = if params.dry_run.unwrap_or(false) {
        ExecutionMode::DryRun
    } else {
        ExecutionMode::Live
    };

    let initiator = headers
        .get("x-worpen-initiator")
        .or_else(|| headers.get("x-initiator"))
        .or_else(|| headers.get("x-caller-id"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| {
            if crate::auth::require_token(&headers).is_ok()
                && crate::auth::configured_token().is_some()
            {
                Some("authenticated_operator".to_string())
            } else {
                None
            }
        });

    let ikey = headers
        .get("idempotency-key")
        .or_else(|| headers.get("x-idempotency-key"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let body = payload.map(|p| p.0);
    match state
        .workflow_service
        .execute_with_meta(&name, body, mode, initiator, ikey)
        .await
    {
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

#[derive(Deserialize)]
pub struct ApprovalActionPayload {
    pub approver: String,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Deserialize)]
pub struct RejectActionPayload {
    pub rejecter: String,
    #[serde(default)]
    pub reason: Option<String>,
}

/// GET /api/v1/runs — List recent durable runs
pub async fn list_runs(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<proto::models::WorkflowRun>>, (StatusCode, Json<Value>)> {
    crate::auth::require_token(&headers)?;
    match state.workflow_service.list_runs(50).await {
        Ok(runs) => {
            let redaction = state.dynamic_route_service.secret_store().redaction();
            let redacted_runs: Vec<proto::models::WorkflowRun> = runs
                .into_iter()
                .map(|mut r| {
                    if let Some(res) = r.result {
                        r.result = Some(redaction.redact_value(&res));
                    }
                    let mut redacted_vars = HashMap::new();
                    for (k, v) in r.variables {
                        redacted_vars.insert(k, redaction.redact_value(&v));
                    }
                    r.variables = redacted_vars;
                    r
                })
                .collect();
            Ok(Json(redacted_runs))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "list_runs_failed", "message": e })),
        )),
    }
}

/// GET /api/v1/runs/pending — List all runs waiting for human approval
pub async fn list_pending_approvals(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<proto::models::WorkflowRun>>, (StatusCode, Json<Value>)> {
    crate::auth::require_token(&headers)?;
    match state.workflow_service.list_pending_approvals().await {
        Ok(runs) => Ok(Json(runs)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "list_pending_failed", "message": e })),
        )),
    }
}

/// GET /api/v1/runs/:id — Get run status by ID
pub async fn get_run(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<proto::models::WorkflowRun>, (StatusCode, Json<Value>)> {
    crate::auth::require_token(&headers)?;
    match state.workflow_service.get_run(&id).await {
        Ok(Some(mut run)) => {
            let redaction = state.dynamic_route_service.secret_store().redaction();
            if let Some(res) = run.result {
                run.result = Some(redaction.redact_value(&res));
            }
            let mut redacted_vars = HashMap::new();
            for (k, v) in run.variables {
                redacted_vars.insert(k, redaction.redact_value(&v));
            }
            run.variables = redacted_vars;
            Ok(Json(run))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "not_found", "message": format!("Run '{}' not found", id) })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "get_run_failed", "message": e })),
        )),
    }
}

/// POST /api/v1/runs/:id/approve — Operator sign-off on an approval gate
pub async fn approve_run(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<ApprovalActionPayload>,
) -> Result<Json<proto::models::WorkflowRun>, (StatusCode, Json<Value>)> {
    crate::auth::require_token(&headers)?;
    let reason = payload
        .reason
        .unwrap_or_else(|| "Approved via API".to_string());
    match state
        .workflow_service
        .approve_run(&id, &payload.approver, &reason)
        .await
    {
        Ok(run) => Ok(Json(run)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "approval_failed", "message": e })),
        )),
    }
}

/// POST /api/v1/runs/:id/reject — Operator rejection of an approval gate
pub async fn reject_run(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<RejectActionPayload>,
) -> Result<Json<proto::models::WorkflowRun>, (StatusCode, Json<Value>)> {
    crate::auth::require_token(&headers)?;
    let reason = payload
        .reason
        .unwrap_or_else(|| "Rejected via API".to_string());
    match state
        .workflow_service
        .reject_run(&id, &payload.rejecter, &reason)
        .await
    {
        Ok(run) => Ok(Json(run)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "rejection_failed", "message": e })),
        )),
    }
}

/// POST /api/v1/runs/:id/cancel — Cancel a run
pub async fn cancel_run(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<proto::models::WorkflowRun>, (StatusCode, Json<Value>)> {
    crate::auth::require_token(&headers)?;
    match state.workflow_service.cancel_run(&id).await {
        Ok(run) => Ok(Json(run)),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "cancel_failed", "message": e })),
        )),
    }
}
