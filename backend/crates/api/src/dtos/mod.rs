use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RegisterAgentRequest {
    #[schema(example = "agent-01")]
    pub hostname: String,
    #[schema(example = "192.168.1.100")]
    pub ip_address: Option<String>,
    #[schema(example = "1.0.0")]
    pub version: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TelemetryRequest {
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub agent_id: uuid::Uuid,
    #[schema(example = 45.5)]
    pub cpu_usage: f32,
    #[schema(example = 1024000)]
    pub memory_usage: u64,
    #[schema(example = 8192000)]
    pub total_memory: u64,
}

