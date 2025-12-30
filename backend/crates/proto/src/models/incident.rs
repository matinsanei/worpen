use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
