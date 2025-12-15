use core::services::AgentService;
use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct AppState {
    pub agent_service: Arc<AgentService>,
    pub connected_agents: Arc<DashMap<uuid::Uuid, Sender<String>>>,
}
