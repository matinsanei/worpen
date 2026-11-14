use crate::compiler::symbol_table::SymbolTable;
use crate::secrets::SecretStore;
use crate::vm::instructions::OptimizedOperation;
use crate::vm::memory::ExecutionMemory;
use crate::websocket::WebSocketManager;
use proto::models::{LoopControl, SandboxPolicy};
use redis::AsyncCommands;
use regex::Regex;
use serde_json::Value;
use sqlx::{Column, Row};
use std::sync::Arc;

pub struct VirtualMachine {
    pub memory: ExecutionMemory,
    symbol_table: SymbolTable,
    db_pool: Option<sqlx::Pool<sqlx::Sqlite>>,
    redis_pool: Option<deadpool_redis::Pool>,
    ws_manager: Option<WebSocketManager>,
    ws_connection_id: Option<String>,
    sandbox: Option<SandboxPolicy>,
    secret_store: Option<Arc<SecretStore>>,
}

impl VirtualMachine {
    pub fn new(memory: ExecutionMemory, symbol_table: SymbolTable) -> Self {
        Self {
            memory,
            symbol_table,
            db_pool: None,
            redis_pool: None,
            ws_manager: None,
            ws_connection_id: None,
            sandbox: None,
            secret_store: None,
        }
    }

    pub fn with_sandbox(mut self, policy: SandboxPolicy) -> Self {
        self.sandbox = Some(policy);
        self
    }

    pub fn set_sandbox(&mut self, policy: Option<SandboxPolicy>) {
        self.sandbox = policy;
    }

    pub fn with_secret_store(mut self, store: Arc<SecretStore>) -> Self {
        self.secret_store = Some(store);
        self
    }

    pub fn set_secret_store(&mut self, store: Arc<SecretStore>) {
        self.secret_store = Some(store);
    }

    pub fn with_db_pool(
        memory: ExecutionMemory,
        symbol_table: SymbolTable,
        db_pool: sqlx::Pool<sqlx::Sqlite>,
    ) -> Self {
        Self {
            memory,
            symbol_table,
            db_pool: Some(db_pool),
            redis_pool: None,
            ws_manager: None,
            ws_connection_id: None,
            sandbox: None,
            secret_store: None,
        }
    }

    pub fn with_redis_pool(
        memory: ExecutionMemory,
        symbol_table: SymbolTable,
        redis_pool: deadpool_redis::Pool,
    ) -> Self {
        Self {
            memory,
            symbol_table,
            db_pool: None,
            redis_pool: Some(redis_pool),
            ws_manager: None,
            ws_connection_id: None,
            sandbox: None,
            secret_store: None,
        }
    }

    pub fn with_both_pools(
        memory: ExecutionMemory,
        symbol_table: SymbolTable,
        db_pool: sqlx::Pool<sqlx::Sqlite>,
        redis_pool: deadpool_redis::Pool,
    ) -> Self {
        Self {
            memory,
            symbol_table,
            db_pool: Some(db_pool),
            redis_pool: Some(redis_pool),
            ws_manager: None,
            ws_connection_id: None,
            sandbox: None,
            secret_store: None,
        }
    }

    pub fn with_websocket(
        memory: ExecutionMemory,
        symbol_table: SymbolTable,
        ws_manager: WebSocketManager,
        connection_id: String,
    ) -> Self {
        Self {
            memory,
            symbol_table,
            db_pool: None,
            redis_pool: None,
            ws_manager: Some(ws_manager),
            ws_connection_id: Some(connection_id),
            sandbox: None,
            secret_store: None,
        }
    }

    pub fn with_all(
        memory: ExecutionMemory,
        symbol_table: SymbolTable,
        db_pool: Option<sqlx::Pool<sqlx::Sqlite>>,
        redis_pool: Option<deadpool_redis::Pool>,
        ws_manager: Option<WebSocketManager>,
        connection_id: Option<String>,
    ) -> Self {
        Self {
            memory,
            symbol_table,
            db_pool,
            redis_pool,
            ws_manager,
            ws_connection_id: connection_id,
            sandbox: None,
            secret_store: None,
        }
    }

    #[allow(dead_code)]
    fn check_net(&self, url: &str) -> Result<String, String> {
        if let Some(policy) = &self.sandbox {
            if !policy.allow_net {
                return Err("sandbox: network access denied (allow_net=false)".to_string());
            }
            let after_scheme = url.split("://").nth(1).unwrap_or(url);
            let host_port = after_scheme.split('/').next().unwrap_or("");
            let host = host_port
                .split('@')
                .next_back()
                .unwrap_or(host_port)
                .split(':')
                .next()
                .unwrap_or(host_port);
            if !policy.is_host_allowed(host) {
                return Err(format!(
                    "sandbox: host '{}' not in allowed_hosts {:?}",
                    host, policy.allowed_hosts
                ));
            }
        }
        Ok(url.to_string())
    }

    pub async fn execute(&mut self, program: &[OptimizedOperation]) -> Result<Value, String> {
        let mut result = Value::Null;
        let mut loop_control = LoopControl::default();

        for op in program.iter() {
            // Handle loop control
            if loop_control.should_break {
                break;
            }
            if loop_control.should_continue {
                loop_control.should_continue = false;
                continue;
            }

            match op {
                OptimizedOperation::Return {
                    value,
                    status,
                    headers,
                    raw,
                } => {
                    // Resolve the return value
                    let mut resolved = self.resolve_value(value)?;

                    // Build enhanced return with metadata if any custom fields are set
                    if status.is_some() || headers.is_some() || raw.is_some() {
                        let mut return_obj = serde_json::Map::new();
                        return_obj.insert("value".to_string(), resolved);

                        if let Some(s) = status {
                            return_obj.insert("status".to_string(), Value::Number((*s).into()));
                        }
                        if let Some(h) = headers {
                            return_obj.insert(
                                "headers".to_string(),
                                serde_json::to_value(h).unwrap_or(Value::Null),
                            );
                        }
                        if let Some(r) = raw {
                            return_obj.insert("raw".to_string(), Value::Bool(*r));
                        }

                        resolved = Value::Object(return_obj);
                    }

                    result = resolved;
                    break;
                }
                OptimizedOperation::Comment { .. } => {
                    // Comment is a no-op, skip execution
                    continue;
                }
                OptimizedOperation::Set { var_index, value } => {
                    let resolved = self.resolve_value(value)?;
                    self.memory.set(*var_index, resolved);
                    // Sandbox memory guard (sparse-vec length approximation)
                    if let Some(policy) = &self.sandbox {
                        if self.memory.len() > policy.max_variables {
                            return Err(format!(
                                "sandbox: variable count exceeds max_variables {}",
                                policy.max_variables
                            ));
                        }
                    }
                }
                OptimizedOperation::Get { var_index } => {
                    result = self.memory.get(*var_index).cloned().unwrap_or(Value::Null);
                }
                OptimizedOperation::MathOp { operation, args } => {
                    let resolved_args: Result<Vec<Value>, String> =
                        args.iter().map(|arg| self.resolve_value(arg)).collect();
                    let resolved_args = resolved_args?;
                    result = self.execute_math_op(operation, &resolved_args)?;
                    // Mirror interpreter: {{math_result}} must be readable afterwards
                    let idx = match self.symbol_table.get_index("math_result") {
                        Some(i) => i,
                        None => self.symbol_table.register("math_result".to_string()),
                    };
                    self.memory.set(idx, result.clone());
                }
                OptimizedOperation::If {
                    condition,
                    then,
                    otherwise,
                } => {
                    let resolved_condition = self.resolve_string(condition)?;
                    let condition_result = self.evaluate_condition(&resolved_condition)?;
                    if condition_result {
                        result = Box::pin(self.execute(then)).await?;
                    } else if let Some(else_ops) = otherwise {
                        result = Box::pin(self.execute(else_ops)).await?;
                    }
                }
                OptimizedOperation::SqlOp {
                    query,
                    arg_indices,
                    output_var_index,
                } => {
                    if let Some(policy) = &self.sandbox {
                        if !policy.allow_sql {
                            return Err("sandbox: sql_op denied (allow_sql=false)".to_string());
                        }
                    }
                    // Resolve all args from memory or secret store
                    let resolved_args: Vec<sqlx::Either<String, i64>> = arg_indices
                        .iter()
                        .map(|idx| {
                            if let Some(var_name) = self.symbol_table.get_name(*idx) {
                                let trimmed = var_name.trim();
                                if let Some(secret_name) = trimmed.strip_prefix("secret:") {
                                    if let Some(store) = &self.secret_store {
                                        let val = store.resolve_secret(secret_name.trim())?;
                                        return Ok(sqlx::Either::Left(val));
                                    } else {
                                        return Err(format!(
                                            "Secret store not configured to resolve '{}'",
                                            trimmed
                                        ));
                                    }
                                }
                            }
                            if let Some(val) = self.memory.get(*idx) {
                                // Convert Value to sqlx compatible type
                                match val {
                                    Value::Number(n) => {
                                        if let Some(i) = n.as_i64() {
                                            Ok(sqlx::Either::Right(i))
                                        } else if let Some(f) = n.as_f64() {
                                            Ok(sqlx::Either::Left(f.to_string()))
                                        } else {
                                            Ok(sqlx::Either::Left(n.to_string()))
                                        }
                                    }
                                    Value::String(s) => {
                                        let resolved = if let Some(store) = &self.secret_store {
                                            store.inject_secrets(s)?
                                        } else {
                                            s.clone()
                                        };
                                        Ok(sqlx::Either::Left(resolved))
                                    }
                                    _ => Ok(sqlx::Either::Left(val.to_string())),
                                }
                            } else {
                                Err(format!("Variable at index {} not set", idx))
                            }
                        })
                        .collect::<Result<Vec<_>, String>>()?;

                    let resolved_query = if let Some(store) = &self.secret_store {
                        store.inject_secrets(query)?
                    } else {
                        query.clone()
                    };

                    if let Some(policy) = &self.sandbox {
                        crate::sandbox::validate_sql_policy(&resolved_query, policy)?;
                    }

                    // Execute SQL query
                    if let Some(pool) = &self.db_pool {
                        let mut query_builder = sqlx::query(&resolved_query);

                        // Bind all arguments
                        for arg in resolved_args {
                            match arg {
                                sqlx::Either::Left(s) => {
                                    query_builder = query_builder.bind(s);
                                }
                                sqlx::Either::Right(i) => {
                                    query_builder = query_builder.bind(i);
                                }
                            }
                        }

                        // Execute and fetch results
                        let rows = query_builder
                            .fetch_all(pool)
                            .await
                            .map_err(|e| format!("SQL execution error: {}", e))?;

                        // Convert rows to JSON array
                        let mut json_rows: Vec<Value> = rows
                            .iter()
                            .map(|row| {
                                let mut obj = serde_json::Map::new();
                                for (i, col) in row.columns().iter().enumerate() {
                                    let col_name = col.name().to_string();

                                    // Try to get the value as different types
                                    let val = if let Ok(v) = row.try_get::<String, _>(i) {
                                        Value::String(v)
                                    } else if let Ok(v) = row.try_get::<i64, _>(i) {
                                        Value::Number(v.into())
                                    } else if let Ok(v) = row.try_get::<f64, _>(i) {
                                        Value::Number(
                                            serde_json::Number::from_f64(v).unwrap_or(0.into()),
                                        )
                                    } else if let Ok(v) = row.try_get::<bool, _>(i) {
                                        Value::Bool(v)
                                    } else {
                                        Value::Null
                                    };

                                    obj.insert(col_name, val);
                                }
                                Value::Object(obj)
                            })
                            .collect();

                        // Enforce row caps
                        if let Some(policy) = &self.sandbox {
                            if json_rows.len() > policy.max_sql_rows {
                                json_rows.truncate(policy.max_sql_rows);
                            }
                        }

                        // Store result in memory
                        self.memory
                            .set(*output_var_index, Value::Array(json_rows.clone()));
                        result = Value::Array(json_rows);
                    } else {
                        return Err("Database pool not available for SqlOp".to_string());
                    }
                }
                OptimizedOperation::RedisOp {
                    command,
                    key,
                    value,
                    ttl_seconds,
                    output_var_index,
                } => {
                    if let Some(policy) = &self.sandbox {
                        if !policy.allow_redis {
                            return Err("sandbox: redis_op denied (allow_redis=false)".to_string());
                        }
                    }
                    if let Some(redis_pool) = &self.redis_pool {
                        // Resolve key template
                        let resolved_key = self.resolve_string(key)?;

                        // Get Redis connection
                        let mut conn = redis_pool
                            .get()
                            .await
                            .map_err(|e| format!("Failed to get Redis connection: {}", e))?;

                        // Execute Redis command
                        let redis_result: Value = match command.as_str() {
                            "GET" => {
                                let value: Option<String> = conn
                                    .get(&resolved_key)
                                    .await
                                    .map_err(|e| format!("Redis GET error: {}", e))?;
                                value.map(Value::String).unwrap_or(Value::Null)
                            }
                            "SET" => {
                                if let Some(val) = value {
                                    let resolved_value = self.resolve_string(val)?;

                                    if let Some(ttl) = ttl_seconds {
                                        // SET with TTL
                                        let _: () = conn
                                            .set_ex(&resolved_key, &resolved_value, *ttl)
                                            .await
                                            .map_err(|e| {
                                                format!("Redis SET with TTL error: {}", e)
                                            })?;
                                    } else {
                                        // SET without TTL
                                        let _: () = conn
                                            .set(&resolved_key, &resolved_value)
                                            .await
                                            .map_err(|e| format!("Redis SET error: {}", e))?;
                                    }
                                    Value::String("OK".to_string())
                                } else {
                                    return Err("SET command requires a value".to_string());
                                }
                            }
                            "DEL" => {
                                let count: i32 = conn
                                    .del(&resolved_key)
                                    .await
                                    .map_err(|e| format!("Redis DEL error: {}", e))?;
                                Value::Number(count.into())
                            }
                            "EXPIRE" => {
                                if let Some(ttl) = ttl_seconds {
                                    let success: bool = conn
                                        .expire(&resolved_key, *ttl as i64)
                                        .await
                                        .map_err(|e| format!("Redis EXPIRE error: {}", e))?;
                                    Value::Bool(success)
                                } else {
                                    return Err("EXPIRE command requires ttl_seconds".to_string());
                                }
                            }
                            "INCR" => {
                                let new_value: i64 = conn
                                    .incr(&resolved_key, 1)
                                    .await
                                    .map_err(|e| format!("Redis INCR error: {}", e))?;
                                Value::Number(new_value.into())
                            }
                            "DECR" => {
                                let new_value: i64 = conn
                                    .decr(&resolved_key, 1)
                                    .await
                                    .map_err(|e| format!("Redis DECR error: {}", e))?;
                                Value::Number(new_value.into())
                            }
                            _ => {
                                return Err(format!("Unsupported Redis command: {}", command));
                            }
                        };

                        // Store result in memory if output variable specified
                        if let Some(index) = output_var_index {
                            self.memory.set(*index, redis_result.clone());
                        }
                        result = redis_result;
                    } else {
                        return Err("Redis pool not available for RedisOp".to_string());
                    }
                }
                OptimizedOperation::WsOp {
                    command,
                    message,
                    channel,
                } => {
                    if let Some(ws_manager) = &self.ws_manager {
                        // Resolve message template
                        let resolved_message = self.resolve_string(message)?;

                        // Execute WebSocket command
                        match command.as_str() {
                            "send" => {
                                // Send to current connection only
                                if let Some(conn_id) = &self.ws_connection_id {
                                    ws_manager
                                        .send_to(conn_id, resolved_message)
                                        .map_err(|e| format!("WebSocket send error: {}", e))?;
                                    result = Value::String("sent".to_string());
                                } else {
                                    return Err("No active WebSocket connection for send command"
                                        .to_string());
                                }
                            }
                            "broadcast" => {
                                // Broadcast to all or to specific channel
                                if let Some(ch) = channel {
                                    let resolved_channel = self.resolve_string(ch)?;
                                    ws_manager
                                        .broadcast_to_channel(&resolved_channel, resolved_message)
                                        .map_err(|e| format!("WebSocket broadcast error: {}", e))?;
                                    result = Value::String(format!(
                                        "broadcast to channel {}",
                                        resolved_channel
                                    ));
                                } else {
                                    ws_manager
                                        .broadcast(resolved_message)
                                        .map_err(|e| format!("WebSocket broadcast error: {}", e))?;
                                    result = Value::String("broadcast to all".to_string());
                                }
                            }
                            _ => {
                                return Err(format!("Unsupported WebSocket command: {}", command));
                            }
                        }
                    } else {
                        return Err("WebSocket manager not available for WsOp".to_string());
                    }
                }
                // Add more operations as needed
                OptimizedOperation::HttpRequest {
                    url,
                    method,
                    body,
                    headers,
                    timeout_ms,
                } => {
                    let resolved_url = self.resolve_string(url)?;
                    let timeout = timeout_ms.unwrap_or(
                        self.sandbox
                            .as_ref()
                            .map(|p| p.max_timeout_ms)
                            .unwrap_or(10_000),
                    );

                    let mut client_builder = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_millis(timeout));

                    if let Some(policy) = &self.sandbox {
                        let (host, port, pinned_ip) =
                            crate::sandbox::resolve_and_validate_host(&resolved_url, policy)
                                .await?;
                        // Pin connection to validated IP to defend against DNS rebinding
                        client_builder = client_builder
                            .resolve(&host, std::net::SocketAddr::new(pinned_ip, port));

                        // Safe redirect defense
                        let policy_clone = policy.clone();
                        let redirect_policy = reqwest::redirect::Policy::custom(move |attempt| {
                            if attempt.previous().len() >= 5 {
                                return attempt.stop();
                            }
                            let next_host_opt = attempt.url().host_str().map(|h| h.to_string());
                            if let Some(next_host) = next_host_opt {
                                if !policy_clone.is_host_allowed(&next_host) {
                                    return attempt.error(format!(
                                        "sandbox: redirect to host '{}' denied by allowlist",
                                        next_host
                                    ));
                                }
                                if policy_clone.block_private_ips {
                                    if let Ok(ip) = next_host.parse::<std::net::IpAddr>() {
                                        if crate::sandbox::is_blocked_ip(ip) {
                                            return attempt.error(format!(
                                                "sandbox: redirect to private IP '{}' denied",
                                                ip
                                            ));
                                        }
                                    }
                                }
                            }
                            attempt.follow()
                        });
                        client_builder = client_builder.redirect(redirect_policy);
                    }

                    let client = client_builder
                        .build()
                        .map_err(|e| format!("HTTP client error: {}", e))?;
                    let mut rb = match method.as_str() {
                        "GET" => client.get(&resolved_url),
                        "POST" => client.post(&resolved_url),
                        "PUT" => client.put(&resolved_url),
                        "DELETE" => client.delete(&resolved_url),
                        "PATCH" => client.patch(&resolved_url),
                        "HEAD" => client.head(&resolved_url),
                        _ => return Err(format!("Unsupported HTTP method: {}", method)),
                    };
                    if let Some(hdrs) = headers {
                        for (k, v) in hdrs {
                            let rv = self.resolve_string(v)?;
                            rb = rb.header(k, rv);
                        }
                    }
                    if let Some(b) = body {
                        let resolved_body = self.resolve_value(b)?;
                        let s = match resolved_body {
                            Value::String(st) => st,
                            _ => serde_json::to_string(&resolved_body).unwrap_or_default(),
                        };
                        rb = rb.body(s);
                    }
                    let resp = rb
                        .send()
                        .await
                        .map_err(|e| format!("HTTP request failed: {}", e))?;
                    let status = resp.status().as_u16();
                    let text = resp
                        .text()
                        .await
                        .map_err(|e| format!("read body failed: {}", e))?;
                    let body_json: Value =
                        serde_json::from_str(&text).unwrap_or(Value::String(text));
                    let res_val = serde_json::json!({"status": status, "body": body_json, "url": resolved_url, "method": method});
                    if let Some(idx) = self.symbol_table.get_index("http_response") {
                        self.memory.set(idx, res_val.clone());
                    }
                    result = res_val;
                }
                OptimizedOperation::Sleep { duration_ms } => {
                    if let Some(policy) = &self.sandbox {
                        if *duration_ms > policy.max_sleep_ms {
                            return Err(format!(
                                "sandbox: sleep {}ms exceeds max_sleep_ms {}ms",
                                duration_ms, policy.max_sleep_ms
                            ));
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(*duration_ms)).await;
                    result = Value::Null;
                }
                OptimizedOperation::ExecuteScript { language, .. } => {
                    if self
                        .sandbox
                        .as_ref()
                        .map(|p| !p.allow_scripts)
                        .unwrap_or(true)
                    {
                        return Err(format!(
                            "sandbox: execute_script '{}' denied (allow_scripts=false)",
                            language
                        ));
                    }
                    result = serde_json::json!({"result": "Script executed", "language": language});
                }
                OptimizedOperation::Log { level, message } => {
                    if let Ok(msg) = self.resolve_string(message) {
                        println!("[{}] {}", level.to_uppercase(), msg);
                    }
                    result = Value::Null;
                }
                OptimizedOperation::While {
                    condition,
                    body,
                    max_iterations,
                } => {
                    let cap = self
                        .sandbox
                        .as_ref()
                        .map(|p| p.max_loop_iterations)
                        .unwrap_or(1000);
                    let max_iter = max_iterations.unwrap_or(cap).min(cap.max(1));
                    let mut iters = 0u32;
                    while iters < max_iter {
                        let resolved = self.resolve_string(condition).unwrap_or_default();
                        if !self.evaluate_condition(&resolved).unwrap_or(false) {
                            break;
                        }
                        iters += 1;
                        result = Box::pin(self.execute(body)).await?;
                        if loop_control.should_break {
                            loop_control.should_break = false;
                            break;
                        }
                        if loop_control.should_continue {
                            loop_control.should_continue = false;
                            continue;
                        }
                    }
                }
                OptimizedOperation::Loop {
                    collection,
                    var_index,
                    body,
                } => {
                    let resolved = self.resolve_string(collection).unwrap_or_default();
                    // Try memory lookup first, else parse literal
                    let items: Value = self
                        .memory
                        .get(*var_index)
                        .cloned()
                        .or_else(|| serde_json::from_str(&resolved).ok())
                        .unwrap_or(Value::Null);
                    // collection may be a var template; attempt symbol resolution
                    let arr: Vec<Value> = if let Value::Array(a) = items {
                        a
                    } else if let Ok(Value::Array(a)) = serde_json::from_str::<Value>(&resolved) {
                        a
                    } else {
                        vec![]
                    };
                    if arr.len() > 10_000 {
                        return Err("sandbox: loop collection exceeds 10_000 items".to_string());
                    }
                    for item in arr {
                        self.memory.set(*var_index, item);
                        result = Box::pin(self.execute(body)).await?;
                        if loop_control.should_break {
                            loop_control.should_break = false;
                            break;
                        }
                        if loop_control.should_continue {
                            loop_control.should_continue = false;
                            continue;
                        }
                    }
                }
                OptimizedOperation::Switch {
                    value,
                    cases,
                    default,
                } => {
                    let resolved = self.resolve_string(value).unwrap_or_default();
                    let mut matched = false;
                    for case in cases {
                        let case_str = match &case.value {
                            Value::String(s) => s.clone(),
                            _ => case.value.to_string().trim_matches('"').to_string(),
                        };
                        if resolved == case_str {
                            result = Box::pin(self.execute(&case.operations)).await?;
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        if let Some(def) = default {
                            result = Box::pin(self.execute(def)).await?;
                        }
                    }
                }
                OptimizedOperation::Try {
                    body,
                    catch,
                    finally,
                } => {
                    match Box::pin(self.execute(body)).await {
                        Ok(v) => result = v,
                        Err(_) => {
                            result = Box::pin(self.execute(catch)).await?;
                        }
                    }
                    if let Some(fin) = finally {
                        let _ = Box::pin(self.execute(fin)).await;
                    }
                }
                OptimizedOperation::Throw { message, .. } => {
                    let msg = self
                        .resolve_string(message)
                        .unwrap_or_else(|_| message.clone());
                    return Err(msg);
                }
                OptimizedOperation::Break => {
                    loop_control.should_break = true;
                }
                OptimizedOperation::Continue => {
                    loop_control.should_continue = false;
                    loop_control.should_break = false;
                }
                _ => {
                    // For now, skip unimplemented operations
                }
            }
        }

        Ok(result)
    }

    /// Convert a memory value to its template string form.
    /// Strings interpolate RAW (no JSON quotes) — matches the interpreter.
    fn value_to_plain_string(val: &Value) -> String {
        match val {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        }
    }

    fn resolve_value(&self, value: &Value) -> Result<Value, String> {
        match value {
            Value::String(s) => {
                let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
                let mut result = s.clone();

                for cap in re.captures_iter(s) {
                    if let (Some(full_match), Some(var_match)) = (cap.get(0), cap.get(1)) {
                        let raw_var = var_match.as_str().trim();
                        if let Some(secret_name) = raw_var.strip_prefix("secret:") {
                            let secret_val = if let Some(store) = &self.secret_store {
                                store.resolve_secret(secret_name.trim())?
                            } else {
                                return Err(format!(
                                    "Secret store not configured to resolve '{}'",
                                    raw_var
                                ));
                            };
                            result = result.replace(full_match.as_str(), &secret_val);
                            continue;
                        }
                        let var_name = var_match.as_str();
                        if let Some(index) = self.symbol_table.get_index(var_name) {
                            if let Some(val) = self.memory.get(index) {
                                let val_str = Self::value_to_plain_string(val);
                                result = result.replace(full_match.as_str(), &val_str);
                            } else {
                                return Err(format!("Variable '{}' not set", var_name));
                            }
                        } else {
                            return Err(format!("Unknown variable '{}'", var_name));
                        }
                    }
                }

                // Try to parse as JSON, otherwise keep as string
                if let Ok(parsed) = serde_json::from_str(&result) {
                    Ok(parsed)
                } else {
                    Ok(Value::String(result))
                }
            }
            Value::Array(arr) => {
                let resolved: Result<Vec<Value>, String> =
                    arr.iter().map(|v| self.resolve_value(v)).collect();
                Ok(Value::Array(resolved?))
            }
            Value::Object(obj) => {
                let resolved: Result<serde_json::Map<String, Value>, String> = obj
                    .iter()
                    .map(|(k, v)| self.resolve_value(v).map(|rv| (k.clone(), rv)))
                    .collect();
                Ok(Value::Object(resolved?))
            }
            _ => Ok(value.clone()),
        }
    }

    fn resolve_string(&self, s: &str) -> Result<String, String> {
        let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let mut result = s.to_string();

        for cap in re.captures_iter(s) {
            if let (Some(full_match), Some(var_match)) = (cap.get(0), cap.get(1)) {
                let raw_var = var_match.as_str().trim();
                if let Some(secret_name) = raw_var.strip_prefix("secret:") {
                    let secret_val = if let Some(store) = &self.secret_store {
                        store.resolve_secret(secret_name.trim())?
                    } else {
                        return Err(format!(
                            "Secret store not configured to resolve '{}'",
                            raw_var
                        ));
                    };
                    result = result.replace(full_match.as_str(), &secret_val);
                    continue;
                }
                let var_name = var_match.as_str();
                if let Some(index) = self.symbol_table.get_index(var_name) {
                    if let Some(val) = self.memory.get(index) {
                        let val_str = Self::value_to_plain_string(val);
                        result = result.replace(full_match.as_str(), &val_str);
                    } else {
                        return Err(format!("Variable '{}' not set", var_name));
                    }
                } else {
                    return Err(format!("Unknown variable '{}'", var_name));
                }
            }
        }

        Ok(result)
    }

    fn evaluate_condition(&self, condition: &str) -> Result<bool, String> {
        // Simple condition evaluation - for now, assume it's a boolean string or comparison
        // This is a placeholder; in real implementation, you'd parse and evaluate expressions
        match condition.trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => {
                // Try to parse as JSON boolean
                if let Ok(val) = serde_json::from_str::<Value>(condition) {
                    match val {
                        Value::Bool(b) => Ok(b),
                        _ => Err(format!("Cannot evaluate condition: {}", condition)),
                    }
                } else {
                    Err(format!("Cannot evaluate condition: {}", condition))
                }
            }
        }
    }

    fn execute_math_op(&self, operation: &str, args: &[Value]) -> Result<Value, String> {
        // Parity with interpreter math.rs — resolve {{vars}} then coerce numbers
        let numbers: Vec<f64> = args
            .iter()
            .filter_map(|v| match v {
                Value::Number(n) => n.as_f64(),
                Value::String(s) => s.parse().ok(),
                _ => None,
            })
            .collect();
        let num = |f: f64| Value::Number(serde_json::Number::from_f64(f).unwrap_or(0.into()));
        match operation {
            "add" | "sum" => Ok(num(numbers.iter().sum())),
            "avg" => {
                if numbers.is_empty() {
                    return Err("avg requires at least one number".to_string());
                }
                Ok(num(numbers.iter().sum::<f64>() / numbers.len() as f64))
            }
            "min" => Ok(num(numbers.iter().cloned().fold(f64::INFINITY, f64::min))),
            "max" => Ok(num(numbers
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max))),
            "round" => Ok(num(numbers.first().unwrap_or(&0.0).round())),
            "floor" => Ok(num(numbers.first().unwrap_or(&0.0).floor())),
            "ceil" => Ok(num(numbers.first().unwrap_or(&0.0).ceil())),
            "abs" => Ok(num(numbers.first().unwrap_or(&0.0).abs())),
            "pow" => {
                let b = *numbers.first().unwrap_or(&0.0);
                let e = *numbers.get(1).unwrap_or(&1.0);
                Ok(num(b.powf(e)))
            }
            "sqrt" => Ok(num(numbers.first().unwrap_or(&0.0).sqrt())),
            "subtract" | "sub" => {
                if numbers.len() >= 2 {
                    Ok(num(numbers[0] - numbers[1]))
                } else {
                    Err("subtract requires two arguments".to_string())
                }
            }
            "multiply" | "mul" => Ok(num(numbers.iter().product())),
            "divide" | "div" => {
                if numbers.len() >= 2 && numbers[1] != 0.0 {
                    Ok(num(numbers[0] / numbers[1]))
                } else {
                    Err("divide requires two numbers with non-zero divisor".to_string())
                }
            }
            "mod" | "rem" => {
                if numbers.len() >= 2 && numbers[1] != 0.0 {
                    Ok(num(numbers[0] % numbers[1]))
                } else {
                    Err("mod requires two numbers with non-zero divisor".to_string())
                }
            }
            _ => Err(format!("Unsupported math operation: {}", operation)),
        }
    }
}
