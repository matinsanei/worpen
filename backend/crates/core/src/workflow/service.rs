use crate::services::DynamicRouteService;
use crate::workflow::compiler::WorkflowCompiler;
use crate::workflow::durable::{DurableRunStore, DurableWorkflowEngine, WorkflowRun};
use crate::workflow::validator::WorkflowValidator;
use proto::models::{
    ExecutionMode, HttpMethod, RouteDefinition, RouteType, WorkflowOpV1, WorkflowSpecV1,
    WorkflowStepV1,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct WorkflowService {
    data_dir: String,
    route_service: Arc<DynamicRouteService>,
}

impl WorkflowService {
    pub fn new(data_dir: String, route_service: Arc<DynamicRouteService>) -> Self {
        Self {
            data_dir,
            route_service,
        }
    }

    fn workflows_dir(&self) -> PathBuf {
        Path::new(&self.data_dir).join("workflows")
    }

    /// Load all existing workflows from data directory at startup
    pub async fn load_persisted_workflows(&self) -> Result<usize, String> {
        let dir = self.workflows_dir();
        if !dir.exists() {
            let _ = fs::create_dir_all(&dir);
            return Ok(0);
        }

        let mut loaded = 0;
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let current_file = entry.path().join("current.yaml");
                    if current_file.exists() {
                        if let Ok(content) = fs::read_to_string(&current_file) {
                            if let Ok(spec) = serde_yaml::from_str::<WorkflowSpecV1>(&content) {
                                let _ = self.sync_route_service(&spec).await;
                                loaded += 1;
                            }
                        }
                    }
                }
            }
        }
        Ok(loaded)
    }

    /// Register a new WorkflowSpecV1.
    /// Performs strict validation, saves an immutable version, updates current pointer,
    /// and synchronizes with the dynamic route engine.
    pub async fn register_workflow(&self, raw_content: &str) -> Result<WorkflowSpecV1, String> {
        // 1. Parse YAML (or JSON)
        let spec: WorkflowSpecV1 = serde_yaml::from_str(raw_content)
            .map_err(|e| format!("Failed to parse WorkflowSpecV1: {}", e))?;

        // 2. Strict validation (name, schema, undefined variables, loops, policy)
        WorkflowValidator::validate(&spec)?;

        // 3. Immutable storage
        let wf_dir = self.workflows_dir().join(&spec.name);
        fs::create_dir_all(&wf_dir)
            .map_err(|e| format!("Failed to create workflow directory: {}", e))?;

        let version_file = wf_dir.join(format!("v{}.yaml", spec.version));
        let yaml_string = serde_yaml::to_string(&spec)
            .map_err(|e| format!("Failed to serialize workflow: {}", e))?;

        // Save immutable version
        fs::write(&version_file, &yaml_string)
            .map_err(|e| format!("Failed to write version file: {}", e))?;

        // Update current.yaml pointer
        let current_file = wf_dir.join("current.yaml");
        fs::write(&current_file, &yaml_string)
            .map_err(|e| format!("Failed to write current.yaml: {}", e))?;

        // 4. Synchronize with route engine
        self.sync_route_service(&spec).await?;

        Ok(spec)
    }

    /// Rollback a workflow to a specific immutable version
    pub async fn rollback_workflow(
        &self,
        name: &str,
        target_version: &str,
    ) -> Result<WorkflowSpecV1, String> {
        let wf_dir = self.workflows_dir().join(name);
        let version_file = wf_dir.join(format!("v{}.yaml", target_version));

        if !version_file.exists() {
            return Err(format!(
                "Workflow '{}' version '{}' not found",
                name, target_version
            ));
        }

        let content = fs::read_to_string(&version_file)
            .map_err(|e| format!("Failed to read version file: {}", e))?;

        let spec: WorkflowSpecV1 = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse workflow version: {}", e))?;

        // Update current.yaml
        let current_file = wf_dir.join("current.yaml");
        fs::write(&current_file, &content)
            .map_err(|e| format!("Failed to update current.yaml: {}", e))?;

        // Sync with route engine
        self.sync_route_service(&spec).await?;

        Ok(spec)
    }

    /// Get current workflow spec by name
    pub fn get_workflow(&self, name: &str) -> Option<WorkflowSpecV1> {
        let current_file = self.workflows_dir().join(name).join("current.yaml");
        if !current_file.exists() {
            return None;
        }
        let content = fs::read_to_string(&current_file).ok()?;
        serde_yaml::from_str(&content).ok()
    }

    /// List all registered workflows
    pub fn list_workflows(&self) -> Vec<WorkflowSpecV1> {
        let mut list = Vec::new();
        let dir = self.workflows_dir();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let current = entry.path().join("current.yaml");
                    if current.exists() {
                        if let Ok(content) = fs::read_to_string(current) {
                            if let Ok(spec) = serde_yaml::from_str(&content) {
                                list.push(spec);
                            }
                        }
                    }
                }
            }
        }
        list
    }

    /// List all immutable versions for a workflow
    pub fn list_versions(&self, name: &str) -> Vec<String> {
        let mut versions = Vec::new();
        let wf_dir = self.workflows_dir().join(name);
        if let Ok(entries) = fs::read_dir(wf_dir) {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                if fname.starts_with('v') && fname.ends_with(".yaml") {
                    let ver = fname.trim_start_matches('v').trim_end_matches(".yaml");
                    versions.push(ver.to_string());
                }
            }
        }
        versions.sort();
        versions
    }

    /// Ensure the durable execution engine and SQLite runs table are initialized
    pub async fn ensure_durable_engine(&self) -> Result<Arc<DurableWorkflowEngine>, String> {
        let pool = self
            .route_service
            .get_db_pool()
            .ok_or_else(|| "Database pool not configured on DynamicRouteService".to_string())?;

        let store = Arc::new(DurableRunStore::new(pool.clone()));
        store.init().await?;
        let engine = DurableWorkflowEngine::new(store, pool, self.route_service.secret_store());
        Ok(Arc::new(engine))
    }

    /// Start a durable workflow run with SQLite persistence and approval gate support
    pub async fn start_run(
        &self,
        name: &str,
        payload: Option<Value>,
        mode: ExecutionMode,
    ) -> Result<WorkflowRun, String> {
        let spec = self
            .get_workflow(name)
            .ok_or_else(|| format!("Workflow '{}' not found", name))?;
        let engine = self.ensure_durable_engine().await?;
        engine.start_run(&spec, payload, mode).await
    }

    /// Retrieve a durable run by ID
    pub async fn get_run(&self, run_id: &str) -> Result<Option<WorkflowRun>, String> {
        let engine = self.ensure_durable_engine().await?;
        engine.store().get_run(run_id).await
    }

    /// List recent durable runs
    pub async fn list_runs(&self, limit: usize) -> Result<Vec<WorkflowRun>, String> {
        let engine = self.ensure_durable_engine().await?;
        engine.store().list_runs(limit).await
    }

    /// List all runs waiting for human approval
    pub async fn list_pending_approvals(&self) -> Result<Vec<WorkflowRun>, String> {
        let engine = self.ensure_durable_engine().await?;
        engine.store().list_pending_approvals().await
    }

    /// Approve a pending approval gate and resume execution
    pub async fn approve_run(
        &self,
        run_id: &str,
        approver: &str,
        reason: &str,
    ) -> Result<WorkflowRun, String> {
        let engine = self.ensure_durable_engine().await?;
        let run = engine
            .store()
            .get_run(run_id)
            .await?
            .ok_or_else(|| format!("Run '{}' not found", run_id))?;
        let spec = self
            .get_workflow(&run.workflow_name)
            .ok_or_else(|| format!("Workflow '{}' not found", run.workflow_name))?;
        engine.approve_run(&spec, run_id, approver, reason).await
    }

    /// Reject a pending approval gate
    pub async fn reject_run(
        &self,
        run_id: &str,
        rejecter: &str,
        reason: &str,
    ) -> Result<WorkflowRun, String> {
        let engine = self.ensure_durable_engine().await?;
        engine.reject_run(run_id, rejecter, reason).await
    }

    /// Cancel a running or waiting workflow run
    pub async fn cancel_run(&self, run_id: &str) -> Result<WorkflowRun, String> {
        let engine = self.ensure_durable_engine().await?;
        engine.cancel_run(run_id).await
    }

    /// Execute workflow in Live or DryRun mode
    pub async fn execute(
        &self,
        name: &str,
        payload: Option<Value>,
        mode: ExecutionMode,
    ) -> Result<Value, String> {
        let spec = self
            .get_workflow(name)
            .ok_or_else(|| format!("Workflow '{}' not found", name))?;

        let has_approval = spec
            .steps
            .iter()
            .any(|s| matches!(s.op, WorkflowOpV1::Approval { .. }));

        if has_approval && mode == ExecutionMode::Live {
            let run = self.start_run(name, payload, mode).await?;
            if let Some(res) = run.result {
                return Ok(res);
            }
            return Ok(serde_json::to_value(&run).unwrap_or_default());
        }

        let result = match mode {
            ExecutionMode::Live => {
                // Execute via dynamic route engine (VM with real DB & network)
                self.route_service
                    .execute_route(&spec.name, payload, HashMap::new(), HashMap::new())
                    .await
            }
            ExecutionMode::DryRun => {
                // Execute simulation with mock side-effects
                self.execute_dry_run(&spec, payload)
            }
        };

        result
            .map(|val| {
                self.route_service
                    .secret_store()
                    .redaction()
                    .redact_value(&val)
            })
            .map_err(|e| self.route_service.secret_store().redaction().redact_str(&e))
    }

    /// Dry-run simulation: executes variables, math, and transforms, but mocks side-effects (sql, http, approval)
    fn execute_dry_run(
        &self,
        spec: &WorkflowSpecV1,
        payload: Option<Value>,
    ) -> Result<Value, String> {
        let mut memory = HashMap::new();

        // 1. Inject payload into memory
        if let Some(Value::Object(map)) = payload {
            for (k, v) in map {
                memory.insert(k, v);
            }
        }

        // 2. Execute steps in simulation mode
        for step in &spec.steps {
            Self::simulate_step(step, &mut memory)?;
        }

        // 3. Resolve and format return value
        let return_val = Self::interpolate_value(&spec.r#return.value, &memory);
        Ok(json!({
            "_worpen_mode": "dry_run",
            "status": spec.r#return.status,
            "result": return_val,
        }))
    }

    fn simulate_step(
        step: &WorkflowStepV1,
        memory: &mut HashMap<String, Value>,
    ) -> Result<(), String> {
        match &step.op {
            WorkflowOpV1::Sql {
                query,
                args,
                output_var,
            } => {
                // Mock SQL: do not touch DB; produce dry-run record
                let bound_args: Vec<Value> = args
                    .iter()
                    .map(|a| Self::interpolate_value(a, memory))
                    .collect();
                let mock_rows = json!([{
                    "_worpen_dry_run": true,
                    "simulated_query": query,
                    "bound_args": bound_args,
                }]);
                memory.insert(output_var.clone(), mock_rows);
            }
            WorkflowOpV1::Http {
                method,
                url,
                output_var,
                ..
            } => {
                // Mock HTTP: do not send packet; produce dry-run record
                let resolved_url = Self::interpolate_string(url, memory);
                let mock_res = json!({
                    "_worpen_dry_run": true,
                    "simulated_http": true,
                    "method": method,
                    "url": resolved_url,
                    "idempotency_key": step.idempotency_key.as_ref().map(|k| Self::interpolate_string(k, memory)),
                });
                if let Some(out) = output_var {
                    memory.insert(out.clone(), mock_res);
                }
            }
            WorkflowOpV1::Transform { expr, output_var } => {
                // Evaluate expression or interpolation
                let evaluated = Self::evaluate_transform_expr(expr, memory)?;
                memory.insert(output_var.clone(), evaluated);
            }
            WorkflowOpV1::Set { var, value } => {
                let evaluated = Self::interpolate_value(value, memory);
                memory.insert(var.clone(), evaluated);
            }
            WorkflowOpV1::If {
                condition: _, then, ..
            } => {
                for s in then {
                    Self::simulate_step(s, memory)?;
                }
            }
            WorkflowOpV1::Switch { cases, default, .. } => {
                if let Some(first_case) = cases.values().next() {
                    for s in first_case {
                        Self::simulate_step(s, memory)?;
                    }
                } else if let Some(def_steps) = default {
                    for s in def_steps {
                        Self::simulate_step(s, memory)?;
                    }
                }
            }
            WorkflowOpV1::Loop {
                as_var,
                body,
                max_iterations,
                ..
            } => {
                // Run 1 simulated iteration
                if *max_iterations > 0 {
                    memory.insert(as_var.clone(), json!({"_worpen_dry_run": true, "index": 0}));
                    for s in body {
                        Self::simulate_step(s, memory)?;
                    }
                }
            }
            WorkflowOpV1::Approval { summary, .. } => {
                let resolved_summary = Self::interpolate_string(summary, memory);
                memory.insert(
                    format!("{}_approved", step.id),
                    json!({
                        "_worpen_dry_run": true,
                        "status": "simulated_approved",
                        "summary": resolved_summary
                    }),
                );
            }
            WorkflowOpV1::TryCatch { r#try, .. } => {
                for s in r#try {
                    Self::simulate_step(s, memory)?;
                }
            }
            WorkflowOpV1::Parallel { branches, .. } => {
                for branch in branches {
                    for s in branch {
                        Self::simulate_step(s, memory)?;
                    }
                }
            }
            WorkflowOpV1::Fail { message, .. } => {
                return Err(format!("Dry-run failure simulation: {}", message));
            }
        }
        Ok(())
    }

    fn evaluate_transform_expr(
        expr: &str,
        memory: &HashMap<String, Value>,
    ) -> Result<Value, String> {
        // If expr is a template or arithmetic expression, evaluate it
        let resolved = Self::interpolate_string(expr, memory);
        if let Ok(v) = serde_json::from_str::<Value>(&resolved) {
            Ok(v)
        } else {
            Ok(Value::String(resolved))
        }
    }

    fn interpolate_string(template: &str, memory: &HashMap<String, Value>) -> String {
        let re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        re.replace_all(template, |caps: &regex::Captures| {
            if let Some(m) = caps.get(1) {
                let key = m.as_str().trim();
                if let Some(val) = memory.get(key) {
                    match val {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => serde_json::to_string(val).unwrap_or_default(),
                    }
                } else {
                    format!("{{{{{}}}}}", key)
                }
            } else {
                String::new()
            }
        })
        .to_string()
    }

    fn interpolate_value(val: &Value, memory: &HashMap<String, Value>) -> Value {
        match val {
            Value::String(s) => {
                // If it is an exact single variable "{{var}}", return the direct value without stringifying
                let trimmed = s.trim();
                if trimmed.starts_with("{{")
                    && trimmed.ends_with("}}")
                    && !trimmed[2..trimmed.len() - 2].contains("{{")
                {
                    let key = trimmed[2..trimmed.len() - 2].trim();
                    if let Some(direct) = memory.get(key) {
                        return direct.clone();
                    }
                }
                Value::String(Self::interpolate_string(s, memory))
            }
            Value::Array(arr) => Value::Array(
                arr.iter()
                    .map(|item| Self::interpolate_value(item, memory))
                    .collect(),
            ),
            Value::Object(map) => {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    new_map.insert(k.clone(), Self::interpolate_value(v, memory));
                }
                Value::Object(new_map)
            }
            other => other.clone(),
        }
    }

    /// Compile and synchronize a WorkflowSpecV1 as a RouteDefinition in DynamicRouteService
    pub async fn sync_route_service(&self, spec: &WorkflowSpecV1) -> Result<(), String> {
        let compiled_logic = WorkflowCompiler::compile(spec);

        let route = RouteDefinition {
            id: spec.name.clone(),
            name: spec.name.clone(),
            description: spec.description.clone(),
            path: format!("/api/workflows/{}", spec.name),
            method: HttpMethod::POST,
            route_type: RouteType::Http,
            logic: compiled_logic,
            ws_hooks: None,
            parameters: vec![],
            response_schema: None,
            auth_required: false,
            rate_limit: None,
            enabled: true,
            version: spec.version.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            created_by: "workflow-engine".to_string(),
            sandbox: spec.policy.clone(),
        };

        self.route_service.register_route(route).await?;
        Ok(())
    }
}
