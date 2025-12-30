use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
