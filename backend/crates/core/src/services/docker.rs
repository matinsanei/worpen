use proto::models::{Container, ContainerState};

pub struct DockerService;

impl DockerService {
    pub fn new() -> Self {
        Self
    }

    pub async fn list_containers(&self) -> Result<Vec<Container>, String> {
        // Mock data matching constants.ts
        Ok(vec![
             Container {
                 id: "a1b2c3d4".to_string(),
                 name: "saleor-api".to_string(),
                 image: "saleor/saleor:3.1".to_string(),
                 state: ContainerState::RUNNING,
                 status: "Up 2 hours".to_string(),
                 node: "hive-worker-beta".to_string(),
                 ports: "8000:8000".to_string(),
             },
             Container {
                 id: "e5f6g7h8".to_string(),
                 name: "saleor-dashboard".to_string(),
                 image: "saleor/dashboard:3.1".to_string(),
                 state: ContainerState::RUNNING,
                 status: "Up 2 hours".to_string(),
                 node: "hive-worker-beta".to_string(),
                 ports: "9000:80".to_string(),
             },
             Container {
                 id: "m3n4o5p6".to_string(),
                 name: "redis-cache".to_string(),
                 image: "redis:6-alpine".to_string(),
                 state: ContainerState::EXITED,
                 status: "Exited (137) 10 min ago".to_string(),
                 node: "edge-robot-x1".to_string(),
                 ports: "6379:6379".to_string(),
             },
        ])
    }

    pub async fn container_action(&self, id: String, action: &str) -> Result<String, String> {
        match action {
            "start" => Ok(format!("Container {} started", id)),
            "stop" => Ok(format!("Container {} stopped", id)),
            "restart" => Ok(format!("Container {} restarted", id)),
            _ => Err("Invalid action".to_string()),
        }
    }
}
