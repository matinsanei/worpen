//! CLI operations — HTTP clients talking to a running Worpen server.
//! Everything here works headless (no UI needed) — enterprise requirement.

use crate::cli::*;

/// Client with optional Bearer token from ServerRef (WORPEN_TOKEN).
fn client_for(s: &ServerRef) -> reqwest::Client {
    let mut b = reqwest::Client::builder().timeout(std::time::Duration::from_secs(30));
    if let Some(t) = s.token.as_deref().map(str::trim).filter(|t| !t.is_empty()) {
        let mut h = reqwest::header::HeaderMap::new();
        if let Ok(v) = format!("Bearer {}", t).parse() {
            h.insert(reqwest::header::AUTHORIZATION, v);
            b = b.default_headers(h);
        }
    }
    b.build().expect("http client")
}

fn base(s: &ServerRef) -> String {
    s.server.trim_end_matches('/').to_string()
}

pub async fn route_list(s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .get(format!("{}/api/v1/dynamic-routes", base(s)))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn route_add(a: &AddRouteArgs) -> Result<(), String> {
    let content =
        std::fs::read_to_string(&a.file).map_err(|e| format!("read {}: {}", a.file, e))?;
    // Normalize to JSON Value so we can inject sandbox when --locked
    let mut value: serde_json::Value = if a.file.ends_with(".yaml") || a.file.ends_with(".yml") {
        let y: serde_yaml::Value =
            serde_yaml::from_str(&content).map_err(|e| format!("yaml parse: {}", e))?;
        serde_json::to_value(y).map_err(|e| e.to_string())?
    } else {
        serde_json::from_str(&content).map_err(|e| format!("json parse: {}", e))?
    };
    if a.locked {
        if let Some(obj) = value.as_object_mut() {
            obj.insert(
                "sandbox".to_string(),
                serde_json::json!({
                    "allow_net": false, "allowed_hosts": [],
                    "allow_sql": false, "allow_redis": false, "allow_scripts": false,
                    "max_timeout_ms": 10000, "max_sleep_ms": 1000, "max_loop_iterations": 500,
                    "max_variables": 500
                }),
            );
        }
    }
    let r = client_for(&a.server)
        .post(format!("{}/api/v1/dynamic-routes", base(&a.server)))
        .json(&value)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = r.status();
    let v: serde_json::Value = r.json().await.unwrap_or(serde_json::json!({}));
    println!(
        "{} {}",
        status,
        serde_json::to_string_pretty(&v).unwrap_or_default()
    );
    if !status.is_success() {
        return Err(format!("server returned {}", status));
    }
    Ok(())
}

pub async fn route_export(id: &str, s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .get(format!("{}/api/v1/dynamic-routes/{}/export", base(s), id))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let text = r.text().await.map_err(|e| e.to_string())?;
    println!("{}", text);
    Ok(())
}

pub async fn route_delete(id: &str, s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .delete(format!("{}/api/v1/dynamic-routes/{}", base(s), id))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    println!("{}", r.status());
    Ok(())
}

pub async fn route_exec(a: &ExecRouteArgs) -> Result<(), String> {
    let payload: serde_json::Value =
        serde_json::from_str(&a.payload).unwrap_or(serde_json::json!({}));
    let r = client_for(&a.server)
        .post(format!(
            "{}/api/v1/dynamic-routes/{}/execute",
            base(&a.server),
            a.id
        ))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.unwrap_or(serde_json::json!({}));
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn func_list(s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .get(format!("{}/api/v1/global-functions", base(s)))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn func_add(a: &AddFuncArgs) -> Result<(), String> {
    let content =
        std::fs::read_to_string(&a.file).map_err(|e| format!("read {}: {}", a.file, e))?;
    let value: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("json parse: {}", e))?;
    let r = client_for(&a.server)
        .post(format!("{}/api/v1/global-functions", base(&a.server)))
        .json(&value)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    println!("{}", r.status());
    Ok(())
}

pub async fn func_delete(name: &str, s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .delete(format!("{}/api/v1/global-functions/{}", base(s), name))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = r.status();
    let body = r.text().await.unwrap_or_default();
    println!("{} {}", status, body);
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(format!("function '{}' not found", name));
    }
    if !status.is_success() {
        return Err(format!("server returned {}", status));
    }
    Ok(())
}

pub async fn doctor(a: &DoctorArgs) -> Result<(), String> {
    // Same rule as the server (see cli::resolve_data_dir).
    let mut dir = crate::cli::resolve_data_dir(&a.data_dir);
    if !std::path::Path::new(&dir).exists() {
        dir = a.data_dir.clone();
    }
    println!("data_dir: {} (requested: {})", dir, a.data_dir);
    let data = std::path::Path::new(&dir);
    for sub in ["routes", "functions"] {
        let p = data.join(sub);
        let n = std::fs::read_dir(&p).map(|d| d.count()).unwrap_or(0);
        println!("  {}/: {} files", sub, n);
    }
    for cand in ["../dist/index.html", "./dist/index.html", "dist/index.html"] {
        println!(
            "  ui {} present={}",
            cand,
            std::path::Path::new(cand).exists()
        );
    }
    println!(
        "env DATABASE_URL={}",
        std::env::var("DATABASE_URL").unwrap_or("(default sqlite:worpen.db?mode=rwc)".into())
    );
    // Never print the secret itself — only whether auth is armed.
    println!(
        "auth WORPEN_API_TOKEN={}",
        if std::env::var("WORPEN_API_TOKEN")
            .ok()
            .filter(|t| !t.trim().is_empty())
            .is_some()
        {
            "set (protected)"
        } else {
            "unset (open)"
        }
    );
    Ok(())
}

pub async fn mcp_tools(s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .get(format!("{}/mcp/tools", base(s)))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn mcp_call(a: &McpCallArgs) -> Result<(), String> {
    let args: serde_json::Value = serde_json::from_str(&a.args).unwrap_or(serde_json::json!({}));
    let body = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":a.tool,"arguments":args}});
    let r = client_for(&a.server)
        .post(format!("{}/mcp", base(&a.server)))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn route_audit(a: &AuditArgs) -> Result<(), String> {
    let r = client_for(&a.server)
        .get(format!(
            "{}/api/v1/dynamic-routes/audit?limit={}",
            base(&a.server),
            a.limit
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn pipe_list(s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .get(format!("{}/api/v1/pipelines", base(s)))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn pipe_define(a: &AddPipeArgs) -> Result<(), String> {
    let content =
        std::fs::read_to_string(&a.file).map_err(|e| format!("read {}: {}", a.file, e))?;
    let value: serde_json::Value = if a.file.ends_with(".yaml") || a.file.ends_with(".yml") {
        let y: serde_yaml::Value =
            serde_yaml::from_str(&content).map_err(|e| format!("yaml parse: {}", e))?;
        serde_json::to_value(y).map_err(|e| e.to_string())?
    } else {
        serde_json::from_str(&content).map_err(|e| format!("json parse: {}", e))?
    };
    let r = client_for(&a.server)
        .post(format!("{}/api/v1/pipelines", base(&a.server)))
        .json(&value)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = r.status();
    let v: serde_json::Value = r.json().await.unwrap_or(serde_json::json!({}));
    println!(
        "{} {}",
        status,
        serde_json::to_string_pretty(&v).unwrap_or_default()
    );
    if !status.is_success() {
        return Err(format!("server returned {}", status));
    }
    Ok(())
}

pub async fn pipe_run(a: &RunPipeArgs) -> Result<(), String> {
    let payload: serde_json::Value =
        serde_json::from_str(&a.payload).unwrap_or(serde_json::json!({}));
    let body = serde_json::json!({"pipelineId": a.name, "payload": payload});
    let r = client_for(&a.server)
        .post(format!("{}/api/v1/pipelines/trigger", base(&a.server)))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = r.status();
    let v: serde_json::Value = r.json().await.unwrap_or(serde_json::json!({}));
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    if !status.is_success() {
        return Err(format!("server returned {}", status));
    }
    Ok(())
}

pub async fn pipe_delete(id: &str, s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .delete(format!("{}/api/v1/pipelines/{}", base(s), id))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = r.status();
    println!("{} {}", status, r.text().await.unwrap_or_default());
    if !status.is_success() {
        return Err(format!("server returned {}", status));
    }
    Ok(())
}

pub async fn pipe_runs(a: &RunsArgs) -> Result<(), String> {
    let mut url = format!(
        "{}/api/v1/pipelines/runs?limit={}",
        base(&a.server),
        a.limit
    );
    if let Some(pid) = &a.pipeline_id {
        url.push_str(&format!("&pipeline_id={}", pid));
    }
    let r = client_for(&a.server)
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

fn ask(prompt: &str, default: &str) -> String {
    use std::io::{self, Write};
    if default.is_empty() {
        print!("{}: ", prompt);
    } else {
        print!("{} [{}]: ", prompt, default);
    }
    let _ = io::stdout().flush();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return default.to_string();
    }
    let line = line.trim().to_string();
    if line.is_empty() {
        default.to_string()
    } else {
        line
    }
}

/// Interactive first-run wizard. Asks host/port/token/ui and writes .env.
/// Non-interactive use stays fully flagged (serve --port ...), this is opt-in.
pub async fn setup() -> Result<(), String> {
    println!("Worpen setup — press Enter to accept [defaults].\n");
    let host = ask("Listen host", "127.0.0.1");
    let port = ask("Port", "3000");
    if port.parse::<u16>().is_err() {
        return Err(format!("invalid port '{}'", port));
    }
    let data_dir = ask("Data dir (auto = smart default)", "auto");
    let token = ask("API token (empty = open server, no auth)", "");
    let no_ui = ask("Disable embedded UI? (y/N)", "n");
    let no_ui = matches!(no_ui.to_lowercase().as_str(), "y" | "yes");

    let mut env = format!(
        "# Generated by `worpen setup` — `worpen serve` reads these automatically.\nHOST={}\nPORT={}\nDATA_DIR={}\n",
        host, port, data_dir
    );
    if !token.is_empty() {
        env.push_str(&format!("WORPEN_API_TOKEN={}\n", token));
    } else {
        env.push_str("# WORPEN_API_TOKEN unset — server runs open (no auth).\n");
    }
    if no_ui {
        env.push_str("WORPEN_NO_UI=1\n");
    }
    std::fs::write(".env", &env).map_err(|e| format!("write .env: {}", e))?;

    println!("\nWrote .env:\n{}", env);
    println!("Run: worpen serve   (uses HOST/PORT from .env)");
    println!(
        "Open: http://{}:{}/",
        if host == "0.0.0.0" {
            "127.0.0.1"
        } else {
            &host
        },
        port
    );
    Ok(())
}
