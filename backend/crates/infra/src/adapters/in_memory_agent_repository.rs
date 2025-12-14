use core::domain::Agent;
use core::ports::AgentRepository;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct InMemoryAgentRepository {
    agents: Arc<DashMap<Uuid, Agent>>,
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
            agents.insert(agent.id, agent);
            Ok(())
        })
    }

    fn find_by_id(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<Option<Agent>, String>> + Send>> {
        let agents = self.agents.clone();
        Box::pin(async move {
            let agent = agents.get(&id).map(|a| a.clone());
            Ok(agent)
        })
    }

    fn delete(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let agents = self.agents.clone();
        Box::pin(async move {
            agents.remove(&id);
            Ok(())
        })
    }
}
