use utoipa::OpenApi;
use crate::dtos::{RegisterAgentRequest, TelemetryRequest};
use crate::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::register_agent,
        handlers::receive_telemetry,
        handlers::health_check,
        handlers::ws::ws_handler
    ),
    components(
        schemas(RegisterAgentRequest, TelemetryRequest)
    ),
    tags(
        (name = "worpen", description = "Worpen Hive Backend API")
    )
)]
pub struct ApiDoc;
