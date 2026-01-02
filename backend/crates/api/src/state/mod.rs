use worpen_core::services::{
    AgentService, DashboardService, DockerService, IncidentService, 
    AutomationService, PipelineService, DynamicRouteService
};
use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct AppState {
    pub agent_service: Arc<AgentService>,
    pub dashboard_service: Arc<DashboardService>,
    pub docker_service: Arc<DockerService>,
    pub incident_service: Arc<IncidentService>,
    pub automation_service: Arc<AutomationService>,
    pub pipeline_service: Arc<PipelineService>,
    pub dynamic_route_service: Arc<DynamicRouteService>,
    pub connected_agents: Arc<DashMap<uuid::Uuid, Sender<String>>>,
}
