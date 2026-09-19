//! Token authentication for Worpen.
//!
//! - Set `WORPEN_API_TOKEN` to enable. When unset, everything is open
//!   (backward compatible with existing deployments and tests).
//! - When set:
//!   * all MUTATING management APIs require the token, and
//!   * execution of routes with `auth_required=true` requires the token.
//! - Read-only management APIs (list/get/stats/audit/tools) stay open.
//! - Token can be sent as `Authorization: Bearer <token>` or `x-api-token: <token>`.

use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};

/// The configured API token, if any (never logged).
pub fn configured_token() -> Option<String> {
    std::env::var("WORPEN_API_TOKEN")
        .ok()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

fn token_from_headers(headers: &HeaderMap) -> Option<String> {
    if let Some(v) = headers.get("authorization").and_then(|v| v.to_str().ok()) {
        let bare = v.trim().strip_prefix("Bearer ").unwrap_or(v.trim()).trim();
        if !bare.is_empty() {
            return Some(bare.to_string());
        }
    }
    headers
        .get("x-api-token")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn unauthorized() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({
            "error": "unauthorized",
            "message": "valid API token required (Authorization: Bearer <token> or x-api-token header)"
        })),
    )
}

/// Require a valid token when one is configured. Open when unset.
pub fn require_token(headers: &HeaderMap) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let Some(expected) = configured_token() else {
        return Ok(());
    };
    match token_from_headers(headers) {
        // NOTE: plain == comparison; for hostile networks prefer a fixed secret
        // rotation + TLS termination in front of Worpen.
        Some(got) if got == expected => Ok(()),
        _ => Err(unauthorized()),
    }
}

/// Require a token only when the route demands auth AND a token is configured.
pub fn require_route_auth(
    headers: &HeaderMap,
    auth_required: bool,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if auth_required {
        require_token(headers)
    } else {
        Ok(())
    }
}
