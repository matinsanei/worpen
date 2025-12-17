use axum::{
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use crate::state::AppState;
use proto::models::Agent as ProtoAgent;
use serde_json::Value;
use uuid::Uuid;
use sqlx::types::chrono::Utc;

#[utoipa::path(
    get,
    path = "/api/v1/agents",
    responses(
        (status = 200, description = "List all agents", body = Vec<ProtoAgent>)
    )
)]
pub async fn list_agents(State(state): State<AppState>) -> Json<Vec<ProtoAgent>> {
    match state.agent_service.list_agents().await {
        Ok(agents) => {
            // Convert domain Agent to Proto Agent
            let proto_agents = agents.into_iter().map(|a| ProtoAgent {
                id: a.id.to_string(),
                name: a.hostname.clone(),
                ip: a.ip_address.clone().unwrap_or_default(),
                os: "Linux".to_string(), // Mock for now
                version: a.version.clone(),
                status: proto::models::AgentStatus::ONLINE, // Mock
                cpu_load: 45,
                memory_load: 60,
                last_seen: a.last_heartbeat.to_rfc3339(),
                uptime: "24h".to_string(),
                peers: 3,
            }).collect();
            Json(proto_agents)
        },
        Err(_) => Json(vec![]),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/agents/{id}",
    responses(
        (status = 200, description = "Get agent details", body = ProtoAgent),
        (status = 404, description = "Agent not found")
    )
)]
pub async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProtoAgent>, StatusCode> {
    let agent_id = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    match state.agent_service.get_agent(agent_id).await {
        Ok(Some(a)) => Ok(Json(ProtoAgent {
            id: a.id.to_string(),
            name: a.hostname.clone(),
            ip: a.ip_address.clone().unwrap_or_default(),
            os: "Linux".to_string(),
            version: a.version.clone(),
            status: proto::models::AgentStatus::ONLINE,
            cpu_load: 45,
            memory_load: 60,
            last_seen: a.last_heartbeat.to_rfc3339(),
            uptime: "24h".to_string(),
            peers: 3,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/agents/sync",
    responses(
        (status = 200, description = "Sync signal sent", body = Value)
    )
)]
pub async fn sync_agents(State(state): State<AppState>) -> Json<Value> {
    match state.agent_service.sync_agents().await {
        Ok(_) => Json(serde_json::json!({"status": "SYNCED"})),
        Err(e) => Json(serde_json::json!({"error": e})),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/internal/heartbeat",
    responses(
        (status = 200, description = "Heartbeat received", body = Value)
    )
)]
pub async fn agent_heartbeat(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let agent_id_str = payload["id"].as_str().unwrap_or("");
    
    if let Ok(agent_id) = Uuid::parse_str(agent_id_str) {
        match state.agent_service.process_heartbeat(agent_id).await {
            Ok(_) => Json(serde_json::json!({"status": "OK"})),
            Err(e) => Json(serde_json::json!({"error": e})),
        }
    } else {
        Json(serde_json::json!({"error": "Invalid agent ID"}))
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/agents/{id}/exec",
    responses(
        (status = 200, description = "Execute command on agent", body = proto::models::ExecResponse)
    )
)]
pub async fn exec_on_agent(
    State(_state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<proto::models::ExecRequest>,
) -> Json<proto::models::ExecResponse> {
    // Mock execution on agent
    let output = format!(
        "Executed '{}' on agent {} with args: {:?}",
        payload.command, id, payload.args
    );
    Json(proto::models::ExecResponse {
        exit_code: 0,
        stdout: output,
        stderr: String::new(),
        execution_time_ms: 95,
    })
}

#[utoipa::path(
    post,
    path = "/api/v1/agents/{id}/shell",
    responses(
        (status = 200, description = "Start interactive shell on agent", body = proto::models::ShellSession)
    )
)]
pub async fn start_agent_shell(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<proto::models::ShellSession> {
    let session_id = Uuid::new_v4().to_string();
    Json(proto::models::ShellSession {
        session_id: session_id.clone(),
        target_type: "agent".to_string(),
        target_id: id,
        created_at: Utc::now().to_rfc3339(),
        last_activity: Utc::now().to_rfc3339(),
        is_active: true,
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/agents/{id}/processes",
    responses(
        (status = 200, description = "List running processes on agent", body = Value)
    )
)]
pub async fn list_agent_processes(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    // Mock process list
    Json(serde_json::json!({
        "agent_id": id,
        "processes": [
            {"pid": 1234, "name": "docker", "cpu": 2.5, "memory": "128MB", "status": "running"},
            {"pid": 5678, "name": "nginx", "cpu": 0.8, "memory": "64MB", "status": "running"},
            {"pid": 9012, "name": "postgres", "cpu": 5.2, "memory": "512MB", "status": "running"}
        ]
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/agents/{id}/system",
    responses(
        (status = 200, description = "Get agent system information", body = Value)
    )
)]
pub async fn get_agent_system_info(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    // Mock system info
    Json(serde_json::json!({
        "agent_id": id,
        "hostname": "bee-worker-01",
        "os": "Ubuntu 22.04 LTS",
        "kernel": "5.15.0-91-generic",
        "architecture": "x86_64",
        "cpu": {
            "cores": 8,
            "model": "Intel Xeon E5-2680",
            "usage": 45.2
        },
        "memory": {
            "total": "16GB",
            "used": "9.6GB",
            "free": "6.4GB",
            "usage_percent": 60
        },
        "disk": {
            "total": "500GB",
            "used": "320GB",
            "free": "180GB",
            "usage_percent": 64
        },
        "network": {
            "interfaces": [
                {"name": "eth0", "ip": "192.168.1.100", "rx": "1.2TB", "tx": "890GB"}
            ]
        },
        "uptime": "15 days, 7:32:15"
    }))
}
