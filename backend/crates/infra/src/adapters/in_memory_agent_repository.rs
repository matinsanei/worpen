use worpen_core::domain::Agent;
use worpen_core::ports::AgentRepository;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct InMemoryAgentRepository {
    agents: Arc<DashMap<String, Agent>>,
}

impl Default for InMemoryAgentRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryAgentRepository {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(DashMap::new()),
        }
    }
}

impl AgentRepository for InMemoryAgentRepository {
    fn save(&self, agent: Agent) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let agents = self.agents.clone();
        Box::pin(async move {
            agents.insert(agent.id.to_string(), agent);
            Ok(())
        })
    }

    fn find_by_id(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<Option<Agent>, String>> + Send>> {
        let agents = self.agents.clone();
        Box::pin(async move {
            let agent = agents.get(&id.to_string()).map(|a| a.clone());
            Ok(agent)
        })
    }

    fn find_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Agent>, String>> + Send>> {
        let agents = self.agents.clone();
        Box::pin(async move {
            let all_agents: Vec<Agent> = agents.iter().map(|entry| entry.value().clone()).collect();
            Ok(all_agents)
        })
    }

    fn delete(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let agents = self.agents.clone();
        Box::pin(async move {
            agents.remove(&id.to_string());
            Ok(())
        })
    }
}
