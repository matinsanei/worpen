use crate::domain::Agent;
use uuid::Uuid;
use std::future::Future;
use std::pin::Pin;

pub trait AgentRepository: Send + Sync {
    fn save(&self, agent: Agent) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
    fn find_by_id(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<Option<Agent>, String>> + Send>>;
    fn delete(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
}
