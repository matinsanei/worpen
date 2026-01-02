use worpen_core::services::dynamic_routes::service::DynamicRouteService;
use proto::models::{RouteDefinition, HttpMethod, LogicOperation};
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn test_hot_routes_cache() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        // Setup service with a temporary directory
        let temp_dir_path = std::env::temp_dir().join(format!("worpen_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir_path).unwrap();
        let data_dir = temp_dir_path.to_str().unwrap().to_string();
        let service = DynamicRouteService::with_data_dir(data_dir.clone());

        // Create a simple route
        let route = RouteDefinition {
            id: "bench_route".to_string(),
            name: "Benchmark Route".to_string(),
            description: "Test route for caching".to_string(),
            path: "/bench".to_string(),
            method: HttpMethod::GET,
            logic: vec![
                LogicOperation::Return { value: Value::String("Success".to_string()) }
            ],
            parameters: vec![],
            response_schema: None,
            auth_required: false,
            rate_limit: None,
            enabled: true,
            version: "1.0.0".to_string(),
            route_type: proto::models::RouteType::Http,
            ws_hooks: None,
            created_at: "".to_string(),
            updated_at: "".to_string(),
            created_by: "system".to_string(),
        };

        // Register route
        let _ = service.register_route(route).await.expect("Failed to register route");

        // First execution (Cold Path - populates cache)
        let start = std::time::Instant::now();
        let result1 = service.execute_route("bench_route", None, HashMap::new(), HashMap::new()).await;
        let duration1 = start.elapsed();
        assert_eq!(result1.unwrap(), Value::String("Success".to_string()));
        println!("First execution time (Cold): {:?}", duration1);

        // Second execution (Hot Path - checks cache)
        let start = std::time::Instant::now();
        let result2 = service.execute_route("bench_route", None, HashMap::new(), HashMap::new()).await;
        let duration2 = start.elapsed();
        assert_eq!(result2.unwrap(), Value::String("Success".to_string()));
        println!("Second execution time (Hot): {:?}", duration2);

        // Run 10,000 times to simulate "hot" usage and verify stability
        let start = std::time::Instant::now();
        for _ in 0..10_000 {
            let _ = service.execute_route("bench_route", None, HashMap::new(), HashMap::new()).await;
        }
        let duration_loop = start.elapsed();
        println!("10,000 executions time: {:?} (Avg: {:?}/op)", duration_loop, duration_loop / 10_000);
    });
}
