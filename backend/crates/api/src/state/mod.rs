use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use worpen_core::services::{
    AgentService, AutomationService, DashboardService, DockerService, DynamicRouteService,
    IncidentService, PipelineService,
};

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub agent_service: Arc<AgentService>,
    #[allow(dead_code)]
    pub dashboard_service: Arc<DashboardService>,
    #[allow(dead_code)]
    pub docker_service: Arc<DockerService>,
    #[allow(dead_code)]
    pub incident_service: Arc<IncidentService>,
    #[allow(dead_code)]
    pub automation_service: Arc<AutomationService>,
    pub pipeline_service: Arc<PipelineService>,
    pub dynamic_route_service: Arc<DynamicRouteService>,
    pub workflow_service: Arc<worpen_core::workflow::WorkflowService>,
    #[allow(dead_code)]
    pub connected_agents: Arc<DashMap<uuid::Uuid, Sender<String>>>,
}

impl AppState {
    #[allow(dead_code)]
    pub async fn new_with_pool_and_data_dir(
        pool: sqlx::Pool<sqlx::Sqlite>,
        data_dir: String,
    ) -> Self {
        let repo = Arc::new(infra::SqliteAgentRepository::new(pool.clone()));
        let incident_repo = Arc::new(infra::repositories::SqliteIncidentRepository::new(
            pool.clone(),
        ));
        let automation_repo = Arc::new(infra::repositories::SqliteAutomationRepository::new(
            pool.clone(),
        ));

        let agent_service = Arc::new(AgentService::new(repo.clone()));
        let dashboard_service =
            Arc::new(DashboardService::new(repo.clone(), incident_repo.clone()));
        let docker_service = Arc::new(DockerService::new());
        let incident_service = Arc::new(IncidentService::new(incident_repo));
        let automation_service = Arc::new(AutomationService::new(automation_repo));
        let mut dynamic_route_service = DynamicRouteService::with_data_dir(data_dir.clone());
        dynamic_route_service.set_db_pool(pool.clone());
        let dynamic_route_service = Arc::new(dynamic_route_service);
        let pipeline_service = Arc::new(PipelineService::with_executor(
            dynamic_route_service.clone(),
        ));
        let workflow_service = Arc::new(worpen_core::workflow::WorkflowService::new(
            data_dir,
            dynamic_route_service.clone(),
        ));
        let _ = workflow_service.load_persisted_workflows().await;
        let connected_agents = Arc::new(DashMap::new());
        Self {
            agent_service,
            dashboard_service,
            docker_service,
            incident_service,
            automation_service,
            pipeline_service,
            dynamic_route_service,
            workflow_service,
            connected_agents,
        }
    }
}
