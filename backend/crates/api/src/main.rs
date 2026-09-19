use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use infra::{adapters::SqliteAgentRepository, initialize_db};
use worpen_core::services::AgentService;
use state::AppState;
use tower_http::cors::{Any, CorsLayer};

mod state;
mod dtos;
mod handlers;
mod apidoc;
mod auth;
mod banner;
mod cli;
mod ops;
mod mcp;
mod ui;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use apidoc::ApiDoc;

#[tokio::main(worker_threads = 4)]
async fn main() {
    // Load .env written by `worpen setup` (explicit env/flags always win).
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();
    let args = cli::parse();
    let cmd = args.command.unwrap_or(Commands::Serve(cli::ServeArgs {
        host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
        port: std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000),
        database_url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:worpen.db?mode=rwc".into()),
        data_dir: std::env::var("DATA_DIR").unwrap_or_else(|_| "auto".into()),
        no_ui: std::env::var("WORPEN_NO_UI").map(|v| v == "1" || v == "true").unwrap_or(false),
        ui_dir: std::env::var("FRONTEND_DIR").ok(),
    }));
    use cli::Commands;
    match cmd {
        Commands::Serve(a) => serve(a).await,
        Commands::Route { cmd } => {
            let res = match cmd {
                cli::RouteCmd::List(s) => ops::route_list(&s).await,
                cli::RouteCmd::Add(a) => ops::route_add(&a).await,
                cli::RouteCmd::Export { id, server } => ops::route_export(&id, &server).await,
                cli::RouteCmd::Delete { id, server } => ops::route_delete(&id, &server).await,
                cli::RouteCmd::Exec(a) => ops::route_exec(&a).await,
                cli::RouteCmd::Audit(a) => ops::route_audit(&a).await,
            };
            if let Err(e) = res {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Func { cmd } => {
            let res = match cmd {
                cli::FuncCmd::List(s) => ops::func_list(&s).await,
                cli::FuncCmd::Add(a) => ops::func_add(&a).await,
                cli::FuncCmd::Delete { name, server } => ops::func_delete(&name, &server).await,
            };
            if let Err(e) = res {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Doctor(a) => {
            if let Err(e) = ops::doctor(&a).await {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Mcp { cmd } => {
            let res = match cmd {
                cli::McpCmd::ServeStdio => mcp::serve_stdio().await,
                cli::McpCmd::Tools(s) => ops::mcp_tools(&s).await,
                cli::McpCmd::Call(a) => ops::mcp_call(&a).await,
            };
            if let Err(e) = res {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Pipeline { cmd } => {
            let res = match cmd {
                cli::PipeCmd::List(s) => ops::pipe_list(&s).await,
                cli::PipeCmd::Define(a) => ops::pipe_define(&a).await,
                cli::PipeCmd::Run(a) => ops::pipe_run(&a).await,
                cli::PipeCmd::Delete { id, server } => ops::pipe_delete(&id, &server).await,
                cli::PipeCmd::Runs(a) => ops::pipe_runs(&a).await,
            };
            if let Err(e) = res {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Setup => {
            if let Err(e) = ops::setup().await {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

async fn build_state(database_url: &str, data_dir: &str) -> AppState {
    let pool = initialize_db(database_url).await.expect("Failed to initialize database");
    let repo = std::sync::Arc::new(SqliteAgentRepository::new(pool.clone()));
    let incident_repo =
        std::sync::Arc::new(infra::repositories::SqliteIncidentRepository::new(pool.clone()));
    let automation_repo =
        std::sync::Arc::new(infra::repositories::SqliteAutomationRepository::new(pool.clone()));

    let agent_service = std::sync::Arc::new(AgentService::new(repo.clone()));
    let dashboard_service = std::sync::Arc::new(worpen_core::services::DashboardService::new(
        repo.clone(),
        incident_repo.clone(),
    ));
    let docker_service = std::sync::Arc::new(worpen_core::services::DockerService::new());
    let incident_service = std::sync::Arc::new(worpen_core::services::IncidentService::new(incident_repo));
    let automation_service = std::sync::Arc::new(worpen_core::services::AutomationService::new(
        automation_repo,
    ));
    // DATA_DIR respected by DynamicRouteService via env.
    // "auto" (default) lets the service pick ./data (inside backend/)
    // or ./backend/data (repo root) based on cwd.
    if data_dir == "auto" || data_dir.trim().is_empty() {
        std::env::remove_var("DATA_DIR");
    } else {
        std::env::set_var("DATA_DIR", data_dir);
    }
    let dynamic_route_service =
        std::sync::Arc::new(worpen_core::services::DynamicRouteService::new());
    // Pipelines execute stages through the same engine (inlining + audit).
    let pipeline_service = std::sync::Arc::new(worpen_core::services::PipelineService::with_executor(
        dynamic_route_service.clone(),
    ));
    let connected_agents = std::sync::Arc::new(dashmap::DashMap::new());
    AppState {
        agent_service,
        dashboard_service,
        docker_service,
        incident_service,
        automation_service,
        pipeline_service,
        dynamic_route_service,
        connected_agents,
    }
}

async fn serve(a: cli::ServeArgs) {
    banner::print_banner();
    let state = build_state(&a.database_url, &a.data_dir).await;
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let ui_dir_opt: Option<String> = if a.no_ui {
        None
    } else {
        ui::find_ui_dir(a.ui_dir.clone())
    };
    if a.no_ui {
        println!("🖥️  UI disabled (--no-ui) — headless CLI mode");
    } else if let Some(ref d) = ui_dir_opt {
        println!("🖥️  UI enabled from {}", d);
    } else {
        println!("🖥️  UI dist not found — headless mode (build with `npm run build`)");
    }

    let api = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // NOTE: "/" is intentionally NOT registered here — when UI is enabled the
        // unified fallback serves index.html; headless mode falls through to dynamic routes.
        .route("/health", get(handlers::health_check))
        .route("/mcp", post(mcp::mcp_handler).get(mcp::mcp_info))
        .route("/mcp/tools", get(mcp::mcp_tools_rest))
        .route("/api/v1/agents/register", post(handlers::register_agent))
        .route("/api/v1/telemetry", post(handlers::receive_telemetry))
        .route("/api/v1/ws", get(handlers::ws_handler))
        .route("/api/v1/dashboard/stats", get(handlers::get_stats))
        .route("/api/v1/dashboard/network", get(handlers::get_network_stats))
        .route("/api/v1/dashboard/services", get(handlers::get_services_health))
        .route("/api/v1/dashboard/load", get(handlers::get_load_distribution))
        .route("/api/v1/agents", get(handlers::list_agents))
        .route("/api/v1/agents/:id", get(handlers::get_agent))
        .route("/api/v1/agents/:id/exec", post(handlers::exec_on_agent))
        .route("/api/v1/agents/:id/shell", post(handlers::start_agent_shell))
        .route("/api/v1/agents/:id/processes", get(handlers::list_agent_processes))
        .route("/api/v1/agents/:id/system", get(handlers::get_agent_system_info))
        .route("/api/v1/agents/sync", post(handlers::sync_agents))
        .route("/api/v1/internal/heartbeat", post(handlers::agent_heartbeat))
        .route("/api/v1/containers", get(handlers::list_containers))
        .route("/api/v1/containers/:id/:action", post(handlers::container_action))
        .route("/api/v1/containers/:id/logs", get(handlers::get_container_logs))
        .route("/api/v1/containers/:id/start", post(handlers::start_container))
        .route("/api/v1/containers/:id/stop", post(handlers::stop_container))
        .route("/api/v1/containers/:id/restart", post(handlers::restart_container))
        .route(
            "/api/v1/containers/:id",
            axum::routing::delete(handlers::delete_container),
        )
        .route("/api/v1/containers/:id/exec", post(handlers::exec_in_container))
        .route("/api/v1/containers/:id/shell", post(handlers::start_container_shell))
        .route("/api/v1/containers/:id/attach", get(handlers::attach_to_container))
        .route("/api/v1/containers/:id/inspect", get(handlers::inspect_container))
        .route("/api/v1/docker/prune", post(handlers::prune_system))
        .route("/api/v1/images", get(handlers::list_images))
        .route("/api/v1/images/pull", post(handlers::pull_image))
        .route("/api/v1/incidents", get(handlers::list_incidents))
        .route("/api/v1/incidents/:id/resolve", post(handlers::resolve_incident))
        .route(
            "/api/v1/automation/rules",
            get(handlers::list_rules).post(handlers::create_rule),
        )
        .route(
            "/api/v1/automation/rules/:id",
            get(handlers::get_rule).put(handlers::update_rule),
        )
        .route("/api/v1/automation/dry-run", post(handlers::dry_run))
        .route(
            "/api/v1/pipelines",
            get(handlers::list_pipelines).post(handlers::define_pipeline),
        )
        .route("/api/v1/pipelines/trigger", post(handlers::trigger_pipeline))
        .route("/api/v1/pipelines/runs", get(handlers::pipeline_runs))
        .route(
            "/api/v1/pipelines/:id",
            get(handlers::get_pipeline).delete(handlers::delete_pipeline),
        )
        .route("/api/v1/pipelines/:id/runs", get(handlers::pipeline_runs_by_id))
        .route("/api/v1/logs", get(handlers::get_logs))
        .route("/api/v1/terminal/container/:id", get(handlers::container_terminal_ws))
        .route("/api/v1/terminal/agent/:id", get(handlers::agent_terminal_ws))
        .route("/api/v1/terminal/sessions", get(handlers::list_terminal_sessions))
        .route(
            "/api/v1/dynamic-routes",
            get(handlers::list_routes).post(handlers::register_route),
        )
        .route("/api/v1/dynamic-routes/register", post(handlers::register_route_raw))
        .route("/api/v1/dynamic-routes/formats", get(handlers::get_format_stats))
        .route("/api/v1/dynamic-routes/audit", get(handlers::get_audit))
        .route("/api/v1/dynamic-routes/stats", get(handlers::get_route_stats))
        .route("/api/v1/dynamic-routes/test", post(handlers::test_route))
        .route("/api/v1/dynamic-routes/import", post(handlers::import_route))
        .route(
            "/api/v1/dynamic-routes/:id",
            get(handlers::get_route).put(handlers::update_route).delete(handlers::delete_route),
        )
        .route("/api/v1/dynamic-routes/:id/execute", post(handlers::execute_route))
        .route("/api/v1/dynamic-routes/:id/export", get(handlers::export_route))
        .route("/api/ws/*path", get(handlers::dynamic_ws_handler))
        .route(
            "/api/v1/global-functions",
            get(handlers::list_global_functions).post(handlers::define_global_function),
        )
        .route(
            "/api/v1/global-functions/:name",
            get(handlers::get_global_function).delete(handlers::delete_global_function),
        )
        .layer(cors)
        .with_state(state.clone());

    // Unified fallback: try UI static file → dynamic route → 404.
    let ui_dir = ui_dir_opt.clone().unwrap_or_default();
    let ui_enabled = ui_dir_opt.is_some();
    let app = api.fallback(move |req: Request<Body>| {
        let state = state.clone();
        let ui_dir = ui_dir.clone();
        async move {
            let path = req.uri().path().to_string();
            if ui_enabled {
                if let Some(resp) = ui::serve_ui_file(&ui_dir, &path).await {
                    return resp;
                }
            }
            // Delegate everything else to the dynamic-route fallback (also 404s cleanly)
            handlers::dynamic_route_fallback(axum::extract::State(state), None, req).await
        }
    });

    let addr: SocketAddr = format!("{}:{}", a.host, a.port)
        .parse()
        .expect("invalid host/port");
    println!("🚀 Worpen listening on http://{} (UI: {})", addr, if ui_enabled { "on" } else { "off" });
    println!("   API: http://{}/api/v1/dynamic-routes | MCP: http://{}/mcp | Swagger: http://{}/swagger-ui", addr, addr, addr);
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind failed");
    axum::serve(listener, app).await.unwrap();
}

// Keep old handler paths compiling (dead-code guard for unused imports)
#[allow(dead_code)]
fn _hold_status() -> StatusCode {
    StatusCode::OK
}
#[allow(dead_code)]
fn _hold_resp() -> Response {
    StatusCode::OK.into_response()
}
