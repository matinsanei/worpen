use super::routes::{LogicOperation, SandboxPolicy};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Kept for backward compatibility with old readers.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum PipelineStatus {
    SUCCESS,
    FAILED,
    RUNNING,
    PENDING,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineStage {
    pub id: String,
    pub name: String,
    pub status: PipelineStatus,
    pub duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub branch: String,
    pub commit: String,
    pub author: String,
    pub status: PipelineStatus,
    pub last_run: String,
    pub stages: Vec<PipelineStage>,
}

// ---------------------------------------------------------------------------
// Real pipelines: multi-stage Logic-as-Data executed by the Worpen VM.
// A pipeline is an ordered list of stages; each stage is a logic program.
// Stages share one variable scope (`last` + `stages.<name>` carry results).
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StageRunStatus {
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineStageDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// Logic program executed for this stage (inlined at definition time).
    pub logic: Vec<LogicOperation>,
    /// Keep going when this stage fails.
    #[serde(default)]
    pub allow_failure: bool,
    /// Per-stage timeout (ms). Defaults to the stage sandbox policy.
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub sandbox: Option<SandboxPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineDefinition {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub stages: Vec<PipelineStageDef>,
    #[serde(default)]
    pub auth_required: bool,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_version")]
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
}

fn default_enabled() -> bool {
    true
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterPipelineRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub stages: Vec<PipelineStageDef>,
    #[serde(default)]
    pub auth_required: bool,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_version")]
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineStageResult {
    pub name: String,
    pub status: StageRunStatus,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineRunResult {
    pub run_id: String,
    pub pipeline_id: String,
    pub pipeline_name: String,
    pub success: bool,
    pub stages: Vec<PipelineStageResult>,
    pub total_time_ms: u64,
    pub finished_at: String,
}

/// One row of the UI-facing pipeline list: definition + last run at a glance.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineListItem {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    pub enabled: bool,
    pub stage_count: usize,
    pub stages: Vec<PipelineStageView>,
    pub last_run: Option<PipelineRunResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineStageView {
    pub name: String,
    pub allow_failure: bool,
    pub last_status: Option<StageRunStatus>,
    pub last_duration_ms: Option<u64>,
}
