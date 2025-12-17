use axum::{
    extract::{State, Path},
    Json,
};
use crate::state::AppState;
use proto::models::Container;
use serde_json::Value;
use sqlx::types::chrono::Utc;

#[utoipa::path(
    get,
    path = "/api/v1/containers",
    responses(
        (status = 200, description = "List containers", body = Vec<Container>)
    )
)]
pub async fn list_containers(State(state): State<AppState>) -> Json<Vec<Container>> {
    match state.docker_service.list_containers().await {
        Ok(containers) => Json(containers),
        Err(_) => Json(vec![]),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/containers/{id}/{action}",
    responses(
        (status = 200, description = "Perform container action", body = Value)
    )
)]
pub async fn container_action(
    State(state): State<AppState>,
    Path((id, action)): Path<(String, String)>,
) -> Json<Value> {
    match state.docker_service.container_action(id, &action).await {
        Ok(msg) => Json(serde_json::json!({"message": msg})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/containers/{id}/logs",
    responses(
        (status = 200, description = "Get container logs", body = Value)
    )
)]
pub async fn get_container_logs(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    // Mock logs
    Json(serde_json::json!({
        "logs": [
            "[2025-12-17 10:00:00] Container started",
            "[2025-12-17 10:01:23] Processing request...",
            "[2025-12-17 10:02:45] Request completed successfully"
        ],
        "container_id": id
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/containers/{id}/start",
    responses(
        (status = 200, description = "Start container", body = Value)
    )
)]
pub async fn start_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    match state.docker_service.container_action(id.clone(), "start").await {
        Ok(msg) => Json(serde_json::json!({"status": "STARTED", "container_id": id, "message": msg})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/containers/{id}/stop",
    responses(
        (status = 200, description = "Stop container", body = Value)
    )
)]
pub async fn stop_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    match state.docker_service.container_action(id.clone(), "stop").await {
        Ok(msg) => Json(serde_json::json!({"status": "STOPPED", "container_id": id, "message": msg})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/containers/{id}/restart",
    responses(
        (status = 200, description = "Restart container", body = Value)
    )
)]
pub async fn restart_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    match state.docker_service.container_action(id.clone(), "restart").await {
        Ok(msg) => Json(serde_json::json!({"status": "RESTARTED", "container_id": id, "message": msg})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/containers/{id}",
    responses(
        (status = 200, description = "Delete container", body = Value)
    )
)]
pub async fn delete_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    // Mock deletion
    Json(serde_json::json!({"status": "DELETED", "container_id": id}))
}

#[utoipa::path(
    post,
    path = "/api/v1/docker/prune",
    responses(
        (status = 200, description = "Prune unused resources", body = Value)
    )
)]
pub async fn prune_system(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let all = payload["all"].as_bool().unwrap_or(false);
    Json(serde_json::json!({
        "status": "PRUNED",
        "reclaimed_space": "1.2GB",
        "all": all
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/images",
    responses(
        (status = 200, description = "List container images", body = Value)
    )
)]
pub async fn list_images(State(_state): State<AppState>) -> Json<Value> {
    // Mock images
    Json(serde_json::json!([
        {"id": "img-001", "repository": "nginx", "tag": "latest", "size": "140MB"},
        {"id": "img-002", "repository": "postgres", "tag": "15", "size": "380MB"},
        {"id": "img-003", "repository": "redis", "tag": "7", "size": "116MB"}
    ]))
}

#[utoipa::path(
    post,
    path = "/api/v1/images/pull",
    responses(
        (status = 200, description = "Pull container image", body = Value)
    )
)]
pub async fn pull_image(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let image = payload["image"].as_str().unwrap_or("unknown");
    Json(serde_json::json!({
        "status": "PULLED",
        "image": image
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/containers/{id}/exec",
    responses(
        (status = 200, description = "Execute command in container", body = proto::models::ExecResponse)
    )
)]
pub async fn exec_in_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<proto::models::ExecRequest>,
) -> Json<proto::models::ExecResponse> {
    // Mock execution
    let output = format!("Executed '{}' in container {}", payload.command, id);
    Json(proto::models::ExecResponse {
        exit_code: 0,
        stdout: output,
        stderr: String::new(),
        execution_time_ms: 125,
    })
}

#[utoipa::path(
    post,
    path = "/api/v1/containers/{id}/shell",
    responses(
        (status = 200, description = "Start interactive shell session", body = proto::models::ShellSession)
    )
)]
pub async fn start_container_shell(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<proto::models::ShellSession> {
    use uuid::Uuid;
    let session_id = Uuid::new_v4().to_string();
    Json(proto::models::ShellSession {
        session_id: session_id.clone(),
        target_type: "container".to_string(),
        target_id: id,
        created_at: Utc::now().to_rfc3339(),
        last_activity: Utc::now().to_rfc3339(),
        is_active: true,
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/containers/{id}/attach",
    responses(
        (status = 200, description = "Attach to container's stdout/stderr", body = Value)
    )
)]
pub async fn attach_to_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    Json(serde_json::json!({
        "session_id": uuid::Uuid::new_v4().to_string(),
        "container_id": id,
        "status": "ATTACHED",
        "message": "Streaming container output..."
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/containers/{id}/inspect",
    responses(
        (status = 200, description = "Inspect container details", body = Value)
    )
)]
pub async fn inspect_container(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    Json(serde_json::json!({
        "id": id,
        "created": "2025-12-17T10:00:00Z",
        "path": "/bin/sh",
        "args": ["-c", "npm start"],
        "state": {
            "status": "running",
            "running": true,
            "paused": false,
            "restarting": false,
            "pid": 12345,
            "exit_code": 0,
            "started_at": "2025-12-17T10:00:05Z"
        },
        "image": "node:18-alpine",
        "network_settings": {
            "ip_address": "172.17.0.2",
            "ports": {"3000/tcp": [{"host_ip": "0.0.0.0", "host_port": "3000"}]}
        },
        "mounts": [],
        "config": {
            "hostname": "api-container",
            "env": ["NODE_ENV=production", "PORT=3000"],
            "cmd": ["npm", "start"]
        }
    }))
}
