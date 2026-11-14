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
    #[allow(dead_code)]
    pub connected_agents: Arc<DashMap<uuid::Uuid, Sender<String>>>,
}
