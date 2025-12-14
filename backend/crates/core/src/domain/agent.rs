use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentStatus {
    Offline,
    Online,
    Busy,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub hostname: String,
    pub ip_address: Option<String>,
    pub status: AgentStatus,
    pub last_heartbeat: DateTime<Utc>,
    pub version: String,
    pub metadata: HashMap<String, String>,
}

impl Agent {
    pub fn new(hostname: String, ip_address: Option<String>, version: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            hostname,
            ip_address,
            status: AgentStatus::Online,
            last_heartbeat: Utc::now(),
            version,
            metadata: HashMap::new(),
        }
    }

    pub fn heartbeat(&mut self) {
        self.last_heartbeat = Utc::now();
        self.status = AgentStatus::Online;
    }
}
