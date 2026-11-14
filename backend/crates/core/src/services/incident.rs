use crate::ports::repository::IncidentRepository;
use proto::models::Incident;
use std::sync::Arc;

pub struct IncidentService {
    repo: Arc<dyn IncidentRepository>,
}

impl IncidentService {
    pub fn new(repo: Arc<dyn IncidentRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_incidents(&self) -> Result<Vec<Incident>, String> {
        self.repo.find_all().await
    }

    pub async fn resolve_incident(
        &self,
        id: String,
        method: String,
        notes: String,
    ) -> Result<(), String> {
        self.repo.resolve(id, method, notes).await
    }
}
