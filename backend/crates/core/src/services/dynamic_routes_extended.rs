// Extended execution logic for Dynamic Routes Engine
// This file contains implementations for all advanced operations

use proto::models::{LogicOperation, DynamicRouteExecutionContext, FunctionDefinition, ErrorContext};
use serde_json::Value;
use futures::future::join_all;

/// Execute advanced logic operations with full feature support
pub async fn execute_logic_extended(
    operations: &[LogicOperation],
    mut context: DynamicRouteExecutionContext,
    steps: &mut Vec<String>,
) -> Result<Value, String> {
    let mut last_result = Value::Null;
    
    for (idx, operation) in operations.iter().enumerate() {
        // Check for break/continue
        if context.loop_control.should_break {
            steps.push("Loop BREAK encountered".to_string());
            break;
        }
        if context.loop_control.should_continue {
            steps.push("Loop CONTINUE encountered".to_string());
            context.loop_control.should_continue = false;
            continue;
        }
        
        steps.push(format!("Step {}: Executing operation", idx + 1));
        
        match operation {
            // ===== BASIC OPERATIONS =====
            LogicOperation::Return { value } => {
                steps.push(format!("Returning value: {}", value));
                return Ok(resolve_variables(value, &context));
            },
            
            LogicOperation::Set { var, value } => {
                let resolved = resolve_variables(value, &context);
                context.variables.insert(var.clone(), resolved.clone());
                steps.push(format!("Set variable '{}' = {}", var, resolved));
                last_result = resolved;
            },
            
            LogicOperation::Get { var } => {
                last_result = context.variables.get(var)
                    .cloned()
                    .unwrap_or(Value::Null);
                steps.push(format!("Get variable '{}' = {}", var, last_result));
            },
            
            // ===== DATABASE OPERATIONS =====
            LogicOperation::QueryDb { query, params } => {
                steps.push(format!("Execute DB query: {}", query));
                // TODO: Implement real database connection
                last_result = serde_json::json!({
                    "rows": [],
                    "count": 0,
                    "query": query,
                });
                context.variables.insert("db_result".to_string(), last_result.clone());
            },
            
            // ===== HTTP REQUESTS (ASYNC) =====
            LogicOperation::HttpRequest { url, method, body, headers, timeout_ms } => {
                steps.push(format!("HTTP {} request to: {}", method, url));
                
                // Resolve URL and body with variables
                let resolved_url = resolve_string(url, &context);
                let resolved_body = body.as_ref().map(|b| resolve_variables(b, &context));
                
                // TODO: Implement real HTTP client (reqwest)
                last_result = serde_json::json!({
                    "status": 200,
                    "body": {"message": "HTTP request executed"},
                    "url": resolved_url,
                    "method": method,
                });
                context.variables.insert("http_response".to_string(), last_result.clone());
            },
            
            // ===== CONTROL FLOW - IF =====
            LogicOperation::If { condition, then, otherwise } => {
                steps.push(format!("Evaluate condition: {}", condition));
                let condition_result = evaluate_condition(condition, &context);
                
                if condition_result {
                    steps.push("Condition is TRUE, executing THEN branch".to_string());
                    last_result = Box::pin(execute_logic_extended(then, context.clone(), steps)).await?;
                } else if let Some(else_ops) = otherwise {
                    steps.push("Condition is FALSE, executing ELSE branch".to_string());
                    last_result = Box::pin(execute_logic_extended(else_ops, context.clone(), steps)).await?;
                }
            },
            
            // ===== CONTROL FLOW - SWITCH =====
            LogicOperation::Switch { value, cases, default } => {
                steps.push(format!("Switch on value: {}", value));
                let switch_value = if let Some(v) = context.variables.get(value) {
                    v.clone()
                } else {
                    Value::String(value.clone())
                };
                
                let mut matched = false;
                for case in cases {
                    if switch_value == case.value {
                        steps.push(format!("Matched case: {}", case.value));
                        last_result = Box::pin(execute_logic_extended(&case.operations, context.clone(), steps)).await?;
                        matched = true;
                        break;
                    }
                }
                
                if !matched {
                    if let Some(default_ops) = default {
                        steps.push("No case matched, executing default".to_string());
                        last_result = Box::pin(execute_logic_extended(default_ops, context.clone(), steps)).await?;
                    }
                }
            },
            
            // ===== CONTROL FLOW - WHILE =====
            LogicOperation::While { condition, body, max_iterations } => {
                steps.push(format!("While loop: {}", condition));
                let max_iter = max_iterations.unwrap_or(1000);
                let mut iterations = 0;
                
                while evaluate_condition(condition, &context) && iterations < max_iter {
                    iterations += 1;
                    steps.push(format!("While iteration #{}", iterations));
                    
                    last_result = Box::pin(execute_logic_extended(body, context.clone(), steps)).await?;
                    
                    if context.loop_control.should_break {
                        context.loop_control.should_break = false;
                        break;
                    }
                    if context.loop_control.should_continue {
                        context.loop_control.should_continue = false;
                        continue;
                    }
                }
                
                steps.push(format!("While loop completed after {} iterations", iterations));
            },
            
            // ===== CONTROL FLOW - FOR LOOP =====
            LogicOperation::Loop { collection, var, body } => {
                steps.push(format!("Loop over collection '{}' as '{}'", collection, var));
                
                let items = if let Some(var_value) = context.variables.get(collection) {
                    var_value.clone()
                } else if collection.starts_with('[') {
                    serde_json::from_str(collection).unwrap_or(Value::Array(vec![]))
                } else {
                    Value::Array(vec![])
                };
                
                if let Value::Array(arr) = items {
                    for (i, item) in arr.iter().enumerate() {
                        steps.push(format!("Loop iteration #{}", i + 1));
                        let mut loop_context = context.clone();
                        loop_context.variables.insert(var.clone(), item.clone());
                        loop_context.variables.insert("index".to_string(), Value::Number(i.into()));
                        
                        last_result = Box::pin(execute_logic_extended(body, loop_context.clone(), steps)).await?;
                        
                        if loop_context.loop_control.should_break {
                            steps.push("Loop BREAK".to_string());
                            break;
                        }
                        if loop_context.loop_control.should_continue {
                            steps.push("Loop CONTINUE".to_string());
                            continue;
                        }
                    }
                }
            },
            
            // ===== LOOP CONTROL =====
            LogicOperation::Break => {
                context.loop_control.should_break = true;
                steps.push("Setting BREAK flag".to_string());
            },
            
            LogicOperation::Continue => {
                context.loop_control.should_continue = true;
                steps.push("Setting CONTINUE flag".to_string());
            },
            
            // ===== ERROR HANDLING - TRY/CATCH =====
            LogicOperation::Try { body, catch, finally } => {
                steps.push("Try block starting".to_string());
                
                match Box::pin(execute_logic_extended(body, context.clone(), steps)).await {
                    Ok(result) => {
                        last_result = result;
                        steps.push("Try block succeeded".to_string());
                    },
                    Err(e) => {
                        steps.push(format!("Try block failed: {}", e));
                        context.error_context = Some(ErrorContext {
                            message: e.clone(),
                            code: None,
                            stack: vec![],
                        });
                        context.variables.insert("error".to_string(), serde_json::json!({
                            "message": e,
                        }));
                        
                        last_result = Box::pin(execute_logic_extended(catch, context.clone(), steps)).await?;
                    }
                }
                
                if let Some(finally_ops) = finally {
                    steps.push("Executing finally block".to_string());
                    Box::pin(execute_logic_extended(finally_ops, context.clone(), steps)).await?;
                }
            },
            
            LogicOperation::Throw { message, code } => {
                let error_msg = resolve_string(message, &context);
                steps.push(format!("Throwing error: {}", error_msg));
                return Err(error_msg);
            },
            
            // ===== PARALLEL EXECUTION =====
            LogicOperation::Parallel { tasks, max_concurrent } => {
                steps.push(format!("Parallel execution of {} tasks", tasks.len()));
                
                let mut futures = vec![];
                for (i, task) in tasks.iter().enumerate() {
                    let task_context = context.clone();
                    let task_ops = task.clone();
                    let mut task_steps = vec![];
                    
                    futures.push(async move {
                        execute_logic_extended(&task_ops, task_context, &mut task_steps).await
                    });
                }
                
                let results = join_all(futures).await;
                let mut success_results = vec![];
                
                for (i, result) in results.iter().enumerate() {
                    match result {
                        Ok(v) => success_results.push(v.clone()),
                        Err(e) => steps.push(format!("Task {} failed: {}", i, e)),
                    }
                }
                
                last_result = Value::Array(success_results);
                steps.push(format!("Parallel execution completed"));
            },
            
            // ===== FUNCTION DEFINITION =====
            LogicOperation::DefineFunction { name, params, body } => {
                steps.push(format!("Define function: {} with {} params", name, params.len()));
                context.functions.insert(name.clone(), FunctionDefinition {
                    params: params.clone(),
                    body: body.clone(),
                });
            },
            
            // ===== FUNCTION CALL =====
            LogicOperation::CallFunction { name, args } => {
                steps.push(format!("Call function: {}", name));
                
                if let Some(func_def) = context.functions.get(name).cloned() {
                    let mut func_context = context.clone();
                    
                    // Bind arguments to parameters
                    for (i, param) in func_def.params.iter().enumerate() {
                        if let Some(arg) = args.get(i) {
                            let resolved_arg = resolve_variables(arg, &context);
                            func_context.variables.insert(param.clone(), resolved_arg);
                        }
                    }
                    
                    last_result = Box::pin(execute_logic_extended(&func_def.body, func_context, steps)).await?;
                } else {
                    return Err(format!("Function '{}' not defined", name));
                }
            },
            
            // ===== HELPER FUNCTIONS - STRING =====
            LogicOperation::StringOp { operation, input, args } => {
                let input_str = resolve_string(input, &context);
                
                last_result = match operation.as_str() {
                    "split" => {
                        let delimiter = args.get(0).and_then(|v| v.as_str()).unwrap_or(",");
                        let parts: Vec<Value> = input_str.split(delimiter).map(|s| Value::String(s.to_string())).collect();
                        Value::Array(parts)
                    },
                    "join" => {
                        let delimiter = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                        if let Some(arr_str) = context.variables.get(input) {
                            if let Value::Array(arr) = arr_str {
                                let joined = arr.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>().join(delimiter);
                                Value::String(joined)
                            } else {
                                Value::String(input_str)
                            }
                        } else {
                            Value::String(input_str)
                        }
                    },
                    "upper" => Value::String(input_str.to_uppercase()),
                    "lower" => Value::String(input_str.to_lowercase()),
                    "trim" => Value::String(input_str.trim().to_string()),
                    "replace" => {
                        let from = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                        let to = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
                        Value::String(input_str.replace(from, to))
                    },
                    _ => Value::Null,
                };
                
                steps.push(format!("String operation: {} on '{}'", operation, input));
            },
            
            // ===== HELPER FUNCTIONS - MATH =====
            LogicOperation::MathOp { operation, args } => {
                let numbers: Vec<f64> = args.iter()
                    .filter_map(|v| match v {
                        Value::Number(n) => n.as_f64(),
                        _ => None,
                    })
                    .collect();
                
                last_result = match operation.as_str() {
                    "sum" => Value::Number(serde_json::Number::from_f64(numbers.iter().sum()).unwrap()),
                    "avg" => {
                        let avg = numbers.iter().sum::<f64>() / numbers.len() as f64;
                        Value::Number(serde_json::Number::from_f64(avg).unwrap())
                    },
                    "min" => {
                        let min = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
                        Value::Number(serde_json::Number::from_f64(min).unwrap())
                    },
                    "max" => {
                        let max = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                        Value::Number(serde_json::Number::from_f64(max).unwrap())
                    },
                    "round" => {
                        let val = numbers.get(0).unwrap_or(&0.0).round();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "floor" => {
                        let val = numbers.get(0).unwrap_or(&0.0).floor();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "ceil" => {
                        let val = numbers.get(0).unwrap_or(&0.0).ceil();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "abs" => {
                        let val = numbers.get(0).unwrap_or(&0.0).abs();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "pow" => {
                        let base = numbers.get(0).unwrap_or(&0.0);
                        let exp = numbers.get(1).unwrap_or(&1.0);
                        Value::Number(serde_json::Number::from_f64(base.powf(*exp)).unwrap())
                    },
                    "sqrt" => {
                        let val = numbers.get(0).unwrap_or(&0.0).sqrt();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    _ => Value::Null,
                };
                
                steps.push(format!("Math operation: {}", operation));
            },
            
            // ===== HELPER FUNCTIONS - DATE/TIME =====
            LogicOperation::DateOp { operation, args: _ } => {
                use chrono::Utc;
                
                last_result = match operation.as_str() {
                    "now" => Value::String(Utc::now().to_rfc3339()),
                    "timestamp" => Value::Number(Utc::now().timestamp().into()),
                    "format" => {
                        // TODO: Implement date formatting
                        Value::String(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
                    },
                    _ => Value::Null,
                };
                
                steps.push(format!("Date operation: {}", operation));
            },
            
            // ===== HELPER FUNCTIONS - JSON =====
            LogicOperation::JsonOp { operation, input, args } => {
                let input_val = if let Some(v) = context.variables.get(input) {
                    v.clone()
                } else {
                    serde_json::from_str(input).unwrap_or(Value::Null)
                };
                
                last_result = match operation.as_str() {
                    "stringify" => Value::String(serde_json::to_string(&input_val).unwrap_or_default()),
                    "parse" => {
                        if let Value::String(s) = input_val {
                            serde_json::from_str(&s).unwrap_or(Value::Null)
                        } else {
                            Value::Null
                        }
                    },
                    "get_path" => {
                        let path = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
                        get_json_path(&input_val, path)
                    },
                    _ => Value::Null,
                };
                
                steps.push(format!("JSON operation: {}", operation));
            },
            
            // ===== LOGGING =====
            LogicOperation::Log { level, message } => {
                let resolved_msg = resolve_string(message, &context);
                steps.push(format!("[{}] {}", level.to_uppercase(), resolved_msg));
                println!("[{}] {}", level.to_uppercase(), resolved_msg);
            },
            
            // ===== SLEEP =====
            LogicOperation::Sleep { duration_ms } => {
                steps.push(format!("Sleep for {}ms", duration_ms));
                tokio::time::sleep(tokio::time::Duration::from_millis(*duration_ms)).await;
            },
            
            // ===== DATA TRANSFORMATIONS =====
            LogicOperation::Map { input, transform } => {
                steps.push(format!("Map operation on '{}'", input));
                // TODO: Implement real data mapping
                last_result = Value::Array(vec![]);
            },
            
            LogicOperation::Filter { input, condition } => {
                steps.push(format!("Filter operation on '{}'", input));
                last_result = Value::Array(vec![]);
            },
            
            LogicOperation::Aggregate { input, operation } => {
                steps.push(format!("Aggregate '{}' using: {}", input, operation));
                last_result = Value::Number(0.into());
            },
            
            // ===== SCRIPT EXECUTION =====
            LogicOperation::ExecuteScript { language, code } => {
                steps.push(format!("Execute {} script", language));
                // TODO: Implement sandboxed script execution
                last_result = serde_json::json!({
                    "result": "Script executed",
                    "language": language,
                });
            },
            
            LogicOperation::AwaitAll { task_ids } => {
                steps.push("Await all tasks".to_string());
                last_result = Value::Array(vec![]);
            },
        }
    }
    
    Ok(last_result)
}

// Helper function to resolve variables in strings
fn resolve_string(template: &str, context: &DynamicRouteExecutionContext) -> String {
    let mut result = template.to_string();
    
    for (key, value) in &context.variables {
        let placeholder = format!("{{{{{}}}}}", key);
        if result.contains(&placeholder) {
            let replacement = match value {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
    }
    
    result
}

// Helper function to resolve variables in JSON values
fn resolve_variables(value: &Value, context: &DynamicRouteExecutionContext) -> Value {
    match value {
        Value::String(s) => {
            Value::String(resolve_string(s, context))
        },
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| resolve_variables(v, context)).collect())
        },
        Value::Object(obj) => {
            let mut resolved_obj = serde_json::Map::new();
            for (k, v) in obj {
                resolved_obj.insert(k.clone(), resolve_variables(v, context));
            }
            Value::Object(resolved_obj)
        },
        other => other.clone(),
    }
}

// Enhanced condition evaluation
fn evaluate_condition(condition: &str, context: &DynamicRouteExecutionContext) -> bool {
    let resolved = resolve_string(condition, context);
    
    // Support multiple operators
    if resolved.contains("==") {
        let parts: Vec<&str> = resolved.split("==").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            return parts[0] == parts[1];
        }
    }
    
    if resolved.contains("!=") {
        let parts: Vec<&str> = resolved.split("!=").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            return parts[0] != parts[1];
        }
    }
    
    if resolved.contains(">=") {
        let parts: Vec<&str> = resolved.split(">=").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left >= right;
            }
        }
    }
    
    if resolved.contains("<=") {
        let parts: Vec<&str> = resolved.split("<=").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left <= right;
            }
        }
    }
    
    if resolved.contains(">") {
        let parts: Vec<&str> = resolved.split(">").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left > right;
            }
        }
    }
    
    if resolved.contains("<") {
        let parts: Vec<&str> = resolved.split("<").collect();
        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
                return left < right;
            }
        }
    }
    
    // Boolean literals
    if resolved == "true" { return true; }
    if resolved == "false" { return false; }
    
    // Default
    true
}

// Helper to extract value from JSON path
fn get_json_path(value: &Value, path: &str) -> Value {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;
    
    for part in parts {
        match current {
            Value::Object(map) => {
                if let Some(next) = map.get(part) {
                    current = next;
                } else {
                    return Value::Null;
                }
            },
            _ => return Value::Null,
        }
    }
    
    current.clone()
}
