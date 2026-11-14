//! Embedded frontend (single-binary mode).
//! Serves the prebuilt SPA (`npm run build` → dist/) from the same port as the API.
//! No CORS, no second process. Missing dist = headless mode (CLI still works).

use axum::{
    body::Body,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

/// Candidate directories for the prebuilt frontend (priority order).
pub fn find_ui_dir(explicit: Option<String>) -> Option<String> {
    let mut cands: Vec<String> = vec![];
    if let Some(e) = explicit {
        cands.push(e);
    }
    if let Ok(env) = std::env::var("FRONTEND_DIR") {
        cands.push(env);
    }
    cands.extend([
        "../dist".to_string(),
        "./dist".to_string(),
        "dist".to_string(),
        "/app/ui".to_string(),
        "../../dist".to_string(),
    ]);
    cands.into_iter().find(|d| {
        let idx = format!("{}/index.html", d.trim_end_matches('/'));
        std::path::Path::new(&idx).exists()
    })
}

fn content_type(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") {
        "text/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else {
        "application/octet-stream"
    }
}

/// Serve a static file from ui_dir, falling back to index.html (SPA routing).
/// Returns None when the request should be handled by the API fallback instead.
pub async fn serve_ui_file(ui_dir: &str, req_path: &str) -> Option<Response> {
    // Never shadow API/system paths
    if req_path.starts_with("/api/")
        || req_path == "/api"
        || req_path.starts_with("/mcp")
        || req_path.starts_with("/swagger")
        || req_path.starts_with("/api-docs")
        || req_path == "/health"
        || req_path == "/openapi.json"
    {
        return None;
    }
    let rel = req_path.trim_start_matches('/');
    let dir = ui_dir.trim_end_matches('/');
    // 1) exact file
    if !rel.is_empty() {
        let fp = format!("{}/{}", dir, rel);
        if std::path::Path::new(&fp).is_file() {
            if let Ok(bytes) = tokio::fs::read(&fp).await {
                return Some(
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, content_type(&fp))],
                        Body::from(bytes),
                    )
                        .into_response(),
                );
            }
        }
    }
    // 2) SPA fallback → index.html
    let index = format!("{}/index.html", dir);
    match tokio::fs::read(&index).await {
        Ok(bytes) => Some(
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                Body::from(bytes),
            )
                .into_response(),
        ),
        Err(_) => None,
    }
}
