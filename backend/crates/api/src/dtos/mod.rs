use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterAgentRequest {
    pub hostname: String,
    pub ip_address: Option<String>,
    pub version: String,
}

#[derive(Deserialize)]
pub struct TelemetryRequest {
    pub agent_id: uuid::Uuid,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub total_memory: u64,
}

