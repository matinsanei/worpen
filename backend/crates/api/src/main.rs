use axum::{routing::{get, post}, Router};
use std::net::{SocketAddr, IpAddr};
use std::io::{self, Write};
use infra::{adapters::SqliteAgentRepository, initialize_db};
use worpen_core::services::AgentService;
use state::AppState;
use tower_http::cors::{CorsLayer, Any};

mod state;
mod dtos;
mod handlers;
mod apidoc;
mod banner;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use apidoc::ApiDoc;

#[tokio::main(worker_threads = 4)]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Print colorful banner
    banner::print_banner();
    
    // Ensure DATABASE_URL is set
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:worpen.db?mode=rwc".to_string());
    
    // Initialize DB and run migrations
    let pool = initialize_db(&database_url).await.expect("Failed to initialize database");
    
    let repo = std::sync::Arc::new(SqliteAgentRepository::new(pool.clone()));
    let incident_repo = std::sync::Arc::new(infra::repositories::SqliteIncidentRepository::new(pool.clone()));
    let automation_repo = std::sync::Arc::new(infra::repositories::SqliteAutomationRepository::new(pool.clone()));

    let agent_service = std::sync::Arc::new(AgentService::new(repo.clone()));
    let dashboard_service = std::sync::Arc::new(worpen_core::services::DashboardService::new(repo.clone(), incident_repo.clone()));
    let docker_service = std::sync::Arc::new(worpen_core::services::DockerService::new());
    let incident_service = std::sync::Arc::new(worpen_core::services::IncidentService::new(incident_repo));
    let automation_service = std::sync::Arc::new(worpen_core::services::AutomationService::new(automation_repo));
    let pipeline_service = std::sync::Arc::new(worpen_core::services::PipelineService::new());
    let dynamic_route_service = std::sync::Arc::new(worpen_core::services::DynamicRouteService::new());
    
    let connected_agents = std::sync::Arc::new(dashmap::DashMap::new());
    
    let state = AppState {
        agent_service,
        dashboard_service,
        docker_service,
        incident_service,
        automation_service,
        pipeline_service,
        dynamic_route_service,
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
        // Dynamic Routes Engine
        .route("/api/v1/dynamic-routes", get(handlers::list_routes).post(handlers::register_route))
        .route("/api/v1/dynamic-routes/register", post(handlers::register_route_raw)) // YAML support
        .route("/api/v1/dynamic-routes/formats", get(handlers::get_format_stats)) // Format info
        .route("/api/v1/dynamic-routes/stats", get(handlers::get_route_stats))
        .route("/api/v1/dynamic-routes/test", post(handlers::test_route))
        .route("/api/v1/dynamic-routes/import", post(handlers::import_route))
        .route("/api/v1/dynamic-routes/:id", get(handlers::get_route).put(handlers::update_route).delete(handlers::delete_route))
        .route("/api/v1/dynamic-routes/:id/execute", post(handlers::execute_route))
        .route("/api/v1/dynamic-routes/:id/export", get(handlers::export_route))
        // WebSocket Dynamic Routes
        .route("/api/ws/*path", get(handlers::dynamic_ws_handler))
        // Global Functions for zero-cost inlining
        .route("/api/v1/global-functions", get(handlers::list_global_functions).post(handlers::define_global_function))
        // Fallback handler برای dynamic routes
        // این handler همه request های ثبت‌نشده رو میگیره
        .fallback(handlers::dynamic_route_fallback)
        .layer(cors)
        .with_state(state);

    let mut ip_str = String::new();
    let mut port_str = String::new();
    
    let (addr, listener) = loop {
        println!("\n🔍 Available local IP addresses:");
        println!("  - 127.0.0.1 (Localhost)");
        println!("  - 0.0.0.0 (All interfaces)");
        
        // Manual tip based on common interfaces
        println!("💡 Tip: Based on your system, you might want: 192.168.1.100 or 192.168.183.1");

        print!("\n🌐 Enter IP address to listen on (default 127.0.0.1): ");
        io::stdout().flush().unwrap();
        ip_str.clear();
        io::stdin().read_line(&mut ip_str).unwrap();
        let ip_trimmed = ip_str.trim();
        
        let ip = if ip_trimmed.is_empty() {
            "127.0.0.1".parse::<IpAddr>().unwrap()
        } else {
            match ip_trimmed.parse::<IpAddr>() {
                Ok(ip) => ip,
                Err(_) => {
                    println!("❌ Invalid IP address format. Please try again.");
                    continue;
                }
            }
        };

        print!("🔌 Enter Port (default 3000): ");
        io::stdout().flush().unwrap();
        port_str.clear();
        io::stdin().read_line(&mut port_str).unwrap();
        let port_trimmed = port_str.trim();
        
        let port = if port_trimmed.is_empty() {
            3000
        } else {
            match port_trimmed.parse::<u16>() {
                Ok(p) => p,
                Err(_) => {
                    println!("❌ Invalid port number. Please try again.");
                    continue;
                }
            }
        };

        let addr = SocketAddr::new(ip, port);
        
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                println!("🚀 Successfully bound to {}", addr);
                break (addr, listener);
            },
            Err(e) => {
                println!("❌ Could not bind to {}: {}", addr, e);
                println!("   (Make sure the IP belongs to this machine and the port is not in use)");
            }
        }
    };

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}


