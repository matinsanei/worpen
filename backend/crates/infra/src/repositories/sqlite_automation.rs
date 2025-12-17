use std::pin::Pin;
use std::future::Future;
use sqlx::SqlitePool;
use core::ports::repository::AutomationRepository;
use proto::models::AutomationRule;

pub struct SqliteAutomationRepository {
    pool: SqlitePool,
}

impl SqliteAutomationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl AutomationRepository for SqliteAutomationRepository {
    fn save(&self, rule: AutomationRule) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            sqlx::query("INSERT OR REPLACE INTO automation_rules (id, name, trigger_event, target_service, script, last_run, active) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(rule.id)
                .bind(rule.name)
                .bind(rule.trigger_event)
                .bind(rule.target_service)
                .bind(rule.script)
                .bind(rule.last_run)
                .bind(rule.active)
                .execute(&pool)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        })
    }

    fn find_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<AutomationRule>, String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            let rows = sqlx::query_as::<_, (String, String, String, String, String, String, bool)>("SELECT id, name, trigger_event, target_service, script, last_run, active FROM automation_rules")
                .fetch_all(&pool)
                .await
                .map_err(|e| e.to_string())?;

            let rules = rows.into_iter().map(|row| AutomationRule {
                id: row.0,
                name: row.1,
                trigger_event: row.2,
                target_service: row.3,
                script: row.4,
                last_run: row.5,
                active: row.6,
            }).collect();
            Ok(rules)
        })
    }
}
