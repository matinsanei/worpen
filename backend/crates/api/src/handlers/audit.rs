use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use worpen_core::audit::{export_cef, export_jsonl, export_otel};

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
}

/// Verify cryptographic integrity of the on-disk audit chain
pub async fn verify_audit(State(state): State<AppState>) -> Response {
    let chain = state.dynamic_route_service.audit_chain();
    match chain.verify() {
        Ok(report) => (StatusCode::OK, Json(report)).into_response(),
        Err(err) => (
            StatusCode::CONFLICT,
            Json(json!({
                "valid": false,
                "error": err.to_string(),
                "broken_index": err.broken_index,
                "reason": err.reason,
                "expected_hash": err.expected_hash,
                "actual_hash": err.actual_hash,
            })),
        )
            .into_response(),
    }
}

/// Export audit log in JSONL, CEF, or OpenTelemetry (OTel) log formats
pub async fn export_audit(
    State(state): State<AppState>,
    Query(query): Query<ExportQuery>,
) -> Response {
    let chain = state.dynamic_route_service.audit_chain();
    let records = match chain.read_all() {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to read audit records: {}", e) })),
            )
                .into_response();
        }
    };

    let fmt = query.format.as_deref().unwrap_or("jsonl").to_lowercase();
    match fmt.as_str() {
        "cef" => {
            let body = export_cef(&records);
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "text/plain; charset=utf-8".parse().unwrap(),
            );
            (StatusCode::OK, headers, body).into_response()
        }
        "otel" => {
            let body = export_otel(&records);
            (StatusCode::OK, Json(body)).into_response()
        }
        _ => {
            let body = export_jsonl(&records);
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                "application/x-ndjson; charset=utf-8".parse().unwrap(),
            );
            (StatusCode::OK, headers, body).into_response()
        }
    }
}
