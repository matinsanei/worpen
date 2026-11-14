use super::routes::SandboxPolicy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::ToSchema;

fn default_spec_version() -> String {
    "1.0".to_string()
}

fn default_status_ok() -> u16 {
    200
}

fn default_max_attempts() -> u32 {
    3
}

fn default_backoff_ms() -> u64 {
    500
}

/// Workflow Spec v1: Strict, human-authored and agent-executable workflow definition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WorkflowSpecV1 {
    #[serde(default = "default_spec_version")]
    pub version: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub input_schema: Option<Value>,
    #[serde(default)]
    pub policy: Option<SandboxPolicy>,
    pub steps: Vec<WorkflowStepV1>,
    pub r#return: WorkflowReturnV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WorkflowStepV1 {
    pub id: String,
    #[serde(flatten)]
    pub op: WorkflowOpV1,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub retry: Option<RetryPolicy>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RetryPolicy {
    #[serde(default = "default_max_attempts")]
    pub max_attempts: u32,
    #[serde(default = "default_backoff_ms")]
    pub backoff_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum WorkflowOpV1 {
    Sql {
        query: String,
        #[serde(default)]
        args: Vec<Value>,
        output_var: String,
    },
    Http {
        method: String,
        url: String,
        #[serde(default)]
        body: Option<Value>,
        #[serde(default)]
        headers: Option<HashMap<String, String>>,
        #[serde(default)]
        output_var: Option<String>,
    },
    If {
        condition: String,
        then: Vec<WorkflowStepV1>,
        #[serde(default)]
        r#else: Option<Vec<WorkflowStepV1>>,
    },
    Switch {
        value: String,
        cases: HashMap<String, Vec<WorkflowStepV1>>,
        #[serde(default)]
        default: Option<Vec<WorkflowStepV1>>,
    },
    Loop {
        items: String,
        as_var: String,
        body: Vec<WorkflowStepV1>,
        max_iterations: u32,
    },
    Transform {
        expr: String,
        output_var: String,
    },
    Set {
        var: String,
        value: Value,
    },
    TryCatch {
        r#try: Vec<WorkflowStepV1>,
        catch: Vec<WorkflowStepV1>,
        #[serde(default)]
        error_var: Option<String>,
    },
    Parallel {
        branches: Vec<Vec<WorkflowStepV1>>,
        #[serde(default)]
        max_concurrent: Option<usize>,
    },
    Approval {
        summary: String,
        #[serde(default)]
        timeout_seconds: Option<u64>,
    },
    Fail {
        message: String,
        #[serde(default)]
        code: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WorkflowReturnV1 {
    #[serde(default = "default_status_ok")]
    pub status: u16,
    pub value: Value,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
}

/// Execution mode for workflow runs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionMode {
    Live,
    DryRun,
}
