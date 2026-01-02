use std::pin::Pin;
use std::future::Future;
use sqlx::SqlitePool;
use worpen_core::ports::repository::IncidentRepository;
use proto::models::{Incident, IncidentStatus};

pub struct SqliteIncidentRepository {
    pool: SqlitePool,
}

impl SqliteIncidentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl IncidentRepository for SqliteIncidentRepository {
    fn save(&self, incident: Incident) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            let status_str = format!("{:?}", incident.status);
            sqlx::query("INSERT INTO incidents (id, service, node, issue, action, status, time) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(incident.id)
                .bind(incident.service)
                .bind(incident.node)
                .bind(incident.issue)
                .bind(incident.action)
                .bind(status_str)
                .bind(incident.time)
                .execute(&pool)
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        })
    }

    fn find_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Incident>, String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            let rows = sqlx::query_as::<_, (String, String, String, String, String, String, String)>("SELECT id, service, node, issue, action, status, time FROM incidents")
                .fetch_all(&pool)
                .await
                .map_err(|e| e.to_string())?;

            let incidents = rows.into_iter().map(|row| {
                let status = match row.5.as_str() {
                    "PENDING" => IncidentStatus::PENDING,
                    "RESOLVED" => IncidentStatus::RESOLVED,
                    _ => IncidentStatus::FAILED,
                };
                Incident {
                    id: row.0,
                    service: row.1,
                    node: row.2,
                    issue: row.3,
                    action: row.4,
                    status,
                    time: row.6,
                }
            }).collect();
            Ok(incidents)
        })
    }

    fn resolve(&self, id: String, _method: String, _notes: String) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
             sqlx::query("UPDATE incidents SET status = 'RESOLVED' WHERE id = ?")
                 .bind(id)
                 .execute(&pool)
                 .await
                 .map_err(|e| e.to_string())?;
             Ok(())
        })
    }
}
