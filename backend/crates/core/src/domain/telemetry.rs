use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub agent_id: Uuid,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub total_memory: u64,
    pub timestamp: DateTime<Utc>,
}

impl Telemetry {
    pub fn new(agent_id: Uuid, cpu_usage: f32, memory_usage: u64, total_memory: u64) -> Self {
        Self {
            agent_id,
            cpu_usage,
            memory_usage,
            total_memory,
            timestamp: Utc::now(),
        }
    }
}
