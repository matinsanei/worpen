use axum::{
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use crate::state::AppState;
use proto::models::AutomationRule;
use serde_json::Value;

#[utoipa::path(
    get,
    path = "/api/v1/automation/rules",
    responses(
        (status = 200, description = "List automation rules", body = Vec<AutomationRule>)
    )
)]
pub async fn list_rules(State(state): State<AppState>) -> Json<Vec<AutomationRule>> {
    match state.automation_service.list_rules().await {
        Ok(rules) => Json(rules),
        Err(_) => Json(vec![]),
    }
}

pub async fn create_rule(
    State(state): State<AppState>,
    Json(rule): Json<AutomationRule>,
) -> Json<Value> {
    match state.automation_service.create_rule(rule).await {
        Ok(_) => Json(serde_json::json!({"status": "CREATED"})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/automation/rules/{id}",
    responses(
        (status = 200, description = "Get specific automation rule", body = AutomationRule),
        (status = 404, description = "Rule not found")
    )
)]
pub async fn get_rule(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AutomationRule>, StatusCode> {
    // Mock rule retrieval
    Ok(Json(AutomationRule {
        id: id.clone(),
        name: "Auto Restart on Error".to_string(),
        trigger_event: "container.error".to_string(),
        target_service: "api-gateway".to_string(),
        script: "docker restart $CONTAINER_ID".to_string(),
        last_run: "2025-12-17T10:00:00Z".to_string(),
        active: true,
    }))
}

#[utoipa::path(
    put,
    path = "/api/v1/automation/rules/{id}",
    responses(
        (status = 200, description = "Update automation rule", body = Value)
    )
)]
pub async fn update_rule(
    State(_state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let active = payload["active"].as_bool().unwrap_or(true);
    Json(serde_json::json!({
        "status": "UPDATED",
        "rule_id": id,
        "active": active
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/automation/dry-run",
    responses(
        (status = 200, description = "Dry run result", body = Value)
    )
)]
pub async fn dry_run(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let script = payload["script"].as_str().unwrap_or("");
    Json(serde_json::json!({
        "status": "SUCCESS",
        "script": script,
        "output": "Mock execution successful",
        "execution_time": "120ms"
    }))
}
