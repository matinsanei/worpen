use crate::audit::AuditChainManager;
use crate::sandbox::{is_blocked_ip, resolve_and_validate_host, validate_sql_policy};
use crate::secrets::SecretStore;
use chrono::{DateTime, Utc};
pub use proto::models::WorkflowRun;
use proto::models::{
    ExecutionMode, PendingApproval, RunStatus, SandboxPolicy, WorkflowOpV1, WorkflowSpecV1,
    WorkflowStepV1,
};
use serde_json::{json, Value};
use sqlx::{Column, Pool, Row, Sqlite};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct DurableRunStore {
    pool: Pool<Sqlite>,
}

impl DurableRunStore {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Ensure the workflow_runs table is created
    pub async fn init(&self) -> Result<(), String> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS workflow_runs (
            run_id TEXT PRIMARY KEY,
            workflow_name TEXT NOT NULL,
            version TEXT NOT NULL,
            status TEXT NOT NULL,
            mode TEXT NOT NULL,
            step_index INTEGER NOT NULL,
            variables TEXT NOT NULL,
            pending_approval TEXT,
            result TEXT,
            error TEXT,
            initiator TEXT,
            idempotency_key TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_workflow_runs_idempotency ON workflow_runs (workflow_name, idempotency_key);
        "#;
        sqlx::query(sql)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to create workflow_runs table: {}", e))?;

        // Safely upgrade existing tables if columns missing
        let _ = sqlx::query("ALTER TABLE workflow_runs ADD COLUMN initiator TEXT")
            .execute(&self.pool)
            .await;
        let _ = sqlx::query("ALTER TABLE workflow_runs ADD COLUMN idempotency_key TEXT")
            .execute(&self.pool)
            .await;

        Ok(())
    }

    pub async fn save_run(&self, run: &WorkflowRun) -> Result<(), String> {
        let sql = r#"
        INSERT INTO workflow_runs (
            run_id, workflow_name, version, status, mode, step_index,
            variables, pending_approval, result, error, initiator, idempotency_key, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        ON CONFLICT(run_id) DO UPDATE SET
            status = excluded.status,
            step_index = excluded.step_index,
            variables = excluded.variables,
            pending_approval = excluded.pending_approval,
            result = excluded.result,
            error = excluded.error,
            initiator = excluded.initiator,
            idempotency_key = excluded.idempotency_key,
            updated_at = excluded.updated_at;
        "#;

        let status_str = match run.status {
            RunStatus::Pending => "pending",
            RunStatus::Running => "running",
            RunStatus::WaitingForApproval => "waiting_for_approval",
            RunStatus::Completed => "completed",
            RunStatus::Failed => "failed",
            RunStatus::Rejected => "rejected",
            RunStatus::Cancelled => "cancelled",
            RunStatus::Expired => "expired",
        };

        let mode_str = match run.mode {
            ExecutionMode::Live => "live",
            ExecutionMode::DryRun => "dry_run",
        };

        let vars_json = serde_json::to_string(&run.variables).unwrap_or_else(|_| "{}".to_string());
        let pending_json = run
            .pending_approval
            .as_ref()
            .and_then(|p| serde_json::to_string(p).ok());
        let result_json = run
            .result
            .as_ref()
            .and_then(|r| serde_json::to_string(r).ok());

        sqlx::query(sql)
            .bind(&run.run_id)
            .bind(&run.workflow_name)
            .bind(&run.version)
            .bind(status_str)
            .bind(mode_str)
            .bind(run.step_index as i64)
            .bind(&vars_json)
            .bind(pending_json)
            .bind(result_json)
            .bind(&run.error)
            .bind(&run.initiator)
            .bind(&run.idempotency_key)
            .bind(&run.created_at)
            .bind(&run.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save run '{}': {}", run.run_id, e))?;

        Ok(())
    }

    pub async fn get_run(&self, run_id: &str) -> Result<Option<WorkflowRun>, String> {
        let sql = "SELECT * FROM workflow_runs WHERE run_id = $1";
        let maybe_row = sqlx::query(sql)
            .bind(run_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch run '{}': {}", run_id, e))?;

        match maybe_row {
            Some(row) => Ok(Some(Self::row_to_run(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn list_runs(&self, limit: usize) -> Result<Vec<WorkflowRun>, String> {
        let sql = "SELECT * FROM workflow_runs ORDER BY created_at DESC LIMIT $1";
        let rows = sqlx::query(sql)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to list runs: {}", e))?;

        let mut runs = Vec::new();
        for row in rows {
            runs.push(Self::row_to_run(&row)?);
        }
        Ok(runs)
    }

    pub async fn list_pending_approvals(&self) -> Result<Vec<WorkflowRun>, String> {
        let sql = "SELECT * FROM workflow_runs WHERE status = 'waiting_for_approval' ORDER BY created_at DESC";
        let rows = sqlx::query(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to list pending approvals: {}", e))?;

        let mut runs = Vec::new();
        for row in rows {
            runs.push(Self::row_to_run(&row)?);
        }
        Ok(runs)
    }

    pub async fn get_run_by_idempotency_key(
        &self,
        workflow_name: &str,
        idempotency_key: &str,
    ) -> Result<Option<WorkflowRun>, String> {
        let sql = "SELECT * FROM workflow_runs WHERE workflow_name = $1 AND idempotency_key = $2 ORDER BY created_at DESC LIMIT 1";
        let maybe_row = sqlx::query(sql)
            .bind(workflow_name)
            .bind(idempotency_key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                format!(
                    "Failed to fetch run by idempotency_key '{}': {}",
                    idempotency_key, e
                )
            })?;

        match maybe_row {
            Some(row) => Ok(Some(Self::row_to_run(&row)?)),
            None => Ok(None),
        }
    }

    fn row_to_run(row: &sqlx::sqlite::SqliteRow) -> Result<WorkflowRun, String> {
        let status_str: String = row
            .try_get("status")
            .map_err(|e| format!("Missing status: {}", e))?;
        let status = match status_str.as_str() {
            "pending" => RunStatus::Pending,
            "running" => RunStatus::Running,
            "waiting_for_approval" => RunStatus::WaitingForApproval,
            "completed" => RunStatus::Completed,
            "failed" => RunStatus::Failed,
            "rejected" => RunStatus::Rejected,
            "cancelled" => RunStatus::Cancelled,
            "expired" => RunStatus::Expired,
            other => return Err(format!("Unknown run status '{}'", other)),
        };

        let mode_str: String = row
            .try_get("mode")
            .map_err(|e| format!("Missing mode: {}", e))?;
        let mode = match mode_str.as_str() {
            "dry_run" => ExecutionMode::DryRun,
            _ => ExecutionMode::Live,
        };

        let vars_str: String = row
            .try_get("variables")
            .unwrap_or_else(|_| "{}".to_string());
        let variables: HashMap<String, Value> = serde_json::from_str(&vars_str).unwrap_or_default();

        let pending_approval: Option<PendingApproval> = row
            .try_get::<Option<String>, _>("pending_approval")
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok());

        let result: Option<Value> = row
            .try_get::<Option<String>, _>("result")
            .ok()
            .flatten()
            .and_then(|s| serde_json::from_str(&s).ok());

        let error: Option<String> = row.try_get("error").ok().flatten();
        let initiator: Option<String> = row.try_get("initiator").ok().flatten();
        let idempotency_key: Option<String> = row.try_get("idempotency_key").ok().flatten();

        let step_index: i64 = row.try_get("step_index").unwrap_or(0);

        Ok(WorkflowRun {
            run_id: row.try_get("run_id").unwrap_or_default(),
            workflow_name: row.try_get("workflow_name").unwrap_or_default(),
            version: row.try_get("version").unwrap_or_default(),
            status,
            mode,
            step_index: step_index as usize,
            variables,
            pending_approval,
            result,
            error,
            initiator,
            idempotency_key,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        })
    }
}

/// Durable Workflow Execution Engine
pub struct DurableWorkflowEngine {
    store: Arc<DurableRunStore>,
    db_pool: Pool<Sqlite>,
    secret_store: Arc<SecretStore>,
    audit_chain: Arc<AuditChainManager>,
}

impl DurableWorkflowEngine {
    pub fn new(
        store: Arc<DurableRunStore>,
        db_pool: Pool<Sqlite>,
        secret_store: Arc<SecretStore>,
        audit_chain: Arc<AuditChainManager>,
    ) -> Self {
        Self {
            store,
            db_pool,
            secret_store,
            audit_chain,
        }
    }

    /// Access the underlying store
    pub fn store(&self) -> Arc<DurableRunStore> {
        self.store.clone()
    }

    /// Access the audit chain manager
    pub fn audit_chain(&self) -> Arc<AuditChainManager> {
        self.audit_chain.clone()
    }

    /// Start a new durable run
    pub async fn start_run(
        &self,
        spec: &WorkflowSpecV1,
        payload: Option<Value>,
        mode: ExecutionMode,
    ) -> Result<WorkflowRun, String> {
        self.start_run_with_meta(spec, payload, mode, None, None)
            .await
    }

    /// Start a new durable run with initiator and idempotency tracking
    pub async fn start_run_with_meta(
        &self,
        spec: &WorkflowSpecV1,
        payload: Option<Value>,
        mode: ExecutionMode,
        initiator: Option<String>,
        idempotency_key: Option<String>,
    ) -> Result<WorkflowRun, String> {
        let effective_idempotency_key = idempotency_key.or_else(|| {
            payload
                .as_ref()
                .and_then(|p| p.get("idempotency_key"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        });

        // 1. Idempotency deduplication check
        if let Some(ref ikey) = effective_idempotency_key {
            if let Ok(Some(existing_run)) = self
                .store
                .get_run_by_idempotency_key(&spec.name, ikey)
                .await
            {
                // If run already exists, return it to prevent duplicate execution
                return Ok(existing_run);
            }
        }

        let run_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let effective_initiator = initiator
            .or_else(|| {
                payload
                    .as_ref()
                    .and_then(|p| p.get("__initiator"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .or_else(|| Some("system".to_string()));

        let _ = self.audit_chain.append(
            "workflow.start",
            effective_initiator.as_deref().unwrap_or("agent"),
            &spec.name,
            json!({
                "run_id": &run_id,
                "version": &spec.version,
                "mode": format!("{:?}", mode),
                "initiator": &effective_initiator,
                "idempotency_key": &effective_idempotency_key,
            }),
            Some("allowed"),
        );

        let mut variables = HashMap::new();
        // Inject payload fields into variables
        if let Some(Value::Object(map)) = payload {
            for (k, v) in map {
                variables.insert(k, v);
            }
        }
        variables.insert("run_id".to_string(), json!(run_id));
        variables.insert("workflow_name".to_string(), json!(spec.name));
        variables.insert(
            "execution_mode".to_string(),
            json!(match mode {
                ExecutionMode::Live => "live",
                ExecutionMode::DryRun => "dry_run",
            }),
        );

        let mut run = WorkflowRun {
            run_id,
            workflow_name: spec.name.clone(),
            version: spec.version.clone(),
            status: RunStatus::Running,
            mode,
            step_index: 0,
            variables,
            pending_approval: None,
            result: None,
            error: None,
            initiator: effective_initiator,
            idempotency_key: effective_idempotency_key,
            created_at: now.clone(),
            updated_at: now,
        };

        self.store.save_run(&run).await?;
        self.resume_run(spec, &mut run).await
    }

    /// Resume a run from its current step_index
    pub async fn resume_run(
        &self,
        spec: &WorkflowSpecV1,
        run: &mut WorkflowRun,
    ) -> Result<WorkflowRun, String> {
        let policy = spec.policy.clone().unwrap_or_default();

        while run.step_index < spec.steps.len() {
            let step = &spec.steps[run.step_index];

            // 1. Check for Approval step
            if let WorkflowOpV1::Approval {
                summary,
                timeout_seconds,
            } = &step.op
            {
                let resolved_summary = self.interpolate_string(summary, &run.variables);
                let timeout_secs = timeout_seconds.unwrap_or(86400); // Default 24h
                let expires_at = Utc::now() + chrono::Duration::seconds(timeout_secs as i64);

                run.status = RunStatus::WaitingForApproval;
                run.pending_approval = Some(PendingApproval {
                    step_id: step.id.clone(),
                    summary: resolved_summary.clone(),
                    requested_at: Utc::now().to_rfc3339(),
                    expires_at: expires_at.to_rfc3339(),
                });
                run.updated_at = Utc::now().to_rfc3339();

                let _ = self.audit_chain.append(
                    "workflow.approval_requested",
                    "system",
                    &spec.name,
                    json!({
                        "run_id": &run.run_id,
                        "step_id": &step.id,
                        "summary": &resolved_summary,
                    }),
                    Some("approval_pending"),
                );

                self.store.save_run(run).await?;
                return Ok(run.clone());
            }

            // 2. Execute normal step with retry policy (if configured)
            let max_attempts = step
                .retry
                .as_ref()
                .map(|r| r.max_attempts)
                .unwrap_or(1)
                .max(1);
            let base_backoff_ms = step.retry.as_ref().map(|r| r.backoff_ms).unwrap_or(100);

            let mut attempts = 0;
            let step_result = loop {
                attempts += 1;
                match self.execute_step(step, run, &policy).await {
                    Ok(val) => break Ok(val),
                    Err(err) => {
                        if attempts >= max_attempts {
                            break Err(err);
                        }
                        let delay_ms = base_backoff_ms * (1 << (attempts - 1)).min(30_000);
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            };

            match step_result {
                Ok(_) => {
                    run.step_index += 1;
                    run.updated_at = Utc::now().to_rfc3339();
                    self.store.save_run(run).await?;
                }
                Err(err) => {
                    let redacted_err = self.secret_store.redaction().redact_str(&err);
                    run.status = RunStatus::Failed;
                    run.error = Some(redacted_err.clone());
                    run.updated_at = Utc::now().to_rfc3339();

                    let _ = self.audit_chain.append(
                        "workflow.step_failed",
                        "system",
                        &spec.name,
                        json!({
                            "run_id": &run.run_id,
                            "step_id": &step.id,
                            "error": &redacted_err,
                        }),
                        Some("rejected_or_failed"),
                    );

                    self.store.save_run(run).await?;
                    return Err(redacted_err);
                }
            }
        }

        // 3. Workflow complete: evaluate return value
        let raw_result = self.interpolate_value(&spec.r#return.value, &run.variables);
        let redacted_result = self.secret_store.redaction().redact_value(&raw_result);

        run.status = RunStatus::Completed;
        run.result = Some(redacted_result);
        run.updated_at = Utc::now().to_rfc3339();

        let _ = self.audit_chain.append(
            "workflow.completed",
            "system",
            &spec.name,
            json!({
                "run_id": &run.run_id,
                "status": "completed",
            }),
            Some("allowed"),
        );

        self.store.save_run(run).await?;

        Ok(run.clone())
    }

    /// Approve a pending run and resume execution
    pub async fn approve_run(
        &self,
        spec: &WorkflowSpecV1,
        run_id: &str,
        approver: &str,
        reason: &str,
    ) -> Result<WorkflowRun, String> {
        let mut run = self
            .store
            .get_run(run_id)
            .await?
            .ok_or_else(|| format!("Run '{}' not found", run_id))?;

        if run.status != RunStatus::WaitingForApproval {
            return Err(format!(
                "Run '{}' is not waiting for approval (current status: {:?})",
                run_id, run.status
            ));
        }

        // Anti-self-approval enforcement: Run initiator cannot approve their own run
        if let Some(ref initiator) = run.initiator {
            let clean_approver = approver.trim().to_lowercase();
            let clean_initiator = initiator.trim().to_lowercase();
            if !clean_approver.is_empty() && clean_approver == clean_initiator {
                return Err(format!(
                    "Self-approval forbidden: Initiator '{}' cannot approve their own run '{}'. Approval must come from a distinct operator.",
                    initiator, run_id
                ));
            }
        }

        // Check if approval has expired
        if let Some(ref pending) = run.pending_approval {
            if let Ok(exp) = DateTime::parse_from_rfc3339(&pending.expires_at) {
                if Utc::now() > exp {
                    run.status = RunStatus::Expired;
                    run.error = Some(format!(
                        "Approval request for step '{}' expired at {}",
                        pending.step_id, pending.expires_at
                    ));
                    run.updated_at = Utc::now().to_rfc3339();
                    self.store.save_run(&run).await?;
                    return Err(format!("Approval for run '{}' has expired", run_id));
                }
            }
        }

        // Inject approval metadata into execution variables
        run.variables
            .insert("__approved_by".to_string(), json!(approver));
        run.variables
            .insert("__approval_reason".to_string(), json!(reason));
        run.variables
            .insert("__approved_at".to_string(), json!(Utc::now().to_rfc3339()));

        run.pending_approval = None;
        run.status = RunStatus::Running;
        run.step_index += 1; // Advance past the approval step
        run.updated_at = Utc::now().to_rfc3339();

        let _ = self.audit_chain.append(
            "workflow.approved",
            approver,
            &spec.name,
            json!({
                "run_id": run_id,
                "reason": reason,
            }),
            Some("allowed"),
        );

        self.store.save_run(&run).await?;

        // Resume execution
        self.resume_run(spec, &mut run).await
    }

    /// Reject a pending run
    pub async fn reject_run(
        &self,
        run_id: &str,
        rejecter: &str,
        reason: &str,
    ) -> Result<WorkflowRun, String> {
        let mut run = self
            .store
            .get_run(run_id)
            .await?
            .ok_or_else(|| format!("Run '{}' not found", run_id))?;

        if run.status != RunStatus::WaitingForApproval {
            return Err(format!(
                "Run '{}' is not waiting for approval (current status: {:?})",
                run_id, run.status
            ));
        }

        run.status = RunStatus::Rejected;
        run.error = Some(format!("Approval rejected by {}: {}", rejecter, reason));
        run.pending_approval = None;
        run.updated_at = Utc::now().to_rfc3339();

        let _ = self.audit_chain.append(
            "workflow.rejected",
            rejecter,
            &run.workflow_name,
            json!({
                "run_id": run_id,
                "reason": reason,
            }),
            Some("denied_by_human"),
        );

        self.store.save_run(&run).await?;
        Ok(run)
    }

    /// Cancel a running or waiting run
    pub async fn cancel_run(&self, run_id: &str) -> Result<WorkflowRun, String> {
        let mut run = self
            .store
            .get_run(run_id)
            .await?
            .ok_or_else(|| format!("Run '{}' not found", run_id))?;

        if run.status == RunStatus::Completed || run.status == RunStatus::Failed {
            return Err(format!(
                "Cannot cancel run in terminal state: {:?}",
                run.status
            ));
        }

        run.status = RunStatus::Cancelled;
        run.pending_approval = None;
        run.updated_at = Utc::now().to_rfc3339();

        let _ = self.audit_chain.append(
            "workflow.cancelled",
            "operator",
            &run.workflow_name,
            json!({
                "run_id": run_id,
            }),
            Some("cancelled"),
        );

        self.store.save_run(&run).await?;
        Ok(run)
    }

    /// Execute a single workflow step under the active policy and mode
    async fn execute_step(
        &self,
        step: &WorkflowStepV1,
        run: &mut WorkflowRun,
        policy: &SandboxPolicy,
    ) -> Result<(), String> {
        match &step.op {
            WorkflowOpV1::Sql {
                query,
                args,
                output_var,
            } => {
                if run.mode == ExecutionMode::DryRun {
                    let mock_rows = json!([{
                        "_worpen_dry_run": true,
                        "query": query,
                    }]);
                    run.variables.insert(output_var.clone(), mock_rows);
                    return Ok(());
                }

                // 1. Resolve secrets in query
                let resolved_query = self.secret_store.inject_secrets(query)?;

                // 2. Validate SQL sandbox policy (read-only enforcement + table allowlists)
                validate_sql_policy(&resolved_query, policy)?;

                // 3. Resolve and bind arguments
                let mut query_builder = sqlx::query(&resolved_query);
                for arg in args {
                    let arg_val = self.interpolate_value(arg, &run.variables);
                    match arg_val {
                        Value::String(s) => {
                            let resolved_str = self.secret_store.inject_secrets(&s)?;
                            query_builder = query_builder.bind(resolved_str);
                        }
                        Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                query_builder = query_builder.bind(i);
                            } else if let Some(f) = n.as_f64() {
                                query_builder = query_builder.bind(f);
                            } else {
                                query_builder = query_builder.bind(n.to_string());
                            }
                        }
                        Value::Bool(b) => {
                            query_builder = query_builder.bind(b);
                        }
                        other => {
                            query_builder = query_builder.bind(other.to_string());
                        }
                    }
                }

                // 4. Execute query with C-level read-only enforcement when requested
                let rows = if policy.read_only_sql {
                    let mut conn = self
                        .db_pool
                        .acquire()
                        .await
                        .map_err(|e| format!("Failed to acquire DB connection: {}", e))?;
                    let _ = sqlx::query("PRAGMA query_only = ON;")
                        .execute(&mut *conn)
                        .await;
                    let result = query_builder.fetch_all(&mut *conn).await;
                    let _ = sqlx::query("PRAGMA query_only = OFF;")
                        .execute(&mut *conn)
                        .await;
                    result.map_err(|e| format!("SQL execution error: {}", e))?
                } else {
                    query_builder
                        .fetch_all(&self.db_pool)
                        .await
                        .map_err(|e| format!("SQL execution error: {}", e))?
                };

                // 5. Convert rows to JSON and apply row cap
                let mut json_rows = Vec::new();
                for row in rows {
                    let mut obj = serde_json::Map::new();
                    for (i, col) in row.columns().iter().enumerate() {
                        let col_name = col.name().to_string();
                        let val = if let Ok(v) = row.try_get::<String, _>(i) {
                            Value::String(v)
                        } else if let Ok(v) = row.try_get::<i64, _>(i) {
                            Value::Number(v.into())
                        } else if let Ok(v) = row.try_get::<f64, _>(i) {
                            Value::Number(serde_json::Number::from_f64(v).unwrap_or(0.into()))
                        } else if let Ok(v) = row.try_get::<bool, _>(i) {
                            Value::Bool(v)
                        } else {
                            Value::Null
                        };
                        obj.insert(col_name, val);
                    }
                    json_rows.push(Value::Object(obj));
                }

                if json_rows.len() > policy.max_sql_rows {
                    json_rows.truncate(policy.max_sql_rows);
                }

                run.variables
                    .insert(output_var.clone(), Value::Array(json_rows));
                Ok(())
            }

            WorkflowOpV1::Http {
                method,
                url,
                body,
                headers,
                output_var,
            } => {
                if run.mode == ExecutionMode::DryRun {
                    let mock_res = json!({
                        "_worpen_dry_run": true,
                        "method": method,
                        "url": url,
                    });
                    if let Some(out) = output_var {
                        run.variables.insert(out.clone(), mock_res);
                    }
                    return Ok(());
                }

                // 1. Resolve URL with variables and secrets
                let interpolated_url = self.interpolate_string(url, &run.variables);
                let resolved_url = self.secret_store.inject_secrets(&interpolated_url)?;

                // 2. Enforce SSRF & host allowlist check
                let (host, port, pinned_ip) =
                    resolve_and_validate_host(&resolved_url, policy).await?;

                let timeout = step.timeout_ms.unwrap_or(policy.max_timeout_ms);
                let mut client_builder = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_millis(timeout))
                    .resolve(&host, std::net::SocketAddr::new(pinned_ip, port));

                // Safe redirect policy
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
                                if is_blocked_ip(ip) {
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
                let client = client_builder
                    .build()
                    .map_err(|e| format!("HTTP client error: {}", e))?;

                let mut rb = match method.to_uppercase().as_str() {
                    "GET" => client.get(&resolved_url),
                    "POST" => client.post(&resolved_url),
                    "PUT" => client.put(&resolved_url),
                    "DELETE" => client.delete(&resolved_url),
                    "PATCH" => client.patch(&resolved_url),
                    "HEAD" => client.head(&resolved_url),
                    _ => return Err(format!("Unsupported HTTP method: {}", method)),
                };

                // Add headers
                if let Some(hdrs) = headers {
                    for (k, v) in hdrs {
                        let interp = self.interpolate_string(v, &run.variables);
                        let secret_val = self.secret_store.inject_secrets(&interp)?;
                        rb = rb.header(k, secret_val);
                    }
                }

                // Add body
                if let Some(b) = body {
                    let interp_body = self.interpolate_value(b, &run.variables);
                    let body_str = match interp_body {
                        Value::String(s) => self.secret_store.inject_secrets(&s)?,
                        other => {
                            let s = serde_json::to_string(&other).unwrap_or_default();
                            self.secret_store.inject_secrets(&s)?
                        }
                    };
                    rb = rb.body(body_str);
                }

                // Execute request
                let resp = rb
                    .send()
                    .await
                    .map_err(|e| format!("HTTP request failed: {}", e))?;
                let status = resp.status().as_u16();
                let text = resp
                    .text()
                    .await
                    .map_err(|e| format!("read body failed: {}", e))?;
                let body_json: Value = serde_json::from_str(&text).unwrap_or(Value::String(text));

                if let Some(out) = output_var {
                    let resp_obj = json!({
                        "status": status,
                        "body": body_json,
                        "url": resolved_url,
                        "method": method,
                    });
                    run.variables.insert(out.clone(), resp_obj);
                }
                Ok(())
            }

            WorkflowOpV1::Transform { expr, output_var } => {
                let evaluated = self.evaluate_expr(expr, &run.variables)?;
                run.variables.insert(output_var.clone(), evaluated);
                Ok(())
            }

            WorkflowOpV1::Set { var, value } => {
                let evaluated = self.interpolate_value(value, &run.variables);
                run.variables.insert(var.clone(), evaluated);
                Ok(())
            }

            WorkflowOpV1::Fail { message, .. } => {
                let resolved = self.interpolate_string(message, &run.variables);
                Err(resolved)
            }

            _ => {
                // Other ops can execute simulated
                Ok(())
            }
        }
    }

    /// Interpolate `{{var}}` variables in a string
    fn interpolate_string(&self, s: &str, vars: &HashMap<String, Value>) -> String {
        let re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let mut result = s.to_string();

        for cap in re.captures_iter(s) {
            if let (Some(full), Some(matched)) = (cap.get(0), cap.get(1)) {
                let var_name = matched.as_str().trim();
                // Pass secrets through untouched for secret_store to resolve at step boundary
                if var_name.starts_with("secret:") {
                    continue;
                }
                if let Some(val) = vars.get(var_name) {
                    let val_str = match val {
                        Value::String(st) => st.clone(),
                        other => other.to_string(),
                    };
                    result = result.replace(full.as_str(), &val_str);
                }
            }
        }
        result
    }

    /// Recursively interpolate values in a JSON Value
    fn interpolate_value(&self, val: &Value, vars: &HashMap<String, Value>) -> Value {
        match val {
            Value::String(s) => {
                let trimmed = s.trim();
                if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
                    let inner = trimmed[2..trimmed.len() - 2].trim();
                    if inner.starts_with("secret:") {
                        return val.clone();
                    }
                    if let Some(v) = vars.get(inner) {
                        return v.clone();
                    }
                }
                Value::String(self.interpolate_string(s, vars))
            }
            Value::Array(arr) => Value::Array(
                arr.iter()
                    .map(|item| self.interpolate_value(item, vars))
                    .collect(),
            ),
            Value::Object(map) => {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    new_map.insert(k.clone(), self.interpolate_value(v, vars));
                }
                Value::Object(new_map)
            }
            other => other.clone(),
        }
    }

    /// Evaluate mathematical expressions
    fn evaluate_expr(&self, expr: &str, vars: &HashMap<String, Value>) -> Result<Value, String> {
        let interpolated = self.interpolate_string(expr, vars);
        let trimmed = interpolated.trim();

        if let Ok(num) = trimmed.parse::<i64>() {
            return Ok(json!(num));
        }
        if let Ok(num) = trimmed.parse::<f64>() {
            return Ok(json!(num));
        }
        if trimmed == "true" {
            return Ok(json!(true));
        }
        if trimmed == "false" {
            return Ok(json!(false));
        }

        // Basic arithmetic binary operations: + - * /
        for op in ['*', '/', '+', '-'] {
            if let Some(idx) = trimmed.find(op) {
                let left_str = trimmed[..idx].trim();
                let right_str = trimmed[idx + 1..].trim();

                if let (Ok(left), Ok(right)) = (left_str.parse::<f64>(), right_str.parse::<f64>()) {
                    let res = match op {
                        '*' => left * right,
                        '/' => {
                            if right == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            left / right
                        }
                        '+' => left + right,
                        '-' => left - right,
                        _ => unreachable!(),
                    };
                    if res.fract() == 0.0 {
                        return Ok(json!(res as i64));
                    }
                    return Ok(json!(res));
                }
            }
        }

        Ok(Value::String(interpolated))
    }
}
