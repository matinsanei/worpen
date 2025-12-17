use crate::domain::Agent;
use proto::models::{Incident, AutomationRule, LogEntry};
use uuid::Uuid;
use std::future::Future;
use std::pin::Pin;

pub trait AgentRepository: Send + Sync {
    fn save(&self, agent: Agent) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
    fn find_by_id(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<Option<Agent>, String>> + Send>>;
    fn find_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Agent>, String>> + Send>>;
    fn delete(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
}

pub trait IncidentRepository: Send + Sync {
    fn save(&self, incident: Incident) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
    fn find_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Incident>, String>> + Send>>;
    fn resolve(&self, id: String, method: String, notes: String) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
}

pub trait AutomationRepository: Send + Sync {
    fn save(&self, rule: AutomationRule) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
    fn find_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<AutomationRule>, String>> + Send>>;
}

pub trait LogRepository: Send + Sync {
    fn save(&self, log: LogEntry) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>;
    fn find_recent(&self, limit: u32) -> Pin<Box<dyn Future<Output = Result<Vec<LogEntry>, String>> + Send>>;
}
