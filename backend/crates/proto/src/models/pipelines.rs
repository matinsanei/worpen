use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
