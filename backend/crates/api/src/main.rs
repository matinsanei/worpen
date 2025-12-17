use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use infra::{adapters::SqliteAgentRepository, initialize_db};
use core::services::AgentService;
use state::AppState;
use tower_http::cors::{CorsLayer, Any};

mod state;
mod dtos;
mod handlers;
mod apidoc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use apidoc::ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Ensure DATABASE_URL is set
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:worpen.db?mode=rwc".to_string());
    
    // Initialize DB and run migrations
    let pool = initialize_db(&database_url).await.expect("Failed to initialize database");
    
    let repo = std::sync::Arc::new(SqliteAgentRepository::new(pool.clone()));
    let incident_repo = std::sync::Arc::new(infra::repositories::SqliteIncidentRepository::new(pool.clone()));
    let automation_repo = std::sync::Arc::new(infra::repositories::SqliteAutomationRepository::new(pool.clone()));

    let agent_service = std::sync::Arc::new(AgentService::new(repo.clone()));
    let dashboard_service = std::sync::Arc::new(core::services::DashboardService::new(repo.clone(), incident_repo.clone()));
    let docker_service = std::sync::Arc::new(core::services::DockerService::new());
    let incident_service = std::sync::Arc::new(core::services::IncidentService::new(incident_repo));
    let automation_service = std::sync::Arc::new(core::services::AutomationService::new(automation_repo));
    let pipeline_service = std::sync::Arc::new(core::services::PipelineService::new());
    
    let connected_agents = std::sync::Arc::new(dashmap::DashMap::new());
    
    let state = AppState {
        agent_service,
        dashboard_service,
        docker_service,
        incident_service,
        automation_service,
        pipeline_service,
        connected_agents,
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health_check))
        .route("/api/v1/agents/register", post(handlers::register_agent))
        .route("/api/v1/telemetry", post(handlers::receive_telemetry))
        .route("/api/v1/ws", get(handlers::ws_handler))
        // Dashboard Routes
        .route("/api/v1/dashboard/stats", get(handlers::get_stats))
        .route("/api/v1/dashboard/network", get(handlers::get_network_stats))
        .route("/api/v1/dashboard/services", get(handlers::get_services_health))
        .route("/api/v1/dashboard/load", get(handlers::get_load_distribution))
        // Fleet Routes
        .route("/api/v1/agents", get(handlers::list_agents))
        .route("/api/v1/agents/:id", get(handlers::get_agent))
        .route("/api/v1/agents/:id/exec", post(handlers::exec_on_agent))
        .route("/api/v1/agents/:id/shell", post(handlers::start_agent_shell))
        .route("/api/v1/agents/:id/processes", get(handlers::list_agent_processes))
        .route("/api/v1/agents/:id/system", get(handlers::get_agent_system_info))
        .route("/api/v1/agents/sync", post(handlers::sync_agents))
        .route("/api/v1/internal/heartbeat", post(handlers::agent_heartbeat))
        // Docker Routes
        .route("/api/v1/containers", get(handlers::list_containers))
        .route("/api/v1/containers/:id/:action", post(handlers::container_action))
        .route("/api/v1/containers/:id/logs", get(handlers::get_container_logs))
        .route("/api/v1/containers/:id/start", post(handlers::start_container))
        .route("/api/v1/containers/:id/stop", post(handlers::stop_container))
        .route("/api/v1/containers/:id/restart", post(handlers::restart_container))
        .route("/api/v1/containers/:id", axum::routing::delete(handlers::delete_container))
        .route("/api/v1/containers/:id/exec", post(handlers::exec_in_container))
        .route("/api/v1/containers/:id/shell", post(handlers::start_container_shell))
        .route("/api/v1/containers/:id/attach", get(handlers::attach_to_container))
        .route("/api/v1/containers/:id/inspect", get(handlers::inspect_container))
        .route("/api/v1/docker/prune", post(handlers::prune_system))
        .route("/api/v1/images", get(handlers::list_images))
        .route("/api/v1/images/pull", post(handlers::pull_image))
        // Incident Routes
        .route("/api/v1/incidents", get(handlers::list_incidents))
        .route("/api/v1/incidents/:id/resolve", post(handlers::resolve_incident))
        // Automation Routes
        .route("/api/v1/automation/rules", get(handlers::list_rules).post(handlers::create_rule))
        .route("/api/v1/automation/rules/:id", get(handlers::get_rule).put(handlers::update_rule))
        .route("/api/v1/automation/dry-run", post(handlers::dry_run))
        // Pipeline Routes
        .route("/api/v1/pipelines", get(handlers::list_pipelines))
        .route("/api/v1/pipelines/trigger", post(handlers::trigger_pipeline))
        // Logs Routes
        .route("/api/v1/logs", get(handlers::get_logs))
        // Terminal WebSocket Routes
        .route("/api/v1/terminal/container/:id", get(handlers::container_terminal_ws))
        .route("/api/v1/terminal/agent/:id", get(handlers::agent_terminal_ws))
        .route("/api/v1/terminal/sessions", get(handlers::list_terminal_sessions))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


