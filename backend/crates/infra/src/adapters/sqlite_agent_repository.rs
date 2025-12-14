use core::domain::{Agent, AgentStatus};
use core::ports::AgentRepository;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use std::future::Future;
use std::pin::Pin;
use chrono::{DateTime, Utc};
use std::str::FromStr;

#[derive(Clone)]
pub struct SqliteAgentRepository {
    pool: SqlitePool,
}

impl SqliteAgentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl AgentRepository for SqliteAgentRepository {
    fn save(&self, agent: Agent) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            let status = match agent.status {
                AgentStatus::Online => "Online",
                AgentStatus::Offline => "Offline",
                AgentStatus::Busy => "Busy",
                AgentStatus::Error => "Error",
            };
            
            let metadata_json = serde_json::to_value(&agent.metadata).map_err(|e| e.to_string())?;

            sqlx::query(
                r#"
                INSERT INTO agents (id, hostname, ip_address, status, last_heartbeat, version, metadata)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT(id) DO UPDATE SET
                    hostname = excluded.hostname,
                    ip_address = excluded.ip_address,
                    status = excluded.status,
                    last_heartbeat = excluded.last_heartbeat,
                    version = excluded.version,
                    metadata = excluded.metadata
                "#
            )
            .bind(agent.id.to_string())
            .bind(agent.hostname)
            .bind(agent.ip_address)
            .bind(status)
            .bind(agent.last_heartbeat)
            .bind(agent.version)
            .bind(metadata_json)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
            
            Ok(())
        })
    }

    fn find_by_id(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<Option<Agent>, String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            let row = sqlx::query("SELECT * FROM agents WHERE id = $1")
                .bind(id.to_string())
                .fetch_optional(&pool)
                .await
                .map_err(|e| e.to_string())?;

            if let Some(row) = row {
                let status_str: String = row.try_get("status").map_err(|e| e.to_string())?;
                let status = match status_str.as_str() {
                    "Online" => AgentStatus::Online,
                    "Busy" => AgentStatus::Busy,
                    "Error" => AgentStatus::Error,
                    _ => AgentStatus::Offline,
                };
                
                let id_str: String = row.try_get("id").map_err(|e| e.to_string())?;
                let id = Uuid::from_str(&id_str).map_err(|e| e.to_string())?;

                let metadata_val: serde_json::Value = row.try_get("metadata").map_err(|e| e.to_string())?;
                let metadata = serde_json::from_value(metadata_val).map_err(|e| e.to_string())?;
                
                // sqlx decodes DATETIME to chrono::DateTime<Utc> automatically if feature is enabled
                let last_heartbeat: DateTime<Utc> = row.try_get("last_heartbeat").map_err(|e| e.to_string())?;

                Ok(Some(Agent {
                    id,
                    hostname: row.try_get("hostname").map_err(|e| e.to_string())?,
                    ip_address: row.try_get("ip_address").map_err(|e| e.to_string())?,
                    status,
                    last_heartbeat,
                    version: row.try_get("version").map_err(|e| e.to_string())?,
                    metadata,
                }))
            } else {
                Ok(None)
            }
        })
    }

    fn delete(&self, id: Uuid) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            sqlx::query("DELETE FROM agents WHERE id = $1")
                .bind(id.to_string())
                .execute(&pool)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        })
    }
}
