use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
