use std::sync::Arc;
use proto::models::AutomationRule;
use crate::ports::repository::AutomationRepository;

pub struct AutomationService {
    repo: Arc<dyn AutomationRepository>,
}

impl AutomationService {
    pub fn new(repo: Arc<dyn AutomationRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_rules(&self) -> Result<Vec<AutomationRule>, String> {
        self.repo.find_all().await
    }

    pub async fn create_rule(&self, rule: AutomationRule) -> Result<(), String> {
        self.repo.save(rule).await
    }
}
