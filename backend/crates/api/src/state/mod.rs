use core::services::AgentService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub agent_service: Arc<AgentService>,
}
