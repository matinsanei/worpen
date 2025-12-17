use std::sync::Arc;
use crate::domain::AgentStatus;
use crate::ports::repository::{AgentRepository, IncidentRepository};

pub struct DashboardService {
    agent_repo: Arc<dyn AgentRepository>,
    incident_repo: Arc<dyn IncidentRepository>,
}

impl DashboardService {
    pub fn new(agent_repo: Arc<dyn AgentRepository>, incident_repo: Arc<dyn IncidentRepository>) -> Self {
        Self { agent_repo, incident_repo }
    }

    pub async fn get_stats(&self) -> Result<serde_json::Value, String> {
        let agents = self.agent_repo.find_all().await?;
        let active_nodes = agents.iter().filter(|a| matches!(a.status, AgentStatus::Online)).count();
        
        let incidents = self.incident_repo.find_all().await?;
        let security_score = if incidents.is_empty() { 98 } else { 85 };

        Ok(serde_json::json!({
            "active_nodes": active_nodes,
            "throughput": "1.2k/s", // Mocked for now
            "avg_load": "42%",     // Mocked
            "security_score": format!("{}%", security_score)
        }))
    }
}
