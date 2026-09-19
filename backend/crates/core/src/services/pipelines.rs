//! Real pipelines: ordered, multi-stage logic programs executed by the Worpen engine.
//!
//! - Definitions persist to `{data_dir}/pipelines/*.json`.
//! - Stages run sequentially in ONE shared variable scope:
//!   `payload` (initial input), `last` (previous stage result),
//!   `stages.<name>` (each completed stage result), `stage`, `stage_index`.
//! - A failing stage stops the run unless `allow_failure` is set;
//!   remaining stages are recorded as `Skipped`.
//! - Every run is audited (`via="pipeline"`) and kept in a bounded history ring.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use proto::models::{
    DynamicRouteExecutionContext, FunctionDefinition, LoopControl, PipelineDefinition,
    PipelineListItem, PipelineRunResult, PipelineStageResult, PipelineStageView,
    RegisterPipelineRequest, SandboxPolicy, StageRunStatus,
};
use serde_json::Value;
use super::dynamic_routes::execute_logic_extended;
use super::dynamic_routes::service::DynamicRouteService;

pub const PIPELINE_RUN_HISTORY_CAP: usize = 50;

pub struct PipelineService {
    pipelines: Arc<std::sync::RwLock<HashMap<String, PipelineDefinition>>>,
    runs: Arc<std::sync::RwLock<VecDeque<PipelineRunResult>>>,
    data_dir: String,
    executor: Option<Arc<DynamicRouteService>>,
}

impl Default for PipelineService {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineService {
    pub fn new() -> Self {
        let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| {
            let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            if current_dir.ends_with("backend") || current_dir.join("crates").exists() {
                "data".to_string()
            } else {
                "backend/data".to_string()
            }
        });
        Self::with_data_dir(data_dir)
    }

    pub fn with_data_dir(data_dir: String) -> Self {
        let service = Self {
            pipelines: Arc::new(std::sync::RwLock::new(HashMap::new())),
            runs: Arc::new(std::sync::RwLock::new(VecDeque::new())),
            data_dir,
            executor: None,
        };
        let _ = service.load_persisted();
        service
    }

    /// Production constructor: shares the route executor (inlining + globals + audit).
    pub fn with_executor(executor: Arc<DynamicRouteService>) -> Self {
        let mut service = Self::new();
        service.executor = Some(executor);
        service
    }

    // -- definitions -------------------------------------------------------

    pub async fn define_pipeline(&self, req: RegisterPipelineRequest) -> Result<String, String> {
        self.validate(&req)?;
        let executor = self.executor.clone().ok_or_else(|| "pipeline executor not configured".to_string())?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut stages = Vec::with_capacity(req.stages.len());
        for mut stage in req.stages {
            if stage.sandbox.is_none() {
                stage.sandbox = Some(SandboxPolicy::default());
            }
            // Zero-cost inlining of global functions at definition time.
            stage.logic = executor.inline_logic(&stage.logic, 0)?;
            stages.push(stage);
        }

        let def = PipelineDefinition {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            description: req.description,
            stages,
            auth_required: req.auth_required,
            enabled: req.enabled,
            version: req.version,
            created_at: now.clone(),
            updated_at: now,
            created_by: "system".to_string(),
        };
        let id = def.id.clone();
        {
            let mut map = self.pipelines.write().unwrap();
            map.insert(id.clone(), def.clone());
        }
        let def_clone = def.clone();
        let dir = self.data_dir.clone();
        tokio::spawn(async move {
            if let Err(e) = Self::save_async(&def_clone, &dir).await {
                eprintln!("[ERROR] Failed to persist pipeline {}: {}", def_clone.id, e);
            }
        });
        Ok(id)
    }

    fn validate(&self, req: &RegisterPipelineRequest) -> Result<(), String> {
        if req.name.trim().is_empty() {
            return Err("Pipeline name is required".to_string());
        }
        if req.stages.is_empty() {
            return Err("Pipeline needs at least one stage".to_string());
        }
        if req.stages.len() > 100 {
            return Err("Pipeline cannot exceed 100 stages".to_string());
        }
        let mut seen = std::collections::HashSet::new();
        for stage in &req.stages {
            if stage.name.trim().is_empty() {
                return Err("Stage name is required".to_string());
            }
            if !seen.insert(stage.name.clone()) {
                return Err(format!("Duplicate stage name '{}'", stage.name));
            }
            if stage.logic.is_empty() {
                return Err(format!("Stage '{}' logic cannot be empty", stage.name));
            }
            if let Some(policy) = &stage.sandbox {
                if policy.max_timeout_ms == 0 || policy.max_timeout_ms > 300_000 {
                    return Err(format!("Stage '{}': sandbox.max_timeout_ms must be 1..300000", stage.name));
                }
            }
        }
        Ok(())
    }

    pub async fn list_definitions(&self) -> Vec<PipelineDefinition> {
        let map = self.pipelines.read().unwrap();
        let mut defs: Vec<_> = map.values().cloned().collect();
        defs.sort_by(|a, b| a.name.cmp(&b.name));
        defs
    }

    /// UI-facing list: definitions + last run at a glance.
    pub async fn list_with_status(&self) -> Vec<PipelineListItem> {
        let defs = self.list_definitions().await;
        let runs = self.runs.read().unwrap();
        defs.into_iter()
            .map(|d| {
                let last = runs.iter().rev().find(|r| r.pipeline_id == d.id).cloned();
                let stages: Vec<PipelineStageView> = d
                    .stages
                    .iter()
                    .map(|s| {
                        let (last_status, last_duration_ms) = last
                            .as_ref()
                            .and_then(|r| r.stages.iter().find(|sr| sr.name == s.name))
                            .map(|sr| (Some(sr.status), Some(sr.execution_time_ms)))
                            .unwrap_or((None, None));
                        PipelineStageView {
                            name: s.name.clone(),
                            allow_failure: s.allow_failure,
                            last_status,
                            last_duration_ms,
                        }
                    })
                    .collect();
                PipelineListItem {
                    id: d.id,
                    name: d.name,
                    description: d.description,
                    version: d.version,
                    enabled: d.enabled,
                    stage_count: stages.len(),
                    stages,
                    last_run: last,
                }
            })
            .collect()
    }

    pub async fn get_definition(&self, id: &str) -> Option<PipelineDefinition> {
        self.pipelines.read().unwrap().get(id).cloned()
    }

    pub async fn find_by_name(&self, name: &str) -> Option<PipelineDefinition> {
        self.pipelines.read().unwrap().values().find(|d| d.name == name).cloned()
    }

    pub async fn delete_pipeline(&self, id: &str) -> bool {
        let removed = self.pipelines.write().unwrap().remove(id).is_some();
        if removed {
            let path = std::path::Path::new(&self.data_dir).join("pipelines").join(format!("{}.json", id));
            let _ = tokio::fs::remove_file(&path).await;
        }
        removed
    }

    // -- runs --------------------------------------------------------------

    pub async fn run_pipeline(
        &self,
        id_or_name: &str,
        payload: Option<Value>,
    ) -> Result<PipelineRunResult, String> {
        let def = match self.get_definition(id_or_name).await {
            Some(d) => d,
            None => self
                .find_by_name(id_or_name)
                .await
                .ok_or_else(|| format!("pipeline '{}' not found", id_or_name))?,
        };
        if !def.enabled {
            return Err("Pipeline is disabled".to_string());
        }
        let executor = self.executor.clone().ok_or_else(|| "pipeline executor not configured".to_string())?;

        let run_id = uuid::Uuid::new_v4().to_string();
        let total_start = std::time::Instant::now();

        // Shared scope across stages.
        let mut variables: HashMap<String, Value> = HashMap::new();
        variables.insert("payload".to_string(), payload.clone().unwrap_or(Value::Null));
        if let Some(Value::Object(obj)) = &payload {
            for (k, v) in obj {
                variables.entry(k.clone()).or_insert_with(|| v.clone());
            }
        }
        // Global functions available to every stage.
        let mut functions: HashMap<String, FunctionDefinition> = HashMap::new();
        for (name, f) in executor.get_global_functions().await {
            functions.insert(name, FunctionDefinition { params: f.params, body: f.logic });
        }

        let mut stage_results = Vec::with_capacity(def.stages.len());
        let mut failed = false;

        for (idx, stage) in def.stages.iter().enumerate() {
            if failed {
                stage_results.push(PipelineStageResult {
                    name: stage.name.clone(),
                    status: StageRunStatus::Skipped,
                    result: None,
                    error: None,
                    execution_time_ms: 0,
                });
                continue;
            }
            let policy = stage.sandbox.clone().unwrap_or_default();
            let timeout_ms = stage.timeout_ms.unwrap_or(policy.max_timeout_ms);
            variables.insert("stage".to_string(), Value::String(stage.name.clone()));
            variables.insert("stage_index".to_string(), Value::Number(idx.into()));

            let mut context = DynamicRouteExecutionContext {
                route_id: def.id.clone(),
                variables: std::mem::take(&mut variables),
                request_payload: payload.clone(),
                path_params: HashMap::new(),
                query_params: HashMap::new(),
                functions: functions.clone(),
                loop_control: LoopControl::default(),
                error_context: None,
                sandbox: Some(policy.clone()),
            };
            let stage_start = std::time::Instant::now();
            let mut steps = Vec::new();
            let exec_fut = execute_logic_extended(&stage.logic, &mut context, &mut steps);
            let outcome: Result<Value, String> =
                tokio::time::timeout(std::time::Duration::from_millis(timeout_ms), exec_fut)
                    .await
                    .map_err(|_| format!("stage '{}' exceeded timeout {}ms", stage.name, timeout_ms))?;
            let elapsed = stage_start.elapsed().as_millis() as u64;

            // Memory guard across the shared scope.
            if context.variables.len() > policy.max_variables {
                let err = format!("sandbox: variable count exceeds max_variables {}", policy.max_variables);
                stage_results.push(PipelineStageResult {
                    name: stage.name.clone(),
                    status: StageRunStatus::Failed,
                    result: None,
                    error: Some(err),
                    execution_time_ms: elapsed,
                });
                failed = true;
                continue;
            }

            match outcome {
                Ok(value) => {
                    context.variables.insert("last".to_string(), value.clone());
                    context.variables.insert(format!("stages.{}", stage.name), value.clone());
                    variables = std::mem::take(&mut context.variables);
                    stage_results.push(PipelineStageResult {
                        name: stage.name.clone(),
                        status: StageRunStatus::Success,
                        result: Some(value),
                        error: None,
                        execution_time_ms: elapsed,
                    });
                }
                Err(e) => {
                    context.variables.insert("error".to_string(), Value::String(e.clone()));
                    variables = std::mem::take(&mut context.variables);
                    if stage.allow_failure {
                        stage_results.push(PipelineStageResult {
                            name: stage.name.clone(),
                            status: StageRunStatus::Failed,
                            result: None,
                            error: Some(e),
                            execution_time_ms: elapsed,
                        });
                    } else {
                        stage_results.push(PipelineStageResult {
                            name: stage.name.clone(),
                            status: StageRunStatus::Failed,
                            result: None,
                            error: Some(e),
                            execution_time_ms: elapsed,
                        });
                        failed = true;
                    }
                }
            }
        }

        let total_time_ms = total_start.elapsed().as_millis() as u64;
        let success = !failed;
        let finished_at = chrono::Utc::now().to_rfc3339();
        let run = PipelineRunResult {
            run_id: run_id.clone(),
            pipeline_id: def.id.clone(),
            pipeline_name: def.name.clone(),
            success,
            stages: stage_results,
            total_time_ms,
            finished_at,
        };
        {
            let mut runs = self.runs.write().unwrap();
            if runs.len() >= PIPELINE_RUN_HISTORY_CAP {
                runs.pop_front();
            }
            runs.push_back(run.clone());
        }
        // Audit the whole run (success flag reflects the run outcome).
        executor.record_audit(
            &def.id,
            def.name.clone(),
            format!("pipeline:{}", def.name),
            success,
            if success { None } else { Some("one or more stages failed".to_string()) },
            total_time_ms,
            "pipeline",
        );
        Ok(run)
    }

    pub async fn run_history(&self, pipeline_id: Option<&str>, limit: usize) -> Vec<PipelineRunResult> {
        let runs = self.runs.read().unwrap();
        runs.iter()
            .rev()
            .filter(|r| pipeline_id.map(|id| r.pipeline_id == id).unwrap_or(true))
            .take(limit.clamp(1, 100))
            .cloned()
            .collect()
    }

    // -- persistence -------------------------------------------------------

    async fn save_async(def: &PipelineDefinition, data_dir: &str) -> Result<(), String> {
        use std::path::Path;
        let dir = Path::new(data_dir).join("pipelines");
        tokio::fs::create_dir_all(&dir).await.map_err(|e| format!("mkdir pipelines: {}", e))?;
        let content = serde_json::to_string_pretty(def).map_err(|e| format!("serialize: {}", e))?;
        tokio::fs::write(dir.join(format!("{}.json", def.id)), content)
            .await
            .map_err(|e| format!("write: {}", e))?;
        Ok(())
    }

    fn load_persisted(&self) -> Result<(), String> {
        use std::path::Path;
        let dir = Path::new(&self.data_dir).join("pipelines");
        if !dir.exists() {
            return Ok(());
        }
        let entries = std::fs::read_dir(&dir).map_err(|e| format!("read pipelines dir: {}", e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("dir entry: {}", e))?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let content = std::fs::read_to_string(&path).map_err(|e| format!("read {}: {}", path.display(), e))?;
            match serde_json::from_str::<PipelineDefinition>(&content) {
                Ok(def) => {
                    self.pipelines.write().unwrap().insert(def.id.clone(), def);
                }
                Err(e) => eprintln!("[WARN] skip pipeline file {}: {}", path.display(), e),
            }
        }
        Ok(())
    }
}
