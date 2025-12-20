use std::sync::Arc;
use std::collections::HashMap;
use proto::models::{
    RouteDefinition, LogicOperation, RouteTestRequest, RouteTestResponse,
    DynamicRouteExecutionContext, LoopControl,
};
use serde_json::Value;
use crate::services::dynamic_routes_extended::execute_logic_extended;

pub struct DynamicRouteService {
    // In production, this would be a repository
    routes: Arc<std::sync::RwLock<HashMap<String, RouteDefinition>>>,
}

impl Default for DynamicRouteService {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicRouteService {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register a new dynamic route
    pub async fn register_route(&self, mut route: RouteDefinition) -> Result<String, String> {
        // Validate route
        self.validate_route(&route)?;
        
        // Generate ID if not provided
        if route.id.is_empty() {
            route.id = uuid::Uuid::new_v4().to_string();
        }
        
        // Set timestamps
        let now = chrono::Utc::now().to_rfc3339();
        route.created_at = now.clone();
        route.updated_at = now;
        
        // Store route
        let mut routes = self.routes.write().unwrap();
        let route_id = route.id.clone();
        routes.insert(route_id.clone(), route);
        
        Ok(route_id)
    }

    /// List all registered routes
    pub async fn list_routes(&self) -> Result<Vec<RouteDefinition>, String> {
        let routes = self.routes.read().unwrap();
        Ok(routes.values().cloned().collect())
    }

    /// Get a specific route by ID
    pub async fn get_route(&self, route_id: &str) -> Result<Option<RouteDefinition>, String> {
        let routes = self.routes.read().unwrap();
        Ok(routes.get(route_id).cloned())
    }

    /// Update an existing route
    pub async fn update_route(&self, route_id: &str, mut route: RouteDefinition) -> Result<(), String> {
        self.validate_route(&route)?;
        
        let mut routes = self.routes.write().unwrap();
        if !routes.contains_key(route_id) {
            return Err("Route not found".to_string());
        }
        
        route.updated_at = chrono::Utc::now().to_rfc3339();
        routes.insert(route_id.to_string(), route);
        
        Ok(())
    }

    /// Delete a route
    pub async fn delete_route(&self, route_id: &str) -> Result<(), String> {
        let mut routes = self.routes.write().unwrap();
        if routes.remove(route_id).is_none() {
            return Err("Route not found".to_string());
        }
        Ok(())
    }

    /// Test a route with test data
    pub async fn test_route(&self, request: RouteTestRequest) -> Result<RouteTestResponse, String> {
        let start_time = std::time::Instant::now();
        let mut steps = Vec::new();
        
        // Get route definition
        let route = {
            let routes = self.routes.read().unwrap();
            routes.get(&request.route_id).cloned()
                .ok_or_else(|| "Route not found".to_string())?
        };
        
        steps.push(format!("Route found: {} ({})", route.name, route.path));
        
        // Create execution context
        let context = DynamicRouteExecutionContext {
            route_id: request.route_id.clone(),
            variables: HashMap::new(),
            request_payload: request.test_payload,
            path_params: HashMap::new(),
            query_params: request.test_params.iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
            functions: HashMap::new(),
            loop_control: LoopControl::default(),
            error_context: None,
        };
        
        steps.push("Execution context created".to_string());
        
        // Execute logic
        match self.execute_logic(&route.logic, context, &mut steps).await {
            Ok(result) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                Ok(RouteTestResponse {
                    success: true,
                    result: Some(result),
                    error: None,
                    execution_time_ms: execution_time,
                    steps_executed: steps,
                })
            },
            Err(e) => {
                let execution_time = start_time.elapsed().as_millis() as u64;
                Ok(RouteTestResponse {
                    success: false,
                    result: None,
                    error: Some(e),
                    execution_time_ms: execution_time,
                    steps_executed: steps,
                })
            }
        }
    }

    /// Execute a dynamic route
    pub async fn execute_route(
        &self,
        route_id: &str,
        payload: Option<Value>,
        path_params: HashMap<String, String>,
        query_params: HashMap<String, String>,
    ) -> Result<Value, String> {
        let route = {
            let routes = self.routes.read().unwrap();
            routes.get(route_id).cloned()
                .ok_or_else(|| "Route not found".to_string())?
        };
        
        if !route.enabled {
            return Err("Route is disabled".to_string());
        }
        
        let context = DynamicRouteExecutionContext {
            route_id: route_id.to_string(),
            variables: HashMap::new(),
            request_payload: payload,
            path_params,
            query_params,
            functions: HashMap::new(),
            loop_control: LoopControl::default(),
            error_context: None,
        };
        
        let mut steps = Vec::new();
        let (result, _context) = execute_logic_extended(&route.logic, context, &mut steps).await?;
        Ok(result)
    }

    /// Legacy execute logic (now uses extended version)
    fn execute_logic<'a>(
        &'a self,
        operations: &'a [LogicOperation],
        context: DynamicRouteExecutionContext,
        steps: &'a mut Vec<String>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, String>> + Send + 'a>> {
        Box::pin(async move {
            let (result, _context) = execute_logic_extended(operations, context, steps).await?;
            Ok(result)
        })
    }

    /// Validate route definition
    fn validate_route(&self, route: &RouteDefinition) -> Result<(), String> {
        if route.name.is_empty() {
            return Err("Route name is required".to_string());
        }
        
        if route.path.is_empty() {
            return Err("Route path is required".to_string());
        }
        
        if !route.path.starts_with('/') {
            return Err("Route path must start with '/'".to_string());
        }
        
        if route.logic.is_empty() {
            return Err("Route logic cannot be empty".to_string());
        }
        
        Ok(())
    }



    /// Export route as JSON
    pub async fn export_route(&self, route_id: &str) -> Result<String, String> {
        let routes = self.routes.read().unwrap();
        let route = routes.get(route_id)
            .ok_or_else(|| "Route not found".to_string())?;
        
        serde_json::to_string_pretty(route)
            .map_err(|e| format!("Failed to serialize route: {}", e))
    }

    /// Import route from JSON
    pub async fn import_route(&self, json: &str) -> Result<String, String> {
        let route: RouteDefinition = serde_json::from_str(json)
            .map_err(|e| format!("Failed to parse route JSON: {}", e))?;
        
        self.register_route(route).await
    }

    /// Execute route logic با context
    pub async fn execute_route_logic(
        &self,
        logic: &[LogicOperation],
        context: &mut DynamicRouteExecutionContext,
    ) -> Result<Value, String> {
        let mut steps = Vec::new();
        let (result, _) = execute_logic_extended(logic, context.clone(), &mut steps).await?;
        Ok(result)
    }
}
