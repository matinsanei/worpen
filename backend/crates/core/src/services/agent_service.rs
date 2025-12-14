use crate::domain::Agent;
use crate::ports::AgentRepository;
use uuid::Uuid;
use std::sync::Arc;

pub struct AgentService {
    repository: Arc<dyn AgentRepository>,
}

impl AgentService {
    pub fn new(repository: Arc<dyn AgentRepository>) -> Self {
        Self { repository }
    }

    pub async fn register_agent(&self, hostname: String, ip_address: Option<String>, version: String) -> Result<Uuid, String> {
        let agent = Agent::new(hostname, ip_address, version);
        let id = agent.id;
        self.repository.save(agent).await?;
        Ok(id)
    }

    pub async fn process_heartbeat(&self, agent_id: Uuid) -> Result<(), String> {
        if let Some(mut agent) = self.repository.find_by_id(agent_id).await? {
            agent.heartbeat();
            self.repository.save(agent).await?;
            Ok(())
        } else {
            Err("Agent not found".to_string())
        }
    }
}
