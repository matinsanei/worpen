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
