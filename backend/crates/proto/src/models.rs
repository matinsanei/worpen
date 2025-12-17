use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// --- AGENT / FLEET ---
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum AgentStatus {
    ONLINE,
    OFFLINE,
    HEALING,
    CRITICAL,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub os: String,
    pub version: String,
    pub status: AgentStatus,
    pub cpu_load: u8,
    pub memory_load: u8,
    pub last_seen: String, // ISO timestamp or relative
    pub uptime: String,
    pub peers: u32,
}

// --- INCIDENTS ---
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum IncidentStatus {
    RESOLVED,
    PENDING,
    FAILED,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Incident {
    pub id: String,
    pub service: String,
    pub node: String,
    pub issue: String,
    pub action: String,
    pub status: IncidentStatus,
    pub time: String,
}

// --- DOCKER / ORCHESTRATION ---
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ContainerState {
    RUNNING,
    EXITED,
    RESTARTING,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub status: String,
    pub node: String,
    pub ports: String,
}

// --- AUTOMATION ---
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AutomationRule {
    pub id: String,
    pub name: String,
    pub trigger_event: String,
    pub target_service: String,
    pub script: String,
    pub last_run: String,
    pub active: bool,
}

// --- LOGS ---
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    SUCCESS,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

// --- PIPELINES (CI/CD) ---
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

// --- TERMINAL / EXECUTION ---
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExecRequest {
    pub command: String,
    pub args: Vec<String>,
    pub workdir: Option<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub tty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExecResponse {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ShellSession {
    pub session_id: String,
    pub target_type: String, // "container" or "agent"
    pub target_id: String,
    pub created_at: String,
    pub last_activity: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TerminalInput {
    pub session_id: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TerminalOutput {
    pub session_id: String,
    pub data: String,
    pub stream: String, // "stdout" or "stderr"
}

// --- DYNAMIC ROUTE ENGINE ---
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum LogicOperation {
    // Data Operations
    #[serde(rename = "return")]
    Return { value: serde_json::Value },
    
    #[serde(rename = "query_db")]
    QueryDb { query: String, params: Vec<serde_json::Value> },
    
    #[serde(rename = "http_request")]
    HttpRequest { url: String, method: String, body: Option<serde_json::Value> },
    
    // Control Flow
    #[serde(rename = "if")]
    If { condition: String, then: Vec<LogicOperation>, otherwise: Option<Vec<LogicOperation>> },
    
    #[serde(rename = "loop")]
    Loop { collection: String, var: String, body: Vec<LogicOperation> },
    
    // Data Transformations
    #[serde(rename = "map")]
    Map { input: String, transform: String },
    
    #[serde(rename = "filter")]
    Filter { input: String, condition: String },
    
    #[serde(rename = "aggregate")]
    Aggregate { input: String, operation: String }, // sum, count, avg, min, max
    
    // Variable Operations
    #[serde(rename = "set")]
    Set { var: String, value: serde_json::Value },
    
    #[serde(rename = "get")]
    Get { var: String },
    
    // Custom Script Execution
    #[serde(rename = "execute_script")]
    ExecuteScript { language: String, code: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub method: HttpMethod,
    pub logic: Vec<LogicOperation>,
    pub parameters: Vec<RouteParameter>,
    pub response_schema: Option<serde_json::Value>,
    pub auth_required: bool,
    pub rate_limit: Option<u32>,
    pub enabled: bool,
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
}

/// Request body for registering a new route (id, timestamps, created_by are auto-generated)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterRouteRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub path: String,
    pub method: HttpMethod,
    pub logic: Vec<LogicOperation>,
    #[serde(default)]
    pub parameters: Vec<RouteParameter>,
    pub response_schema: Option<serde_json::Value>,
    #[serde(default)]
    pub auth_required: bool,
    pub rate_limit: Option<u32>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_enabled() -> bool {
    true
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteParameter {
    pub name: String,
    pub param_type: String, // "path", "query", "body"
    pub data_type: String,  // "string", "number", "boolean", "object", "array"
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation: Option<String>, // regex or validation rule
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteTestRequest {
    pub route_id: String,
    pub test_payload: Option<serde_json::Value>,
    pub test_params: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteTestResponse {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub steps_executed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DynamicRouteExecutionContext {
    pub route_id: String,
    pub variables: std::collections::HashMap<String, serde_json::Value>,
    pub request_payload: Option<serde_json::Value>,
    pub path_params: std::collections::HashMap<String, String>,
    pub query_params: std::collections::HashMap<String, String>,
}
