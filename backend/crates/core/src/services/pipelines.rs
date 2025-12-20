use proto::models::{Pipeline, PipelineStatus, PipelineStage};

pub struct PipelineService;

impl Default for PipelineService {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineService {
    pub fn new() -> Self {
        Self
    }

    pub async fn list_pipelines(&self) -> Result<Vec<Pipeline>, String> {
        // Mock data
        Ok(vec![
            Pipeline {
                id: "pipe-001".to_string(),
                name: "production-deploy".to_string(),
                branch: "main".to_string(),
                commit: "7f8a91b".to_string(),
                author: "dev_ops_Lead".to_string(),
                status: PipelineStatus::SUCCESS,
                last_run: "2 hours ago".to_string(),
                stages: vec![
                    PipelineStage { id: "s1".to_string(), name: "BUILD".to_string(), status: PipelineStatus::SUCCESS, duration: "45s".to_string() },
                    PipelineStage { id: "s2".to_string(), name: "TEST".to_string(), status: PipelineStatus::SUCCESS, duration: "1m 20s".to_string() },
                    PipelineStage { id: "s3".to_string(), name: "SCAN".to_string(), status: PipelineStatus::SUCCESS, duration: "30s".to_string() },
                    PipelineStage { id: "s4".to_string(), name: "DEPLOY".to_string(), status: PipelineStatus::SUCCESS, duration: "15s".to_string() },
                ],
            },
            Pipeline {
                id: "pipe-002".to_string(),
                name: "staging-integration".to_string(),
                branch: "develop".to_string(),
                commit: "3c4d5e6".to_string(),
                author: "frontend_ninja".to_string(),
                status: PipelineStatus::FAILED,
                last_run: "5 mins ago".to_string(),
                stages: vec![
                    PipelineStage { id: "s1".to_string(), name: "BUILD".to_string(), status: PipelineStatus::SUCCESS, duration: "42s".to_string() },
                    PipelineStage { id: "s2".to_string(), name: "TEST".to_string(), status: PipelineStatus::FAILED, duration: "3s".to_string() },
                ],
            },
        ])
    }
}
