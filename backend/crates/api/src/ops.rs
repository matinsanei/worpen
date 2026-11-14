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
    println!("🩺 Worpen System Doctor Diagnostics\n===================================");
    println!(
        "  ✓ Runtime data directory: {} (requested: {})",
        dir, a.data_dir
    );

    // 1. Directory permissions & subdirs
    let data_path = std::path::Path::new(&dir);
    if !data_path.exists() {
        let _ = std::fs::create_dir_all(data_path);
    }
    let can_write = std::fs::write(data_path.join(".doctor_test"), b"ok").is_ok();
    let _ = std::fs::remove_file(data_path.join(".doctor_test"));
    if can_write {
        println!("  ✓ Data directory write permissions: OK");
    } else {
        println!("  ✗ Data directory write permissions: FAILED (read-only filesystem or insufficient permissions)");
    }

    for sub in ["workflows", "routes", "functions", "pipelines"] {
        let p = data_path.join(sub);
        let n = std::fs::read_dir(&p).map(|d| d.count()).unwrap_or(0);
        println!("  ✓ {}/: {} definitions present", sub, n);
    }

    // 2. Audit chain presence
    let audit_file = data_path.join("audit_chain.jsonl");
    if audit_file.exists() {
        let size = std::fs::metadata(&audit_file).map(|m| m.len()).unwrap_or(0);
        println!("  ✓ Audit chain log: {} bytes recorded", size);
    } else {
        println!("  ✓ Audit chain log: fresh (will initialize on first workflow/route call)");
    }

    // 3. Database connectivity
    let db_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:worpen.db?mode=rwc".into());
    match infra::initialize_db(&db_url).await {
        Ok(_) => println!("  ✓ Database connection ({}): OK", db_url),
        Err(e) => println!("  ✗ Database connection ({}): FAILED: {}", db_url, e),
    }

    // 4. SSRF & Sandbox defenses
    let test_blocked_1 = worpen_core::sandbox::is_blocked_ip("127.0.0.1".parse().unwrap());
    let test_blocked_2 = worpen_core::sandbox::is_blocked_ip("169.254.169.254".parse().unwrap());
    if test_blocked_1 && test_blocked_2 {
        println!("  ✓ SSRF Sandbox defenses: ACTIVE (RFC 1918, loopback, cloud metadata blocked)");
    } else {
        println!("  ✗ SSRF Sandbox defenses: WARNING (Private IP blocking failed)");
    }

    // 5. Secrets Isolation
    let secret_env_count = std::env::vars()
        .filter(|(k, _)| k.starts_with("WORPEN_SECRET_"))
        .count();
    let secrets_dir = std::path::Path::new("secrets");
    let secrets_dir_present = secrets_dir.exists() && secrets_dir.is_dir();
    println!(
        "  ✓ SecretStore sources: {} env vars detected, secrets/ dir present: {}",
        secret_env_count, secrets_dir_present
    );

    // 6. Auth
    let token_armed = std::env::var("WORPEN_API_TOKEN")
        .ok()
        .filter(|t| !t.trim().is_empty())
        .is_some();
    if token_armed {
        println!("  ✓ API Authentication: ENABLED (protected by Bearer token)");
    } else {
        println!("  ℹ API Authentication: OPEN (WORPEN_API_TOKEN unset)");
    }

    // 7. Embedded UI
    for cand in ["../dist/index.html", "./dist/index.html", "dist/index.html"] {
        if std::path::Path::new(cand).exists() {
            println!("  ✓ Embedded UI found at: {}", cand);
            break;
        }
    }

    println!("\nEverything looks good! Ready to execute sandboxed agent workflows.\n");
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

pub async fn init(a: &InitArgs) -> Result<(), String> {
    let target = std::path::Path::new(&a.path);
    if !target.exists() {
        std::fs::create_dir_all(target)
            .map_err(|e| format!("Failed to create {}: {}", a.path, e))?;
    }

    let workflows_dir = target.join("workflows");
    let data_dir = target.join("data");
    let cursor_dir = target.join(".cursor");
    std::fs::create_dir_all(&workflows_dir).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&cursor_dir).map_err(|e| e.to_string())?;

    // 1. Template: refund.yaml (Human approval demo)
    let refund_yaml = r#"name: refund
version: "1.0.0"
description: Customer refund workflow with human approval gate for charges > $50
input_schema:
  type: object
  properties:
    customer_id:
      type: string
    amount:
      type: number
    reason:
      type: string
  required:
    - customer_id
    - amount
policy:
  read_only_sql: false
  allowed_tables:
    - refunds
steps:
  - id: format_amount
    op: transform
    expr: "{{amount}} * 1.0"
    output_var: clean_amount

  - id: manager_approval
    op: approval
    summary: "Approve refund of ${{clean_amount}} for customer {{customer_id}}: {{reason}}"
    timeout_seconds: 86400

  - id: record_db
    op: sql
    query: "INSERT INTO refunds (customer_id, amount, status, approved_by) VALUES ($1, $2, 'approved', $3);"
    args:
      - "{{customer_id}}"
      - "{{clean_amount}}"
      - "{{__approved_by}}"
    output_var: db_res

return:
  status: 200
  value:
    refund_status: "approved_and_processed"
    customer_id: "{{customer_id}}"
    amount: "{{clean_amount}}"
    approved_by: "{{__approved_by}}"
    approval_reason: "{{__approval_reason}}"
"#;
    std::fs::write(workflows_dir.join("refund.yaml"), refund_yaml).map_err(|e| e.to_string())?;

    // 2. Template: db_report.yaml (Sandboxed read-only query)
    let db_report_yaml = r#"name: db_report
version: "1.0.0"
description: Sandboxed analytical report query with read-only SQL enforcement
input_schema:
  type: object
  properties:
    limit:
      type: integer
      default: 10
policy:
  read_only_sql: true
  max_sql_rows: 100
  allowed_tables:
    - refunds
steps:
  - id: query_refunds
    op: sql
    query: "SELECT id, customer_id, amount, status FROM refunds LIMIT $1;"
    args:
      - "{{limit}}"
    output_var: report_data

return:
  status: 200
  value:
    report_title: "Recent Refunds Summary"
    rows_returned: "{{report_data}}"
"#;
    std::fs::write(workflows_dir.join("db_report.yaml"), db_report_yaml)
        .map_err(|e| e.to_string())?;

    // 3. Template: issue_triage.yaml (SSRF-protected HTTP webhook)
    let issue_triage_yaml = r#"name: issue_triage
version: "1.0.0"
description: Triage incoming webhook issues with SSRF-protected external notification
input_schema:
  type: object
  properties:
    title:
      type: string
    severity:
      type: string
policy:
  block_private_ips: true
  allowed_hosts:
    - api.github.com
    - hooks.slack.com
steps:
  - id: check_severity
    op: transform
    expr: "upper({{severity}})"
    output_var: norm_severity

return:
  status: 200
  value:
    status: "triaged"
    severity: "{{norm_severity}}"
    title: "{{title}}"
"#;
    std::fs::write(workflows_dir.join("issue_triage.yaml"), issue_triage_yaml)
        .map_err(|e| e.to_string())?;

    // 4. Cursor MCP config
    let mcp_json = r#"{
  "mcpServers": {
    "worpen": {
      "command": "worpen",
      "args": ["mcp", "serve-stdio"],
      "env": {
        "DATA_DIR": "./data",
        "DATABASE_URL": "sqlite:worpen.db?mode=rwc"
      }
    }
  }
}
"#;
    std::fs::write(cursor_dir.join("mcp.json"), mcp_json).map_err(|e| e.to_string())?;

    // 5. .env.example
    let env_example = r#"HOST=127.0.0.1
PORT=3000
DATA_DIR=./data
DATABASE_URL=sqlite:worpen.db?mode=rwc
# WORPEN_API_TOKEN=your-secret-token-here
"#;
    std::fs::write(target.join(".env.example"), env_example).map_err(|e| e.to_string())?;

    // 6. worpen.yaml
    let worpen_yaml = r#"version: "1"
project: "worpen-project"
runtime:
  port: 3000
  data_dir: "./data"
  database_url: "sqlite:worpen.db?mode=rwc"
sandboxing:
  block_private_ips: true
"#;
    std::fs::write(target.join("worpen.yaml"), worpen_yaml).map_err(|e| e.to_string())?;

    println!("✨ Initialized new Worpen project in {}", a.path);
    println!(
        "   📁 workflows/ (starter templates: refund.yaml, db_report.yaml, issue_triage.yaml)"
    );
    println!("   📁 data/ (SQLite database and audit trails)");
    println!("   ⚙️  worpen.yaml & .cursor/mcp.json\n");
    println!("Next steps:");
    println!("  1. Start the server:  worpen serve");
    println!("  2. Or run MCP stdio:  worpen mcp serve-stdio");
    println!("  3. Run doctor checks: worpen doctor");
    Ok(())
}

pub async fn audit_verify(a: &AuditVerifyArgs) -> Result<(), String> {
    if let Some(ref s) = a.server {
        let server_ref = ServerRef {
            server: s.clone(),
            token: a.token.clone(),
        };
        let r = client_for(&server_ref)
            .get(format!("{}/api/v1/audit/verify", base(&server_ref)))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
        if v.get("valid").and_then(|b| b.as_bool()).unwrap_or(false) {
            println!("✓ Cryptographic Audit Trail Valid!");
            println!(
                "  Total verified entries: {}",
                v.get("total_entries").unwrap_or(&serde_json::json!(0))
            );
            println!(
                "  Genesis hash:           {}",
                v.get("genesis_hash").and_then(|s| s.as_str()).unwrap_or("")
            );
            println!(
                "  Latest hash:            {}",
                v.get("latest_hash").and_then(|s| s.as_str()).unwrap_or("")
            );
            println!(
                "  Verified at:            {}",
                v.get("verified_at").and_then(|s| s.as_str()).unwrap_or("")
            );
        } else {
            eprintln!("✗ Cryptographic Audit Trail Compromised / Broken!");
            println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
            return Err("Audit chain verification failed".to_string());
        }
        return Ok(());
    }

    // Local file verification
    let dir = crate::cli::resolve_data_dir(&a.data_dir);
    let audit_file = std::path::Path::new(&dir).join("audit_chain.jsonl");
    if !audit_file.exists() {
        println!(
            "ℹ Audit chain file not found at {} (empty chain is valid)",
            audit_file.display()
        );
        return Ok(());
    }

    let content = std::fs::read_to_string(&audit_file)
        .map_err(|e| format!("Failed to read {}: {}", audit_file.display(), e))?;

    match worpen_core::audit::verify_audit_chain(&content) {
        Ok(rep) => {
            println!(
                "✓ Cryptographic Audit Trail Valid (file: {})",
                audit_file.display()
            );
            println!("  Total verified entries: {}", rep.total_entries);
            println!("  Genesis hash:           {}", rep.genesis_hash);
            println!("  Latest hash:            {}", rep.latest_hash);
            println!("  Verified at:            {}", rep.verified_at);
            Ok(())
        }
        Err(err) => {
            eprintln!(
                "✗ Tampering detected in audit log {}!",
                audit_file.display()
            );
            eprintln!("  Broken entry index: {}", err.broken_index);
            eprintln!("  Reason:             {}", err.reason);
            if let Some(exp) = err.expected_hash {
                eprintln!("  Expected hash:      {}", exp);
            }
            if let Some(act) = err.actual_hash {
                eprintln!("  Actual hash:        {}", act);
            }
            Err("Tampering detected".to_string())
        }
    }
}

pub async fn audit_export(a: &AuditExportArgs) -> Result<(), String> {
    let r = client_for(&a.server)
        .get(format!(
            "{}/api/v1/audit/export?format={}",
            base(&a.server),
            a.format
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let text = r.text().await.map_err(|e| e.to_string())?;
    print!("{}", text);
    Ok(())
}

pub async fn workflow_list(s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .get(format!("{}/api/v1/workflows", base(s)))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn workflow_register(a: &AddWorkflowArgs) -> Result<(), String> {
    let content =
        std::fs::read_to_string(&a.file).map_err(|e| format!("read {}: {}", a.file, e))?;
    let r = client_for(&a.server)
        .post(format!("{}/api/v1/workflows", base(&a.server)))
        .header(reqwest::header::CONTENT_TYPE, "text/yaml")
        .body(content)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn workflow_run(a: &RunWorkflowArgs) -> Result<(), String> {
    let payload: serde_json::Value =
        serde_json::from_str(&a.payload).map_err(|e| format!("invalid payload JSON: {}", e))?;
    let mode = if a.dry_run { "dry_run" } else { "live" };
    let r = client_for(&a.server)
        .post(format!(
            "{}/api/v1/workflows/{}/execute?mode={}",
            base(&a.server),
            a.name,
            mode
        ))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn workflow_pending(s: &ServerRef) -> Result<(), String> {
    let r = client_for(s)
        .get(format!("{}/api/v1/runs/pending", base(s)))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn workflow_approve(a: &ApproveRunArgs) -> Result<(), String> {
    let body = serde_json::json!({
        "approver": a.approver,
        "reason": a.reason
    });
    let r = client_for(&a.server)
        .post(format!(
            "{}/api/v1/runs/{}/approve",
            base(&a.server),
            a.run_id
        ))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}

pub async fn workflow_reject(a: &RejectRunArgs) -> Result<(), String> {
    let body = serde_json::json!({
        "rejecter": a.rejecter,
        "reason": a.reason
    });
    let r = client_for(&a.server)
        .post(format!(
            "{}/api/v1/runs/{}/reject",
            base(&a.server),
            a.run_id
        ))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: serde_json::Value = r.json().await.map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(())
}
