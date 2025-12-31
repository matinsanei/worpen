use std::sync::Arc;
use std::collections::HashMap;
use std::cmp::Reverse;
use proto::models::{
    RouteDefinition, LogicOperation, RouteTestRequest, RouteTestResponse,
    DynamicRouteExecutionContext, LoopControl, FunctionDef, FunctionDefinition, SwitchCase,
};
use serde_json::Value;
use regex;
use super::execution::execute_logic_extended;
use super::cache::ExecutionPlan;
use crate::compiler::lowerer::LogicCompiler;
use crate::vm::machine::VirtualMachine;
use crate::vm::memory::ExecutionMemory;

pub struct DynamicRouteService {
    // In production, this would be a repository
    routes: Arc<std::sync::RwLock<HashMap<String, RouteDefinition>>>,
    // Global function registry for zero-cost inlining
    global_functions: Arc<std::sync::RwLock<HashMap<String, FunctionDef>>>,
    // Hot routes cache for optimized execution
    hot_routes_cache: Arc<std::sync::RwLock<HashMap<String, Arc<ExecutionPlan>>>>,
    data_dir: String,
}

impl Default for DynamicRouteService {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicRouteService {
    pub fn new() -> Self {
        Self::with_data_dir("data".to_string())
    }

    pub fn with_data_dir(data_dir: String) -> Self {
        let service = Self {
            routes: Arc::new(std::sync::RwLock::new(HashMap::new())),
            global_functions: Arc::new(std::sync::RwLock::new(HashMap::new())),
            hot_routes_cache: Arc::new(std::sync::RwLock::new(HashMap::new())),
            data_dir,
        };
        // Load persisted data
        let _ = service.load_persisted_data();
        service
    }

    /// Register a new dynamic route
    pub async fn register_route(&self, mut route: RouteDefinition) -> Result<String, String> {
        // Validate route
        self.validate_route(&route)?;
        
        // Generate ID if not provided
        if route.id.is_empty() {
            route.id = uuid::Uuid::new_v4().to_string();
        }
        
        // Inline global function calls for zero-cost abstraction
        route.logic = self.inline_logic(&route.logic, 0)?;
        
        // Set timestamps
        let now = chrono::Utc::now().to_rfc3339();
        route.created_at = now.clone();
        route.updated_at = now;
        
        // Pre-compile execution plan for hot cache
        let execution_plan = self.compile_execution_plan(&route)?;
        
        // Store route
        let mut routes = self.routes.write().unwrap();
        let route_id = route.id.clone();
        routes.insert(route_id.clone(), route.clone());
        
        // Cache the execution plan
        let mut cache = self.hot_routes_cache.write().unwrap();
        cache.insert(route_id.clone(), Arc::new(execution_plan));
        
        // Persist to disk asynchronously
        let route_clone = route.clone();
        let data_dir_clone = self.data_dir.clone();
        tokio::spawn(async move {
            drop(Self::save_route_async(&route_clone, &data_dir_clone));
        });
        
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

    /// Compile an execution plan for a route (pre-computation for hot cache)
    fn compile_execution_plan(&self, route: &RouteDefinition) -> Result<ExecutionPlan, String> {
        // Compile to bytecode for VM execution
        let (bytecode, symbol_table) = self.compile_logic(&route.logic)?;
        
        Ok(ExecutionPlan {
            logic: Arc::new(route.logic.clone()),
            enabled: route.enabled,
            version: route.version.clone(),
            bytecode: bytecode.map(Arc::new),
            symbol_table: symbol_table.map(Arc::new),
        })
    }

    /// Save a route to disk asynchronously
    async fn save_route_async(route: &RouteDefinition, data_dir: &str) -> Result<(), String> {
        use std::fs;
        use std::path::Path;

        let routes_dir = Path::new(data_dir).join("routes");
        fs::create_dir_all(&routes_dir)
            .map_err(|e| format!("Failed to create routes dir: {}", e))?;

        let file_path = routes_dir.join(format!("{}.json", route.id));
        let content = serde_json::to_string_pretty(route)
            .map_err(|e| format!("Failed to serialize route: {}", e))?;
        fs::write(&file_path, content)
            .map_err(|e| format!("Failed to write route file {}: {}", file_path.display(), e))?;

        Ok(())
    }

    /// Delete a route file from disk asynchronously
    async fn delete_route_file_async(route_id: &str, data_dir: &str) -> Result<(), String> {
        use std::fs;
        use std::path::Path;

        let routes_dir = Path::new(data_dir).join("routes");
        let file_path = routes_dir.join(format!("{}.json", route_id));
        
        if file_path.exists() {
            fs::remove_file(&file_path)
                .map_err(|e| format!("Failed to delete route file {}: {}", file_path.display(), e))?;
        }

        Ok(())
    }

    /// Update an existing route
    pub async fn update_route(&self, route_id: &str, mut route: RouteDefinition) -> Result<(), String> {
        self.validate_route(&route)?;
        
        let mut routes = self.routes.write().unwrap();
        if !routes.contains_key(route_id) {
            return Err("Route not found".to_string());
        }
        
        route.updated_at = chrono::Utc::now().to_rfc3339();
        routes.insert(route_id.to_string(), route.clone());
        
        // Pre-compile and cache the execution plan
        let execution_plan = self.compile_execution_plan(&route)?;
        let mut cache = self.hot_routes_cache.write().unwrap();
        cache.insert(route_id.to_string(), Arc::new(execution_plan));
        
        // Persist to disk asynchronously
        let route_clone = route.clone();
        let data_dir_clone = self.data_dir.clone();
        tokio::spawn(async move {
            drop(Self::save_route_async(&route_clone, &data_dir_clone));
        });
        
        Ok(())
    }

    /// Delete a route
    pub async fn delete_route(&self, route_id: &str) -> Result<(), String> {
        let mut routes = self.routes.write().unwrap();
        if routes.remove(route_id).is_none() {
            return Err("Route not found".to_string());
        }
        
        // Invalidate cache
        let mut cache = self.hot_routes_cache.write().unwrap();
        cache.remove(route_id);

        // Delete from disk asynchronously
        let route_id_clone = route_id.to_string();
        let data_dir_clone = self.data_dir.clone();
        tokio::spawn(async move {
            drop(Self::delete_route_file_async(&route_id_clone, &data_dir_clone));
        });
        
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
        // Fast path: Check hot cache first (should always be populated now)
        let cached_plan = {
            let cache = self.hot_routes_cache.read().unwrap();
            cache.get(route_id).cloned()
        };

        let plan = match cached_plan {
            Some(p) => p,
            None => {
                // Fallback: Route not in cache (should not happen in normal operation)
                // This maintains backward compatibility but should be rare
                let routes = self.routes.read().unwrap();
                let route = routes.get(route_id)
                    .ok_or_else(|| "Route not found".to_string())?;
                
                // Critical Optimization: Flatten logic tree (inline functions) before caching
                let optimized_logic = self.inline_logic(&route.logic, 0)?;
                
                // Compile to bytecode for VM execution
                let (bytecode, symbol_table) = self.compile_logic(&optimized_logic)?;
                    
                Arc::new(ExecutionPlan {
                    logic: Arc::new(optimized_logic),
                    enabled: route.enabled,
                    version: route.version.clone(),
                    bytecode: bytecode.map(Arc::new),
                    symbol_table: symbol_table.map(Arc::new),
                })
            }
        };
        
        if !plan.enabled {
            return Err("Route is disabled".to_string());
        }
        
        // Execute using VM if bytecode is available, otherwise fallback to interpreter
        let result = if let (Some(bytecode), Some(symbol_table)) = (&plan.bytecode, &plan.symbol_table) {
            // VM execution path
            let memory = ExecutionMemory::new();
            let mut vm = VirtualMachine::new(memory, (**symbol_table).clone());
            
            // Bridge request data to VM memory
            self.inject_request_data_into_vm(&mut vm, &payload, &path_params, &query_params, symbol_table)?;
            
            // Execute bytecode
            vm.execute(bytecode).await?
        } else {
            // Fallback to interpreter (not measured for telemetry as it's legacy path)
            let mut context = DynamicRouteExecutionContext {
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
            execute_logic_extended(&plan.logic, &mut context, &mut steps).await?
        };
        Ok(result)
    }

    /// Legacy execute logic (now uses extended version)
    fn execute_logic<'a>(
        &'a self,
        operations: &'a [LogicOperation],
        mut context: DynamicRouteExecutionContext,
        steps: &'a mut Vec<String>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, String>> + Send + 'a>> {
        Box::pin(async move {
            let result = execute_logic_extended(operations, &mut context, steps).await?;
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

    /// Define a global function for zero-cost inlining
    pub async fn define_global_function(&self, func: FunctionDef) -> Result<(), String> {
        let mut functions = self.global_functions.write().unwrap();
        functions.insert(func.name.clone(), func.clone());
        // Persist to disk
        self.save_function(&func)?;
        Ok(())
    }

    /// Get all global functions
    pub async fn get_global_functions(&self) -> HashMap<String, FunctionDef> {
        let functions = self.global_functions.read().unwrap();
        functions.clone()
    }

    /// Get a specific global function
    pub async fn get_global_function(&self, name: &str) -> Option<FunctionDef> {
        let functions = self.global_functions.read().unwrap();
        functions.get(name).cloned()
    }

    /// Inline global function calls in logic operations for zero-cost abstraction
    /// This function processes call_function operations BEFORE execution phase (during route creation/registration)
    /// The algorithm is recursive with depth limiting for security
    pub fn inline_logic(&self, logic: &[LogicOperation], depth: usize) -> Result<Vec<LogicOperation>, String> {
        // Security Check: Prevent Stack Overflow
        if depth > 50 {
            return Err("Maximum recursion depth exceeded (50) - possible circular function calls".to_string());
        }

        let mut result = Vec::new();
        let functions = self.global_functions.read().unwrap();

        for operation in logic {
            match operation {
                LogicOperation::DefineFunction { name, params, body } => {
                    // Register the function in the global registry
                    let mut functions = self.global_functions.write().unwrap();
                    functions.insert(name.clone(), FunctionDef {
                        name: name.clone(),
                        params: params.clone(),
                        logic: body.clone(),
                    });
                    // Don't add to result - function definition is processed at registration time
                },
                LogicOperation::CallFunction { name, args, output_var } => {
                    // Look up the function logic from the global store
                    if let Some(func_def) = functions.get(name) {
                        // Extract variables from the function being called for proper scoping
                        let function_variables = self.extract_function_variables(func_def);
                        
                        // Create unique variable scope for this function call
                        let scope_prefix = format!("_{}_", depth);
                        
                        // Create Set operations to map args to scoped function parameters
                        for (i, param) in func_def.params.iter().enumerate() {
                            if let Some(arg) = args.get(i) {
                                let scoped_param = format!("{}{}", scope_prefix, param);
                                result.push(LogicOperation::Set {
                                    var: scoped_param,
                                    value: arg.clone(),
                                });
                            }
                        }

                        // Recursively inline the function's logic with scoped variables
                        let scoped_body = self.inline_scoped_logic(&func_def.logic, &scope_prefix, depth + 1, &function_variables)?;
                        
                        // Inject the scoped body into the current operation list
                        result.extend(scoped_body);

                        // Map the scoped result to the output_var
                        let scoped_result = format!("{}result", scope_prefix);
                        result.push(LogicOperation::Set {
                            var: output_var.clone(),
                            value: serde_json::Value::String(format!("${{{}}}", scoped_result)),
                        });
                    } else {
                        return Err(format!("Function '{}' not found in global registry", name));
                    }
                },
                // Handle nested operations recursively
                LogicOperation::If { condition, then, otherwise } => {
                    let flattened_then = self.inline_logic(then, depth + 1)?;
                    let flattened_otherwise = if let Some(otherwise_ops) = otherwise {
                        Some(self.inline_logic(otherwise_ops, depth + 1)?)
                    } else {
                        None
                    };
                    result.push(LogicOperation::If {
                        condition: condition.clone(),
                        then: flattened_then,
                        otherwise: flattened_otherwise,
                    });
                },
                LogicOperation::Loop { collection, var, body } => {
                    let flattened_body = self.inline_logic(body, depth + 1)?;
                    result.push(LogicOperation::Loop {
                        collection: collection.clone(),
                        var: var.clone(),
                        body: flattened_body,
                    });
                },
                LogicOperation::Switch { value, cases, default } => {
                    let flattened_cases = cases.iter().map(|case| {
                        Ok(SwitchCase {
                            value: case.value.clone(),
                            operations: self.inline_logic(&case.operations, depth + 1)?,
                        })
                    }).collect::<Result<Vec<_>, String>>()?;

                    let flattened_default = if let Some(default_ops) = default {
                        Some(self.inline_logic(default_ops, depth + 1)?)
                    } else {
                        None
                    };

                    result.push(LogicOperation::Switch {
                        value: value.clone(),
                        cases: flattened_cases,
                        default: flattened_default,
                    });
                },
                LogicOperation::While { condition, body, max_iterations } => {
                    let flattened_body = self.inline_logic(body, depth + 1)?;
                    result.push(LogicOperation::While {
                        condition: condition.clone(),
                        body: flattened_body,
                        max_iterations: *max_iterations,
                    });
                },
                LogicOperation::Try { body, catch, finally } => {
                    let flattened_body = self.inline_logic(body, depth + 1)?;
                    let flattened_catch = self.inline_logic(catch, depth + 1)?;
                    let flattened_finally = if let Some(finally_ops) = finally {
                        Some(self.inline_logic(finally_ops, depth + 1)?)
                    } else {
                        None
                    };
                    result.push(LogicOperation::Try {
                        body: flattened_body,
                        catch: flattened_catch,
                        finally: flattened_finally,
                    });
                },
                LogicOperation::Parallel { tasks, max_concurrent } => {
                    let flattened_tasks = tasks.iter()
                        .map(|task| self.inline_logic(task, depth + 1))
                        .collect::<Result<Vec<_>, String>>()?;
                    result.push(LogicOperation::Parallel {
                        tasks: flattened_tasks,
                        max_concurrent: *max_concurrent,
                    });
                },
                // For all other operations, just push them as is
                _ => result.push(operation.clone()),
            }
        }

        Ok(result)
    }

    /// Helper method to inline logic with variable scoping
    fn inline_scoped_logic(&self, logic: &[LogicOperation], scope_prefix: &str, depth: usize, variables: &[String]) -> Result<Vec<LogicOperation>, String> {
        if depth > 50 {
            return Err("Maximum recursion depth exceeded (50) - possible circular function calls".to_string());
        }

        let mut result = Vec::new();
        let functions = self.global_functions.read().unwrap();

        for operation in logic {
            match operation {
                LogicOperation::DefineFunction { name, params, body } => {
                    // Register the function in the global registry (even from within scoped logic)
                    let mut functions = self.global_functions.write().unwrap();
                    functions.insert(name.clone(), FunctionDef {
                        name: name.clone(),
                        params: params.clone(),
                        logic: body.clone(),
                    });
                    // Don't add to result - function definition is processed at registration time
                },
                LogicOperation::Set { var, value } => {
                    // Scope the variable name
                    let scoped_var = format!("{}{}", scope_prefix, var);
                    let scoped_value = self.scope_value_references(value, scope_prefix, variables);
                    result.push(LogicOperation::Set {
                        var: scoped_var,
                        value: scoped_value,
                    });
                },
                LogicOperation::CallFunction { name, args, output_var } => {
                    // Look up the function logic from the global store
                    if let Some(func_def) = functions.get(name) {
                        // Extract variables from the function being called
                        let function_variables = self.extract_function_variables(func_def);
                        
                        // Create deeper scope for nested function call
                        let nested_prefix = format!("{}{}_", scope_prefix, depth);
                        
                        // Create Set operations to map args to scoped function parameters
                        for (i, param) in func_def.params.iter().enumerate() {
                            if let Some(arg) = args.get(i) {
                                let scoped_param = format!("{}{}", nested_prefix, param);
                                let scoped_arg = self.scope_value_references(arg, scope_prefix, variables);
                                result.push(LogicOperation::Set {
                                    var: scoped_param,
                                    value: scoped_arg,
                                });
                            }
                        }

                        // Recursively inline the function's logic with deeper scoping
                        let scoped_body = self.inline_scoped_logic(&func_def.logic, &nested_prefix, depth + 1, &function_variables)?;
                        
                        // Inject the scoped body
                        result.extend(scoped_body);

                        // Map the scoped result to the scoped output_var
                        let scoped_output = format!("{}{}", scope_prefix, output_var);
                        let nested_result = format!("{}result", nested_prefix);
                        result.push(LogicOperation::Set {
                            var: scoped_output,
                            value: serde_json::Value::String(format!("${{{}}}", nested_result)),
                        });
                    } else {
                        return Err(format!("Function '{}' not found in global registry", name));
                    }
                },
                // For other operations, just scope any variable references in their expressions
                LogicOperation::If { condition, then, otherwise } => {
                    let scoped_condition = self.scope_string_references(condition, scope_prefix, variables);
                    let scoped_then = self.inline_scoped_logic(then, scope_prefix, depth, variables)?;
                    let scoped_otherwise = if let Some(ops) = otherwise {
                        Some(self.inline_scoped_logic(ops, scope_prefix, depth, variables)?)
                    } else {
                        None
                    };
                    result.push(LogicOperation::If {
                        condition: scoped_condition,
                        then: scoped_then,
                        otherwise: scoped_otherwise,
                    });
                },
                LogicOperation::Return { value } => {
                    let scoped_value = self.scope_value_references(value, scope_prefix, variables);
                    result.push(LogicOperation::Return { value: scoped_value });
                },
                LogicOperation::MathOp { operation, args } => {
                    let scoped_args = args.iter()
                        .map(|arg| self.scope_value_references(arg, scope_prefix, variables))
                        .collect();
                    result.push(LogicOperation::MathOp {
                        operation: operation.clone(),
                        args: scoped_args,
                    });
                },
                // For simplicity, handle other operations by scoping string fields
                _ => {
                    // For now, just pass through other operations (they would need individual handling)
                    result.push(operation.clone());
                }
            }
        }

        Ok(result)
    }

    /// Helper to scope variable references in a Value
    fn scope_value_references(&self, value: &serde_json::Value, scope_prefix: &str, variables: &[String]) -> serde_json::Value {
        match value {
            serde_json::Value::String(s) => serde_json::Value::String(self.scope_string_references(s, scope_prefix, variables)),
            serde_json::Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(|v| self.scope_value_references(v, scope_prefix, variables)).collect())
            },
            serde_json::Value::Object(obj) => {
                let mut scoped_obj = serde_json::Map::new();
                for (k, v) in obj {
                    scoped_obj.insert(k.clone(), self.scope_value_references(v, scope_prefix, variables));
                }
                serde_json::Value::Object(scoped_obj)
            },
            _ => value.clone(),
        }
    }

    /// Helper to extract all variable names used within a function's scope
    fn extract_function_variables(&self, func_def: &FunctionDef) -> Vec<String> {
        let mut variables = std::collections::HashSet::new();
        
        // Add function parameters
        for param in &func_def.params {
            variables.insert(param.clone());
        }
        
        // Scan function logic for all variable references
        self.scan_logic_for_variables(&func_def.logic, &mut variables);
        
        // Convert to sorted Vec for consistent ordering
        let mut result: Vec<String> = variables.into_iter().collect();
        result.sort();
        result
    }

    /// Helper to recursively scan logic operations for variable references
    fn scan_logic_for_variables(&self, logic: &[LogicOperation], variables: &mut std::collections::HashSet<String>) {
        for operation in logic {
            match operation {
                LogicOperation::DefineFunction { name: _, params, body } => {
                    // Add parameters to variables
                    for param in params {
                        variables.insert(param.clone());
                    }
                    // Scan function body for variables
                    self.scan_logic_for_variables(body, variables);
                },
                LogicOperation::Set { var, value } => {
                    variables.insert(var.clone());
                    self.scan_value_for_variables(value, variables);
                },
                LogicOperation::CallFunction { name: _, args, output_var } => {
                    variables.insert(output_var.clone());
                    for arg in args {
                        self.scan_value_for_variables(arg, variables);
                    }
                },
                LogicOperation::If { condition, then, otherwise } => {
                    self.scan_string_for_variables(condition, variables);
                    self.scan_logic_for_variables(then, variables);
                    if let Some(otherwise_ops) = otherwise {
                        self.scan_logic_for_variables(otherwise_ops, variables);
                    }
                },
                LogicOperation::Loop { collection, var, body } => {
                    variables.insert(var.clone());
                    self.scan_string_for_variables(collection, variables);
                    self.scan_logic_for_variables(body, variables);
                },
                LogicOperation::While { condition, body, .. } => {
                    self.scan_string_for_variables(condition, variables);
                    self.scan_logic_for_variables(body, variables);
                },
                LogicOperation::Switch { value, cases, default } => {
                    self.scan_string_for_variables(value, variables);
                    for case in cases {
                        self.scan_value_for_variables(&case.value, variables);
                        self.scan_logic_for_variables(&case.operations, variables);
                    }
                    if let Some(default_ops) = default {
                        self.scan_logic_for_variables(default_ops, variables);
                    }
                },
                LogicOperation::Try { body, catch, finally } => {
                    self.scan_logic_for_variables(body, variables);
                    self.scan_logic_for_variables(catch, variables);
                    if let Some(finally_ops) = finally {
                        self.scan_logic_for_variables(finally_ops, variables);
                    }
                },
                LogicOperation::Parallel { tasks, .. } => {
                    for task in tasks {
                        self.scan_logic_for_variables(task, variables);
                    }
                },
                LogicOperation::Return { value } => {
                    self.scan_value_for_variables(value, variables);
                },
                LogicOperation::MathOp { operation: _, args } => {
                    for arg in args {
                        self.scan_value_for_variables(arg, variables);
                    }
                },
                _ => {} // Other operations might not contain variable references
            }
        }
    }

    /// Helper to scan a Value for variable references
    fn scan_value_for_variables(&self, value: &serde_json::Value, variables: &mut std::collections::HashSet<String>) {
        match value {
            serde_json::Value::String(s) => {
                self.scan_string_for_variables(s, variables);
            },
            serde_json::Value::Array(arr) => {
                for v in arr {
                    self.scan_value_for_variables(v, variables);
                }
            },
            serde_json::Value::Object(obj) => {
                for (_, v) in obj {
                    self.scan_value_for_variables(v, variables);
                }
            },
            _ => {}
        }
    }

    /// Helper to scan a string for variable references (like ${var_name})
    fn scan_string_for_variables(&self, s: &str, variables: &mut std::collections::HashSet<String>) {
        use regex::Regex;
        
        // Pattern to match ${variable_name} expressions
        if let Ok(re) = Regex::new(r"\$\{([^}]+)\}") {
            for cap in re.captures_iter(s) {
                if let Some(var_expr) = cap.get(1) {
                    let expr = var_expr.as_str();
                    // Extract variable names from the expression
                    // This is a simple approach - split by operators and extract word-like tokens
                    let tokens: Vec<&str> = expr.split(|c: char| !c.is_alphanumeric() && c != '_')
                        .filter(|s| !s.is_empty())
                        .collect();
                    for token in tokens {
                        // Only consider tokens that start with a letter or underscore (proper variable names)
                        if !token.is_empty() && 
                           token.chars().all(|c| c.is_alphanumeric() || c == '_') &&
                           token.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_') {
                            variables.insert(token.to_string());
                        }
                    }
                }
            }
        }
    }

    /// Helper to scope variable references in a string expression
    fn scope_string_references(&self, s: &str, scope_prefix: &str, variables: &[String]) -> String {
        use regex::Regex;

        let mut result = s.to_string();

        // 1. Sort variables by length descending to prevent partial replacements of overlapping names
        // (e.g. replacing 'val' inside 'value')
        let mut sorted_vars = variables.to_vec();
        sorted_vars.sort_by_key(|v| Reverse(v.len()));

        for var in sorted_vars {
            // Pattern: match 'var' only if it's NOT preceded by another variable character or '${'
            // and NOT followed by another variable character.
            // Simplified: \b{var}\b (word boundary) works for most cases
            
            // Fix for your specific case: "${product + 10}" -> "${_0_product + 10}"
            // We use standard word boundaries.
            let pattern = format!(r"\b{}\b", regex::escape(&var));
            
            if let Ok(re) = Regex::new(&pattern) {
                let replacement = format!("{}{}", scope_prefix, var);
                result = re.replace_all(&result, replacement.as_str()).to_string();
            }
        }

        result
    }

    /// Execute route logic با context
    pub async fn execute_route_logic(
        &self,
        logic: &[LogicOperation],
        context: &mut DynamicRouteExecutionContext,
    ) -> Result<Value, String> {
        // Inject global functions into context for execution
        let global_functions = self.global_functions.read().unwrap().clone();
        for (name, func_def) in global_functions.iter() {
            context.functions.insert(name.clone(), FunctionDefinition {
                params: func_def.params.clone(),
                body: func_def.logic.clone(),
            });
        }
        
        let mut steps = Vec::new();
        let result = execute_logic_extended(logic, context, &mut steps).await?;
        Ok(result)
    }

    /// Load persisted routes and functions from disk
    fn load_persisted_data(&self) -> Result<(), String> {
        use std::fs;
        use std::path::Path;

        let routes_dir = Path::new(&self.data_dir).join("routes");
        let functions_dir = Path::new(&self.data_dir).join("functions");

        // Load routes
        if routes_dir.exists() {
            let entries = fs::read_dir(&routes_dir)
                .map_err(|e| format!("Failed to read routes dir: {}", e))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read route file {}: {}", path.display(), e))?;
                    let route: RouteDefinition = serde_json::from_str(&content)
                        .map_err(|e| format!("Failed to parse route from {}: {}", path.display(), e))?;
                    
                    // Pre-compile execution plan for hot cache
                    let execution_plan = self.compile_execution_plan(&route)?;
                    
                    let mut routes = self.routes.write().unwrap();
                    routes.insert(route.id.clone(), route.clone());
                    
                    // Cache the execution plan
                    let mut cache = self.hot_routes_cache.write().unwrap();
                    cache.insert(route.id.clone(), Arc::new(execution_plan));
                }
            }
        }

        // Load functions
        if functions_dir.exists() {
            let entries = fs::read_dir(&functions_dir)
                .map_err(|e| format!("Failed to read functions dir: {}", e))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let content = fs::read_to_string(&path)
                        .map_err(|e| format!("Failed to read function file {}: {}", path.display(), e))?;
                    let func: FunctionDef = serde_json::from_str(&content)
                        .map_err(|e| format!("Failed to parse function from {}: {}", path.display(), e))?;
                    let mut functions = self.global_functions.write().unwrap();
                    functions.insert(func.name.clone(), func);
                }
            }
        }

        Ok(())
    }

    /// Save a function to disk
    fn save_function(&self, func: &FunctionDef) -> Result<(), String> {
        use std::fs;
        use std::path::Path;

        let functions_dir = Path::new(&self.data_dir).join("functions");
        fs::create_dir_all(&functions_dir)
            .map_err(|e| format!("Failed to create functions dir: {}", e))?;

        let file_path = functions_dir.join(format!("{}.json", func.name));
        let content = serde_json::to_string_pretty(func)
            .map_err(|e| format!("Failed to serialize function: {}", e))?;
        fs::write(&file_path, content)
            .map_err(|e| format!("Failed to write function file {}: {}", file_path.display(), e))?;

        Ok(())
    }

    /// Compile logic operations to VM bytecode
    fn compile_logic(&self, logic: &[LogicOperation]) -> Result<(Option<Vec<crate::vm::instructions::OptimizedOperation>>, Option<crate::compiler::symbol_table::SymbolTable>), String> {
        let mut compiler = LogicCompiler::new();
        let bytecode = compiler.compile(logic);
        if bytecode.is_empty() {
            Ok((None, None))
        } else {
            Ok((Some(bytecode), Some(compiler.get_symbol_table().clone())))
        }
    }

    /// Inject request data into VM memory for execution
    fn inject_request_data_into_vm(
        &self,
        vm: &mut VirtualMachine,
        payload: &Option<Value>,
        path_params: &HashMap<String, String>,
        query_params: &HashMap<String, String>,
        symbol_table: &Arc<crate::compiler::symbol_table::SymbolTable>,
    ) -> Result<(), String> {
        // Inject payload fields as top-level variables
        if let Some(Value::Object(obj)) = payload {
            for (key, value) in obj {
                if let Some(index) = symbol_table.get_index(key) {
                    vm.memory.set(index, value.clone());
                }
            }
        }

        // Inject path parameters
        for (key, value_str) in path_params {
            if let Some(index) = symbol_table.get_index(key) {
                // Try to parse as JSON, otherwise keep as string
                let value = serde_json::from_str(value_str)
                    .unwrap_or(Value::String(value_str.clone()));
                vm.memory.set(index, value);
            }
        }

        // Inject query parameters
        for (key, value_str) in query_params {
            if let Some(index) = symbol_table.get_index(key) {
                // Try to parse as JSON, otherwise keep as JSON string
                let value = serde_json::from_str(value_str)
                    .unwrap_or(Value::String(value_str.clone()));
                vm.memory.set(index, value);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn create_test_service() -> DynamicRouteService {
        DynamicRouteService::new()
    }

    fn create_simple_function() -> FunctionDef {
        FunctionDef {
            name: "add_numbers".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            logic: vec![
                LogicOperation::Set {
                    var: "result".to_string(),
                    value: Value::String("${a + b}".to_string()),
                }
            ],
        }
    }

    fn create_nested_function() -> FunctionDef {
        FunctionDef {
            name: "multiply_and_add".to_string(),
            params: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            logic: vec![
                LogicOperation::CallFunction {
                    name: "add_numbers".to_string(),
                    args: vec![Value::String("${x * y}".to_string()), Value::String("${z}".to_string())],
                    output_var: "temp".to_string(),
                },
                LogicOperation::Set {
                    var: "result".to_string(),
                    value: Value::String("${temp * 2}".to_string()),
                }
            ],
        }
    }

    #[test]
    fn test_inline_logic_basic_function_call() {
        let service = create_test_service();
        
        // Define a simple function
        let func = create_simple_function();
        service.global_functions.write().unwrap().insert(func.name.clone(), func);

        // Create logic that calls the function
        let logic = vec![
            LogicOperation::CallFunction {
                name: "add_numbers".to_string(),
                args: vec![Value::Number(5.into()), Value::Number(3.into())],
                output_var: "sum".to_string(),
            }
        ];

        // Inline the logic
        let result = service.inline_logic(&logic, 0).unwrap();

        // Should have parameter bindings, function body, and result mapping
        assert_eq!(result.len(), 4);
        
        // First operation should be setting scoped parameter '_0_a'
        match &result[0] {
            LogicOperation::Set { var, value } => {
                assert_eq!(var, "_0_a");
                assert_eq!(value, &Value::Number(5.into()));
            }
            _ => panic!("Expected Set operation for parameter '_0_a'"),
        }

        // Second operation should be setting scoped parameter '_0_b'
        match &result[1] {
            LogicOperation::Set { var, value } => {
                assert_eq!(var, "_0_b");
                assert_eq!(value, &Value::Number(3.into()));
            }
            _ => panic!("Expected Set operation for parameter 'b'"),
        }

        // Third operation should be the function body (setting scoped result)
        match &result[2] {
            LogicOperation::Set { var, value } => {
                assert_eq!(var, "_0_result");
                assert_eq!(value, &Value::String("${_0_a + _0_b}".to_string()));
            }
            _ => panic!("Expected Set operation for result"),
        }

        // Fourth operation should be mapping scoped result to output_var
        match &result[3] {
            LogicOperation::Set { var, value } => {
                assert_eq!(var, "sum");
                assert_eq!(value, &Value::String("${_0_result}".to_string()));
            }
            _ => panic!("Expected Set operation for output mapping"),
        }
    }

    #[test]
    fn test_inline_logic_preserves_non_call_operations() {
        let service = create_test_service();

        let logic = vec![
            LogicOperation::Set {
                var: "x".to_string(),
                value: Value::Number(10.into()),
            },
            LogicOperation::Return {
                value: Value::String("done".to_string()),
            }
        ];

        let result = service.inline_logic(&logic, 0).unwrap();

        // Should preserve all operations unchanged
        assert_eq!(result.len(), 2);
        assert!(matches!(&result[0], LogicOperation::Set { .. }));
        assert!(matches!(&result[1], LogicOperation::Return { .. }));
    }

    #[test]
    fn test_inline_logic_nested_operations() {
        let service = create_test_service();
        
        // Define a function
        let func = create_simple_function();
        service.global_functions.write().unwrap().insert(func.name.clone(), func);

        // Create logic with function call inside an if statement
        let logic = vec![
            LogicOperation::If {
                condition: "${x > 0}".to_string(),
                then: vec![
                    LogicOperation::CallFunction {
                        name: "add_numbers".to_string(),
                        args: vec![Value::String("${x}".to_string()), Value::Number(1.into())],
                        output_var: "result".to_string(),
                    }
                ],
                otherwise: None,
            }
        ];

        let result = service.inline_logic(&logic, 0).unwrap();

        // Should have one If operation
        assert_eq!(result.len(), 1);
        match &result[0] {
            LogicOperation::If { condition, then, otherwise } => {
                assert_eq!(condition, "${x > 0}");
                assert!(otherwise.is_none());
                // The 'then' branch should contain the inlined function logic
                assert_eq!(then.len(), 4); // 2 parameter sets + 1 result set + 1 output mapping
            }
            _ => panic!("Expected If operation"),
        }
    }

    #[test]
    fn test_inline_logic_recursive_function_calls() {
        let service = create_test_service();
        
        // Define both functions
        let simple_func = create_simple_function();
        let nested_func = create_nested_function();
        
        service.global_functions.write().unwrap().insert(simple_func.name.clone(), simple_func);
        service.global_functions.write().unwrap().insert(nested_func.name.clone(), nested_func);

        // Create logic that calls the nested function
        let logic = vec![
            LogicOperation::CallFunction {
                name: "multiply_and_add".to_string(),
                args: vec![Value::Number(2.into()), Value::Number(3.into()), Value::Number(4.into())],
                output_var: "final_result".to_string(),
            }
        ];

        let result = service.inline_logic(&logic, 0).unwrap();

        // Should have parameter bindings for multiply_and_add, then inlined add_numbers call, then result mapping
        assert!(result.len() > 3); // At least: 3 param sets + 2 param sets for inner call + 1 result set + 1 final result set
        
        // Check that we have multiple Set operations for parameters
        let set_operations: Vec<_> = result.iter().filter(|op| matches!(op, LogicOperation::Set { .. })).collect();
        assert!(set_operations.len() >= 5); // x, y, z for outer function, a, b for inner function
    }

    #[test]
    fn test_inline_logic_depth_limiting() {
        let service = create_test_service();

        // Create a function that calls itself (would cause infinite recursion)
        let recursive_func = FunctionDef {
            name: "recursive".to_string(),
            params: vec!["n".to_string()],
            logic: vec![
                LogicOperation::CallFunction {
                    name: "recursive".to_string(),
                    args: vec![Value::String("${n - 1}".to_string())],
                    output_var: "result".to_string(),
                }
            ],
        };

        service.global_functions.write().unwrap().insert(recursive_func.name.clone(), recursive_func);

        let logic = vec![
            LogicOperation::CallFunction {
                name: "recursive".to_string(),
                args: vec![Value::Number(1.into())],
                output_var: "output".to_string(),
            }
        ];

        // Should fail with depth error
        let result = service.inline_logic(&logic, 49); // Start close to limit
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Maximum recursion depth exceeded"));
    }

    #[test]
    fn test_inline_logic_function_not_found() {
        let service = create_test_service();

        let logic = vec![
            LogicOperation::CallFunction {
                name: "nonexistent_function".to_string(),
                args: vec![Value::Number(1.into())],
                output_var: "result".to_string(),
            }
        ];

        let result = service.inline_logic(&logic, 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found in global registry"));
    }

    #[test]
    fn test_inline_logic_complex_nested_structure() {
        let service = create_test_service();
        
        // Define a function
        let func = create_simple_function();
        service.global_functions.write().unwrap().insert(func.name.clone(), func);

        // Create complex nested logic with function calls in multiple places
        let logic = vec![
            LogicOperation::Set {
                var: "counter".to_string(),
                value: Value::Number(0.into()),
            },
            LogicOperation::While {
                condition: "${counter < 3}".to_string(),
                body: vec![
                    LogicOperation::CallFunction {
                        name: "add_numbers".to_string(),
                        args: vec![Value::String("${counter}".to_string()), Value::Number(1.into())],
                        output_var: "temp".to_string(),
                    },
                    LogicOperation::Set {
                        var: "counter".to_string(),
                        value: Value::String("${temp}".to_string()),
                    }
                ],
                max_iterations: Some(10),
            },
            LogicOperation::Return {
                value: Value::String("${counter}".to_string()),
            }
        ];

        let result = service.inline_logic(&logic, 0).unwrap();

        // Should preserve the overall structure but inline the function call
        assert_eq!(result.len(), 3); // Set + While + Return
        
        // First operation should still be the counter initialization
        match &result[0] {
            LogicOperation::Set { var, .. } => assert_eq!(var, "counter"),
            _ => panic!("Expected Set operation for counter"),
        }

        // Second operation should be the While loop with inlined body
        match &result[1] {
            LogicOperation::While { body, .. } => {
                // The body should contain the inlined function logic
                assert!(body.len() > 2); // More than just the counter increment
            }
            _ => panic!("Expected While operation"),
        }
    }

    #[test]
    fn test_inline_logic_empty_logic() {
        let service = create_test_service();
        
        let logic = vec![];
        let result = service.inline_logic(&logic, 0).unwrap();
        
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_inline_logic_mixed_operations() {
        let service = create_test_service();
        
        // Define a function
        let func = create_simple_function();
        service.global_functions.write().unwrap().insert(func.name.clone(), func);

        let logic = vec![
            LogicOperation::Set {
                var: "start".to_string(),
                value: Value::String("begin".to_string()),
            },
            LogicOperation::CallFunction {
                name: "add_numbers".to_string(),
                args: vec![Value::Number(10.into()), Value::Number(20.into())],
                output_var: "calculation".to_string(),
            },
            LogicOperation::Return {
                value: Value::String("${calculation}".to_string()),
            }
        ];

        let result = service.inline_logic(&logic, 0).unwrap();

        // Should have: Set(start), Set(a), Set(b), Set(result), Set(calculation), Return
        assert_eq!(result.len(), 6);
        
        // Check that the first and last operations are preserved
        match &result[0] {
            LogicOperation::Set { var, value } => {
                assert_eq!(var, "start");
                assert_eq!(value, &Value::String("begin".to_string()));
            }
            _ => panic!("Expected Set operation for start"),
        }

        match &result[5] {
            LogicOperation::Return { value } => {
                assert_eq!(value, &Value::String("${calculation}".to_string()));
            }
            _ => panic!("Expected Return operation"),
        }
    }
}
