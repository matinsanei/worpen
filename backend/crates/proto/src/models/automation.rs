use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
