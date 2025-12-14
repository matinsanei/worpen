use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use infra::{adapters::SqliteAgentRepository, initialize_db};
use core::services::AgentService;
use state::AppState;

mod state;
mod dtos;
mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Ensure DATABASE_URL is set
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:worpen.db?mode=rwc".to_string());
    
    // Initialize DB and run migrations
    let pool = initialize_db(&database_url).await.expect("Failed to initialize database");
    
    let repo = std::sync::Arc::new(SqliteAgentRepository::new(pool));
    let service = std::sync::Arc::new(AgentService::new(repo));
    
    let state = AppState {
        agent_service: service,
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/agents/register", post(handlers::register_agent))
        .route("/telemetry", post(handlers::receive_telemetry))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn health_check() -> &'static str {
    "OK"
}
