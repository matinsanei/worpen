use axum::{
    extract::State,
    Json,
};
use crate::state::AppState;
use proto::models::{LogEntry, LogLevel};

#[utoipa::path(
    get,
    path = "/api/v1/logs",
    responses(
        (status = 200, description = "Get centralized logs", body = Vec<LogEntry>)
    )
)]
pub async fn get_logs(State(_state): State<AppState>) -> Json<Vec<LogEntry>> {
    // Mock log entries
    let logs = vec![
        LogEntry {
            id: "log-001".to_string(),
            timestamp: "2025-12-17T10:00:00Z".to_string(),
            level: LogLevel::INFO,
            source: "api-gateway".to_string(),
            message: "Request processed successfully".to_string(),
        },
        LogEntry {
            id: "log-002".to_string(),
            timestamp: "2025-12-17T10:01:00Z".to_string(),
            level: LogLevel::WARN,
            source: "database".to_string(),
            message: "Connection pool near capacity".to_string(),
        },
        LogEntry {
            id: "log-003".to_string(),
            timestamp: "2025-12-17T10:02:00Z".to_string(),
            level: LogLevel::ERROR,
            source: "bee-001".to_string(),
            message: "Container restart failed".to_string(),
        },
        LogEntry {
            id: "log-004".to_string(),
            timestamp: "2025-12-17T10:03:00Z".to_string(),
            level: LogLevel::SUCCESS,
            source: "auto-healer".to_string(),
            message: "Service recovered automatically".to_string(),
        },
    ];
    
    Json(logs)
}
