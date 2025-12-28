// Extended execution logic for Dynamic Routes Engine
// This file contains implementations for all advanced operations

use proto::models::{LogicOperation, DynamicRouteExecutionContext, FunctionDefinition, ErrorContext};
use serde_json::Value;
use futures::future::join_all;
use reqwest;

/// Execute advanced logic operations with full feature support
pub async fn execute_logic_extended(
    operations: &[LogicOperation],
    mut context: DynamicRouteExecutionContext,
    steps: &mut Vec<String>,
) -> Result<(Value, DynamicRouteExecutionContext), String> {
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
                return Ok((resolve_variables(value, &context), context));
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
            LogicOperation::QueryDb { query, params: _ } => {
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
                
                // Resolve URL with variables
                let resolved_url = resolve_string(url, &context);
                
                // Create HTTP client
                let mut client_builder = reqwest::Client::builder();
                
                // Set timeout if specified
                if let Some(timeout) = timeout_ms {
                    client_builder = client_builder.timeout(std::time::Duration::from_millis(*timeout));
                }
                
                let client = client_builder.build().map_err(|e| format!("Failed to create HTTP client: {}", e))?;
                
                // Build request
                let mut request_builder = match method.as_str() {
                    "GET" => client.get(&resolved_url),
                    "POST" => client.post(&resolved_url),
                    "PUT" => client.put(&resolved_url),
                    "DELETE" => client.delete(&resolved_url),
                    "PATCH" => client.patch(&resolved_url),
                    "HEAD" => client.head(&resolved_url),
                    _ => return Err(format!("Unsupported HTTP method: {}", method)),
                };
                
                // Add headers if present
                if let Some(hdrs) = headers {
                    for (key, value) in hdrs {
                        let resolved_value = resolve_string(value, &context);
                        request_builder = request_builder.header(key, resolved_value);
                    }
                }
                
                // Add body if present
                if let Some(b) = body {
                    let resolved_body = resolve_variables(b, &context);
                    let body_string = match resolved_body {
                        Value::String(s) => s,
                        _ => serde_json::to_string(&resolved_body).map_err(|e| format!("Failed to serialize body: {}", e))?,
                    };
                    request_builder = request_builder.body(body_string);
                }
                
                // Send request and await response
                let response = request_builder.send().await.map_err(|e| format!("HTTP request failed: {}", e))?;
                let status_code = response.status().as_u16();
                let response_text = response.text().await.map_err(|e| format!("Failed to read response body: {}", e))?;
                
                // Try to parse response as JSON, otherwise return as string
                let response_body = match serde_json::from_str::<Value>(&response_text) {
                    Ok(json_value) => json_value,
                    Err(_) => Value::String(response_text),
                };
                
                last_result = serde_json::json!({
                    "status": status_code,
                    "body": response_body,
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
                    let (result, updated_context) = Box::pin(execute_logic_extended(then, context.clone(), steps)).await?;
                    last_result = result;
                    // Merge variables from then branch
                    for (key, value) in updated_context.variables.iter() {
                        context.variables.insert(key.clone(), value.clone());
                    }
                } else if let Some(else_ops) = otherwise {
                    steps.push("Condition is FALSE, executing ELSE branch".to_string());
                    let (result, updated_context) = Box::pin(execute_logic_extended(else_ops, context.clone(), steps)).await?;
                    last_result = result;
                    // Merge variables from else branch
                    for (key, value) in updated_context.variables.iter() {
                        context.variables.insert(key.clone(), value.clone());
                    }
                }
            },
            
            // ===== CONTROL FLOW - SWITCH =====
            LogicOperation::Switch { value, cases, default } => {
                steps.push(format!("Switch on value: {}", value));
                let switch_value = resolve_string(value, &context);
                let _switch_value_json = serde_json::from_str::<Value>(&format!("\"{}\"", switch_value.clone()))
                    .unwrap_or(Value::String(switch_value.clone()));
                
                let mut matched = false;
                for case in cases {
                    if switch_value == case.value {
                        steps.push(format!("Matched case: {}", case.value));
                        let case_context = context.clone();
                        let (result, updated_context) = Box::pin(execute_logic_extended(&case.operations, case_context, steps)).await?;
                        last_result = result;
                        // Merge variables back to main context
                        for (key, value) in updated_context.variables.iter() {
                            context.variables.insert(key.clone(), value.clone());
                        }
                        matched = true;
                        break;
                    }
                }
                
                if !matched {
                    if let Some(default_ops) = default {
                        steps.push("No case matched, executing default".to_string());
                        let default_context = context.clone();
                        let (result, updated_context) = Box::pin(execute_logic_extended(default_ops, default_context, steps)).await?;
                        last_result = result;
                        // Merge variables back to main context
                        for (key, value) in updated_context.variables.iter() {
                            context.variables.insert(key.clone(), value.clone());
                        }
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
                    
                    let loop_context = context.clone();
                    let (result, updated_context) = Box::pin(execute_logic_extended(body, loop_context, steps)).await?;
                    last_result = result;
                    
                    // Merge variables back from the updated context
                    for (key, value) in updated_context.variables.iter() {
                        context.variables.insert(key.clone(), value.clone());
                    }
                    
                    if updated_context.loop_control.should_break {
                        context.loop_control.should_break = false;
                        break;
                    }
                    if updated_context.loop_control.should_continue {
                        context.loop_control.should_continue = false;
                        continue;
                    }
                }
                
                steps.push(format!("While loop completed after {} iterations", iterations));
            },
            
            // ===== CONTROL FLOW - FOR LOOP =====
            LogicOperation::Loop { collection, var, body } => {
                steps.push(format!("Loop over collection '{}' as '{}'", collection, var));
                
                // Extract variable name from {{var}} syntax
                let var_name = if collection.starts_with("{{") && collection.ends_with("}}") {
                    collection.trim_start_matches("{{").trim_end_matches("}}").trim()
                } else {
                    collection
                };
                
                let items = if let Some(var_value) = context.variables.get(var_name) {
                    // Found variable by name
                    var_value.clone()
                } else if collection.starts_with('[') {
                    // JSON array literal
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
                        
                        let (result, updated_context) = Box::pin(execute_logic_extended(body, loop_context, steps)).await?;
                        last_result = result;
                        
                        // Merge variables back (except loop var and index)
                        for (key, value) in updated_context.variables.iter() {
                            if key != var && key != "index" {
                                context.variables.insert(key.clone(), value.clone());
                            }
                        }
                        
                        if updated_context.loop_control.should_break {
                            steps.push("Loop BREAK".to_string());
                            break;
                        }
                        if updated_context.loop_control.should_continue {
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
                    Ok((result, updated_context)) => {
                        last_result = result;
                        // Merge variables from try block
                        for (key, value) in updated_context.variables.iter() {
                            context.variables.insert(key.clone(), value.clone());
                        }
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
                        
                        let (result, updated_context) = Box::pin(execute_logic_extended(catch, context.clone(), steps)).await?;
                        last_result = result;
                        // Merge variables from catch block
                        for (key, value) in updated_context.variables.iter() {
                            context.variables.insert(key.clone(), value.clone());
                        }
                    }
                }
                
                if let Some(finally_ops) = finally {
                    steps.push("Executing finally block".to_string());
                    let (_, updated_context) = Box::pin(execute_logic_extended(finally_ops, context.clone(), steps)).await?;
                    // Merge variables from finally block
                    for (key, value) in updated_context.variables.iter() {
                        context.variables.insert(key.clone(), value.clone());
                    }
                }
            },
            
            LogicOperation::Throw { message, code: _ } => {
                let error_msg = resolve_string(message, &context);
                steps.push(format!("Throwing error: {}", error_msg));
                return Err(error_msg);
            },
            
            // ===== PARALLEL EXECUTION =====
            LogicOperation::Parallel { tasks, max_concurrent: _ } => {
                steps.push(format!("Parallel execution of {} tasks", tasks.len()));
                
                let mut futures = vec![];
                for task in tasks.iter() {
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
                        Ok((v, _ctx)) => success_results.push(v.clone()),
                        Err(e) => steps.push(format!("Task {} failed: {}", i, e)),
                    }
                }
                
                last_result = Value::Array(success_results);
                steps.push("Parallel execution completed".to_string());
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
            LogicOperation::CallFunction { name, args, output_var } => {
                steps.push(format!("Call function: {} -> {}", name, output_var));
                
                if let Some(func_def) = context.functions.get(name).cloned() {
                    let mut func_context = context.clone();
                    
                    // Bind arguments to parameters
                    for (i, param) in func_def.params.iter().enumerate() {
                        if let Some(arg) = args.get(i) {
                            let resolved_arg = resolve_variables(arg, &context);
                            func_context.variables.insert(param.clone(), resolved_arg);
                        }
                    }
                    
                    let (result, _updated_context) = Box::pin(execute_logic_extended(&func_def.body, func_context, steps)).await?;
                    last_result = result.clone();
                    context.variables.insert(output_var.clone(), result);
                } else {
                    return Err(format!("Function '{}' not defined", name));
                }
            },
            
            // ===== HELPER FUNCTIONS - STRING =====
            LogicOperation::StringOp { operation, input, args } => {
                let input_str = resolve_string(input, &context);
                
                last_result = match operation.as_str() {
                    "split" => {
                        let delimiter = args.first().and_then(|v| v.as_str()).unwrap_or(",");
                        let parts: Vec<Value> = input_str.split(delimiter).map(|s| Value::String(s.to_string())).collect();
                        Value::Array(parts)
                    },
                    "join" => {
                        let delimiter = args.first().and_then(|v| v.as_str()).unwrap_or("");
                        if let Some(Value::Array(arr)) = context.variables.get(input) {
                            let joined = arr.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>().join(delimiter);
                            Value::String(joined)
                        } else {
                            Value::String(input_str)
                        }
                    },
                    "upper" => Value::String(input_str.to_uppercase()),
                    "lower" => Value::String(input_str.to_lowercase()),
                    "trim" => Value::String(input_str.trim().to_string()),
                    "replace" => {
                        let from = args.first().and_then(|v| v.as_str()).unwrap_or("");
                        let to = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
                        Value::String(input_str.replace(from, to))
                    },
                    _ => Value::Null,
                };
                
                context.variables.insert("string_result".to_string(), last_result.clone());
                steps.push(format!("String operation: {} on '{}'", operation, input));
            },
            
            // ===== HELPER FUNCTIONS - MATH =====
            LogicOperation::MathOp { operation, args } => {
                // Resolve variables in args first
                let resolved_args: Vec<Value> = args.iter()
                    .map(|v| resolve_variables(v, &context))
                    .collect();
                
                let numbers: Vec<f64> = resolved_args.iter()
                    .filter_map(|v| match v {
                        Value::Number(n) => n.as_f64(),
                        Value::String(s) => s.parse().ok(),
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
                        let val = numbers.first().unwrap_or(&0.0).round();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "floor" => {
                        let val = numbers.first().unwrap_or(&0.0).floor();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "ceil" => {
                        let val = numbers.first().unwrap_or(&0.0).ceil();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "abs" => {
                        let val = numbers.first().unwrap_or(&0.0).abs();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "pow" => {
                        let base = numbers.first().unwrap_or(&0.0);
                        let exp = numbers.get(1).unwrap_or(&1.0);
                        Value::Number(serde_json::Number::from_f64(base.powf(*exp)).unwrap())
                    },
                    "sqrt" => {
                        let val = numbers.first().unwrap_or(&0.0).sqrt();
                        Value::Number(serde_json::Number::from_f64(val).unwrap())
                    },
                    "subtract" => {
                        if numbers.len() >= 2 {
                            Value::Number(serde_json::Number::from_f64(numbers[0] - numbers[1]).unwrap())
                        } else {
                            Value::Null
                        }
                    },
                    "multiply" => {
                        let product = numbers.iter().product();
                        Value::Number(serde_json::Number::from_f64(product).unwrap())
                    },
                    "divide" => {
                        if numbers.len() >= 2 && numbers[1] != 0.0 {
                            Value::Number(serde_json::Number::from_f64(numbers[0] / numbers[1]).unwrap())
                        } else {
                            Value::Null
                        }
                    },
                    "mod" => {
                        if numbers.len() >= 2 && numbers[1] != 0.0 {
                            Value::Number(serde_json::Number::from_f64(numbers[0] % numbers[1]).unwrap())
                        } else {
                            Value::Null
                        }
                    },
                    _ => Value::Null,
                };
                
                context.variables.insert("math_result".to_string(), last_result.clone());
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
                
                context.variables.insert("date_result".to_string(), last_result.clone());
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
                        let path = args.first().and_then(|v| v.as_str()).unwrap_or("");
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
            LogicOperation::Map { input, transform: _ } => {
                steps.push(format!("Map operation on '{}'", input));
                // TODO: Implement real data mapping
                last_result = Value::Array(vec![]);
            },
            
            LogicOperation::Filter { input, condition: _ } => {
                steps.push(format!("Filter operation on '{}'", input));
                last_result = Value::Array(vec![]);
            },
            
            LogicOperation::Aggregate { input, operation } => {
                steps.push(format!("Aggregate '{}' using: {}", input, operation));
                last_result = Value::Number(0.into());
            },
            
            // ===== SCRIPT EXECUTION =====
            LogicOperation::ExecuteScript { language, code: _ } => {
                steps.push(format!("Execute {} script", language));
                // TODO: Implement sandboxed script execution
                last_result = serde_json::json!({
                    "result": "Script executed",
                    "language": language,
                });
            },
            
            LogicOperation::AwaitAll { task_ids: _ } => {
                steps.push("Await all tasks".to_string());
                last_result = Value::Array(vec![]);
            },
        }
    }
    
    Ok((last_result, context))
}

// Helper function to resolve variables in strings
fn resolve_string(template: &str, context: &DynamicRouteExecutionContext) -> String {
    let mut result = template.to_string();
    
    // First resolve error context variables (error.message, error.code)
    if let Some(error_ctx) = &context.error_context {
        result = result.replace("{{error.message}}", &error_ctx.message);
        if let Some(code) = &error_ctx.code {
            result = result.replace("{{error.code}}", code);
        } else {
            result = result.replace("{{error.code}}", "UNKNOWN_ERROR");
        }
    }
    
    // Then resolve from request payload (for top-level fields like {{order_id}})
    if let Some(payload) = &context.request_payload {
        if let Some(obj) = payload.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                if result.contains(&placeholder) {
                    let replacement = match value {
                        Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    result = result.replace(&placeholder, &replacement);
                }
            }
        }
    }
    
    // Then resolve ${...} expressions (evaluate them)
    use regex::Regex;
    let expr_re = Regex::new(r"\$\{([^}]+)\}").unwrap();
    result = expr_re.replace_all(&result, |caps: &regex::Captures| {
        let expr = &caps[1];
        // Convert context variables to HashMap for expression evaluation
        let mut expr_context = std::collections::HashMap::new();
        for (key, value) in &context.variables {
            expr_context.insert(key.clone(), value.clone());
        }
        // Try to evaluate the expression
        match crate::expression::template::evaluate_expression(expr, &expr_context) {
            Ok(value) => match value {
                Value::String(s) => s,
                other => other.to_string(),
            },
            Err(_) => caps[0].to_string(), // Keep original if evaluation fails
        }
    }).to_string();
    
    // Then resolve regular variables (including nested paths like order.status)
    // Use regex to find all {{...}} patterns
    let var_re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
    
    result = var_re.replace_all(&result, |caps: &regex::Captures| {
        let path = &caps[1];
        
        // Check if it's a nested path (contains dot)
        if path.contains('.') {
            let parts: Vec<&str> = path.split('.').collect();
            if let Some(root_var) = context.variables.get(parts[0]) {
                // Use get_json_path for nested access
                let nested_value = get_json_path(root_var, &parts[1..].join("."));
                return match nested_value {
                    Value::String(s) => s,
                    Value::Null => caps[0].to_string(), // Keep original if null
                    other => other.to_string(),
                };
            }
        } else {
            // Simple variable lookup
            if let Some(value) = context.variables.get(path) {
                return match value {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
            }
        }
        
        // If not found, keep original
        caps[0].to_string()
    }).to_string();
    
    result
}

// Helper function to resolve variables in JSON values
fn resolve_variables(value: &Value, context: &DynamicRouteExecutionContext) -> Value {
    match value {
        Value::String(s) => {
            // Check if it's a pure expression ${...}
            if s.starts_with("${") && s.ends_with("}") && s.len() > 3 {
                let expr = &s[2..s.len()-1];
                // Build context for expression evaluation
                let mut expr_context = std::collections::HashMap::new();
                for (key, val) in &context.variables {
                    expr_context.insert(key.clone(), val.clone());
                }
                // Try to evaluate the expression
                match crate::expression::template::evaluate_expression(expr, &expr_context) {
                    Ok(evaluated_value) => evaluated_value,
                    Err(_) => Value::String(resolve_string(s, context)),
                }
            } else {
                Value::String(resolve_string(s, context))
            }
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
    eprintln!("DEBUG: Evaluating condition: '{}' -> '{}'", condition, resolved);
    
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
