use super::utils::{evaluate_condition, resolve_string, resolve_variables};
use super::{date, io, json, math, string};
use futures::future::join_all;
use proto::models::{
    DynamicRouteExecutionContext, ErrorContext, FunctionDefinition, LogicOperation, SandboxPolicy,
};
use serde_json::Value;

fn active_policy(context: &DynamicRouteExecutionContext) -> Option<SandboxPolicy> {
    context.sandbox.clone()
}

fn extract_host(url: &str) -> String {
    let after_scheme = url.split("://").nth(1).unwrap_or(url);
    let host_port = after_scheme.split('/').next().unwrap_or("");
    // strip userinfo and port
    let host = host_port.split('@').next_back().unwrap_or(host_port);
    host.split(':').next().unwrap_or(host).to_string()
}

fn enforce_http(url: &str, context: &DynamicRouteExecutionContext) -> Result<String, String> {
    let resolved = resolve_string(url, context);
    if let Some(policy) = active_policy(context) {
        if !policy.allow_net {
            return Err(
                "sandbox: network access denied for this route (allow_net=false)".to_string(),
            );
        }
        let host = extract_host(&resolved);
        if !policy.is_host_allowed(&host) {
            return Err(format!(
                "sandbox: host '{}' not in allowed_hosts {:?}",
                host, policy.allowed_hosts
            ));
        }
        // Enforce per-request timeout default from policy if op has none — handled by caller
    }
    Ok(resolved)
}

/// Execute advanced logic operations with full feature support
///
/// OPTIMIZATION: Now accepts `&mut context` instead of value to prevent
/// excessive cloning during recursion.
pub async fn execute_logic_extended(
    operations: &[LogicOperation],
    context: &mut DynamicRouteExecutionContext,
    steps: &mut Vec<String>,
) -> Result<Value, String> {
    let mut last_result = Value::Null;

    for operation in operations.iter() {
        // Check for break/continue
        if context.loop_control.should_break {
            // steps.push("Loop BREAK encountered".to_string());
            break;
        }
        if context.loop_control.should_continue {
            // steps.push("Loop CONTINUE encountered".to_string());
            context.loop_control.should_continue = false;
            continue;
        }

        // steps.push(format!("Step {}: Executing operation", _idx + 1));

        match operation {
            // ===== BASIC OPERATIONS =====
            LogicOperation::Return {
                value,
                status,
                headers,
                raw,
            } => {
                // Resolve the return value
                let mut resolved = resolve_variables(value, context);

                // Build enhanced return with metadata if any custom fields are set
                if status.is_some() || headers.is_some() || raw.is_some() {
                    let mut return_obj = serde_json::Map::new();
                    return_obj.insert("value".to_string(), resolved);

                    if let Some(s) = status {
                        return_obj.insert("status".to_string(), Value::Number((*s).into()));
                    }
                    if let Some(h) = headers {
                        return_obj.insert(
                            "headers".to_string(),
                            serde_json::to_value(h).unwrap_or(Value::Null),
                        );
                    }
                    if let Some(r) = raw {
                        return_obj.insert("raw".to_string(), Value::Bool(*r));
                    }

                    resolved = Value::Object(return_obj);
                }

                return Ok(resolved);
            }

            LogicOperation::Comment { .. } => {
                // Comment is a no-op, skip execution
                continue;
            }

            LogicOperation::Set { var, value } => {
                let resolved = resolve_variables(value, context);
                context.variables.insert(var.clone(), resolved.clone());
                // Sandbox memory guard
                if let Some(policy) = active_policy(context) {
                    if context.variables.len() > policy.max_variables {
                        return Err(format!(
                            "sandbox: variable count exceeds max_variables {}",
                            policy.max_variables
                        ));
                    }
                }
                // steps.push(format!("Set variable '{}' = {}", var, resolved));
                last_result = resolved;
            }

            LogicOperation::Get { var } => {
                last_result = context.variables.get(var).cloned().unwrap_or(Value::Null);
                // steps.push(format!("Get variable '{}' = {}", var, last_result));
            }

            // ===== DATABASE OPERATIONS =====
            LogicOperation::QueryDb { query, params } => {
                last_result = io::handle_query_db(query, params, context, steps);
            }

            // ===== HTTP REQUESTS (ASYNC) =====
            LogicOperation::HttpRequest {
                url,
                method,
                body,
                headers,
                timeout_ms,
            } => {
                let _resolved = enforce_http(url, context)?;
                // Apply sandbox default timeout when op doesn't set one
                let effective_timeout = match timeout_ms {
                    Some(t) => Some(*t),
                    None => active_policy(context).map(|p| p.max_timeout_ms),
                };
                last_result = io::handle_http_request(
                    url,
                    method,
                    body,
                    headers,
                    &effective_timeout,
                    context,
                    steps,
                )
                .await?;
            }

            // ===== CONTROL FLOW - IF =====
            LogicOperation::If {
                condition,
                then,
                otherwise,
            } => {
                // steps.push(format!("Evaluate condition: {}", condition));
                let condition_result = evaluate_condition(condition, context);

                if condition_result {
                    // steps.push("Condition is TRUE, executing THEN branch".to_string());
                    // Pass mutable context directly - variable changes persist (imperative style)
                    last_result = Box::pin(execute_logic_extended(then, context, steps)).await?;
                } else if let Some(else_ops) = otherwise {
                    // steps.push("Condition is FALSE, executing ELSE branch".to_string());
                    last_result =
                        Box::pin(execute_logic_extended(else_ops, context, steps)).await?;
                }
            }

            // ===== CONTROL FLOW - SWITCH =====
            LogicOperation::Switch {
                value,
                cases,
                default,
            } => {
                // steps.push(format!("Switch on value: {}", value));
                let switch_value = resolve_string(value, context);

                let mut matched = false;
                for case in cases {
                    if switch_value == case.value.as_str().unwrap_or("") {
                        let switch_val_json =
                            serde_json::from_str::<Value>(&format!("\"{}\"", switch_value))
                                .unwrap_or(Value::String(switch_value.clone()));

                        if switch_val_json == case.value {
                            // steps.push(format!("Matched case: {}", case.value));
                            // Pass mutable context directly
                            last_result =
                                Box::pin(execute_logic_extended(&case.operations, context, steps))
                                    .await?;
                            matched = true;
                            break;
                        }
                    }
                }

                if !matched {
                    if let Some(default_ops) = default {
                        // steps.push("No case matched, executing default".to_string());
                        last_result =
                            Box::pin(execute_logic_extended(default_ops, context, steps)).await?;
                    }
                }
            }

            // ===== CONTROL FLOW - WHILE =====
            LogicOperation::While {
                condition,
                body,
                max_iterations,
            } => {
                // steps.push(format!("While loop: {}", condition));
                let cap = active_policy(context)
                    .map(|p| p.max_loop_iterations)
                    .unwrap_or(1000);
                let max_iter = max_iterations.unwrap_or(cap).min(cap.max(1));
                let mut iterations = 0;

                while evaluate_condition(condition, context) && iterations < max_iter {
                    iterations += 1;
                    // steps.push(format!("While iteration #{}", iterations));

                    last_result = Box::pin(execute_logic_extended(body, context, steps)).await?;

                    if context.loop_control.should_break {
                        context.loop_control.should_break = false;
                        break;
                    }
                    if context.loop_control.should_continue {
                        context.loop_control.should_continue = false;
                        continue;
                    }
                }

                // steps.push(format!("While loop completed after {} iterations", iterations));
            }

            // ===== CONTROL FLOW - FOR LOOP =====
            LogicOperation::Loop {
                collection,
                var,
                body,
            } => {
                // steps.push(format!("Loop over collection '{}' as '{}'", collection, var));

                let var_name = if collection.starts_with("{{") && collection.ends_with("}}") {
                    collection
                        .trim_start_matches("{{")
                        .trim_end_matches("}}")
                        .trim()
                } else {
                    collection
                };

                let items = if let Some(var_value) = context.variables.get(var_name) {
                    var_value.clone()
                } else if collection.starts_with('[') {
                    serde_json::from_str(collection).unwrap_or(Value::Array(vec![]))
                } else {
                    Value::Array(vec![])
                };

                if let Value::Array(arr) = items {
                    // Sandbox DoS guard: cap collection size
                    if arr.len() > 10_000 {
                        return Err("sandbox: loop collection exceeds 10_000 items".to_string());
                    }
                    // Optimization: Pre-allocate space for variables? No, HashMap doesn't work that way easily.
                    // Important: Save previous values of loop variable to restore later (scope isolation)
                    let prev_var = context.variables.get(var).cloned();
                    let prev_index = context.variables.get("index").cloned();

                    for (i, item) in arr.iter().enumerate() {
                        // Set loop variables
                        context.variables.insert(var.clone(), item.clone());
                        context
                            .variables
                            .insert("index".to_string(), Value::Number(i.into()));

                        last_result =
                            Box::pin(execute_logic_extended(body, context, steps)).await?;

                        if context.loop_control.should_break {
                            // steps.push("Loop BREAK".to_string());
                            break;
                        }
                        if context.loop_control.should_continue {
                            // steps.push("Loop CONTINUE".to_string());
                            continue;
                        }
                    }

                    // Cleanup / Restore previous values
                    if let Some(v) = prev_var {
                        context.variables.insert(var.clone(), v);
                    } else {
                        context.variables.remove(var);
                    }
                    if let Some(v) = prev_index {
                        context.variables.insert("index".to_string(), v);
                    } else {
                        context.variables.remove("index");
                    }
                }
            }

            // ===== LOOP CONTROL =====
            LogicOperation::Break => {
                context.loop_control.should_break = true;
                // steps.push("Setting BREAK flag".to_string());
            }

            LogicOperation::Continue => {
                context.loop_control.should_continue = true;
                // steps.push("Setting CONTINUE flag".to_string());
            }

            // ===== ERROR HANDLING - TRY/CATCH =====
            LogicOperation::Try {
                body,
                catch,
                finally,
            } => {
                // steps.push("Try block starting".to_string());

                match Box::pin(execute_logic_extended(body, context, steps)).await {
                    Ok(result) => {
                        last_result = result;
                        // steps.push("Try block succeeded".to_string());
                    }
                    Err(e) => {
                        // steps.push(format!("Try block failed: {}", e));
                        context.error_context = Some(ErrorContext {
                            message: e.clone(),
                            code: None,
                            stack: vec![],
                        });
                        context.variables.insert(
                            "error".to_string(),
                            serde_json::json!({
                                "message": e,
                            }),
                        );

                        last_result =
                            Box::pin(execute_logic_extended(catch, context, steps)).await?;
                    }
                }

                if let Some(finally_ops) = finally {
                    // steps.push("Executing finally block".to_string());
                    let _ = Box::pin(execute_logic_extended(finally_ops, context, steps)).await?;
                }
            }

            LogicOperation::Throw { message, code: _ } => {
                let error_msg = resolve_string(message, context);
                // steps.push(format!("Throwing error: {}", error_msg));
                return Err(error_msg);
            }

            // ===== PARALLEL EXECUTION =====
            LogicOperation::Parallel {
                tasks,
                max_concurrent: _,
            } => {
                // steps.push(format!("Parallel execution of {} tasks", tasks.len()));

                // Note: Parallel execution inherently requires cloning context
                // because each task runs independently.
                let mut futures = vec![];
                for task in tasks.iter() {
                    let mut task_context = context.clone();
                    let task_ops = task.clone();
                    let mut task_steps = vec![]; // Separate steps for each task

                    futures.push(async move {
                        execute_logic_extended(&task_ops, &mut task_context, &mut task_steps).await
                    });
                }

                // TODO: Implement concurrency limit
                let results = join_all(futures).await;
                let mut success_results = vec![];

                for (i, result) in results.into_iter().enumerate() {
                    match result {
                        Ok(v) => success_results.push(v),
                        Err(e) => steps.push(format!("Task {} failed: {}", i, e)),
                    }
                }

                last_result = Value::Array(success_results);
                // steps.push("Parallel execution completed".to_string());
            }

            // ===== FUNCTION DEFINITION =====
            LogicOperation::DefineFunction { name, params, body } => {
                // steps.push(format!("Define function: {} with {} params", name, params.len()));
                context.functions.insert(
                    name.clone(),
                    FunctionDefinition {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
            }

            // ===== FUNCTION CALL =====
            LogicOperation::CallFunction {
                name,
                args,
                output_var,
            } => {
                // steps.push(format!("Call function: {} -> {}", name, output_var));

                // Clone definition to avoid borrow issues with context
                if let Some(func_def) = context.functions.get(name).cloned() {
                    // SCOPING: Function calls must NOT leak variables to parent,
                    // but they DO need access to global/parent variables?
                    // Usually functions are either proper closures or pure.
                    // Here we likely want a new scope that inherits but doesn't modify parent variables
                    // EXCEPT via return value.
                    // However, for performance we want to avoid cloning the whole map.

                    // Strategy:
                    // 1. Create a "Stack Frame" of variables we change.
                    // 2. Or just clone for now as Function Call is a "boundary".
                    // Optimization: We can inline function calls at definition time!
                    // But for runtime calls:
                    let mut func_context = context.clone();
                    // ^ Cloning here is expensive but hard to avoid without complex scope management.
                    // Since "Zero Cost" inlining is preferred, runtime calls are invalidating the perf goal anyway.
                    // But let's support it.

                    // Bind arguments to parameters
                    for (i, param) in func_def.params.iter().enumerate() {
                        if let Some(arg) = args.get(i) {
                            let resolved_arg = resolve_variables(arg, context); // Resolve using CALLER context
                            func_context.variables.insert(param.clone(), resolved_arg);
                        }
                    }

                    let result = Box::pin(execute_logic_extended(
                        &func_def.body,
                        &mut func_context,
                        steps,
                    ))
                    .await?;
                    last_result = result.clone();
                    context.variables.insert(output_var.clone(), result);
                } else {
                    return Err(format!("Function '{}' not defined", name));
                }
            }

            // ===== HELPER OPERATIONS - DELEGATED =====
            LogicOperation::StringOp {
                operation,
                input,
                args,
            } => {
                last_result = string::handle_string_op(operation, input, args, context);
                context
                    .variables
                    .insert("string_result".to_string(), last_result.clone());
                // steps.push(format!("String operation: {} on '{}'", operation, input));
            }

            LogicOperation::MathOp { operation, args } => {
                last_result = math::handle_math_op(operation, args, context);
                context
                    .variables
                    .insert("math_result".to_string(), last_result.clone());
                // steps.push(format!("Math operation: {}", operation));
            }

            LogicOperation::DateOp { operation, args: _ } => {
                last_result = date::handle_date_op(operation);
                context
                    .variables
                    .insert("date_result".to_string(), last_result.clone());
                // steps.push(format!("Date operation: {}", operation));
            }

            LogicOperation::JsonOp {
                operation,
                input,
                args,
            } => {
                last_result = json::handle_json_op(operation, input, args, context);
                // steps.push(format!("JSON operation: {}", operation));
            }

            // ===== LOGGING & SLEEP =====
            LogicOperation::Log { level, message } => {
                io::handle_log(level, message, context, steps);
            }

            LogicOperation::Sleep { duration_ms } => {
                if let Some(policy) = active_policy(context) {
                    if *duration_ms > policy.max_sleep_ms {
                        return Err(format!(
                            "sandbox: sleep {}ms exceeds max_sleep_ms {}ms",
                            duration_ms, policy.max_sleep_ms
                        ));
                    }
                }
                io::handle_sleep(*duration_ms, steps).await;
            }

            // ===== DATA TRANSFORMATIONS (Placeholder) =====
            LogicOperation::Map {
                input: _,
                transform: _,
            } => {
                // steps.push(format!("Map operation on '{}'", input));
                last_result = Value::Array(vec![]);
            }

            LogicOperation::Filter {
                input: _,
                condition: _,
            } => {
                // steps.push(format!("Filter operation on '{}'", input));
                last_result = Value::Array(vec![]);
            }

            LogicOperation::Aggregate {
                input: _,
                operation: _,
            } => {
                // steps.push(format!("Aggregate '{}' using: {}", input, operation));
                last_result = Value::Number(0.into());
            }

            LogicOperation::ExecuteScript { language, code: _ } => {
                if let Some(policy) = active_policy(context) {
                    if !policy.allow_scripts {
                        return Err(format!(
                            "sandbox: execute_script '{}' denied (allow_scripts=false)",
                            language
                        ));
                    }
                }
                // steps.push(format!("Execute {} script", language));
                last_result = serde_json::json!({
                    "result": "Script executed",
                    "language": language,
                });
            }

            LogicOperation::SqlOp {
                query: _,
                args: _,
                output_var,
            } => {
                if let Some(policy) = active_policy(context) {
                    if !policy.allow_sql {
                        return Err(
                            "sandbox: sql_op denied for this route (allow_sql=false)".to_string()
                        );
                    }
                }
                // Note: SqlOp is handled by the VM execution path
                // This fallback is for legacy interpreter path
                context
                    .variables
                    .insert(output_var.clone(), Value::Array(vec![]));
                last_result = Value::Array(vec![]);
            }

            LogicOperation::RedisOp {
                command: _,
                key: _,
                value: _,
                ttl_seconds: _,
                output_var,
            } => {
                if let Some(policy) = active_policy(context) {
                    if !policy.allow_redis {
                        return Err(
                            "sandbox: redis_op denied for this route (allow_redis=false)"
                                .to_string(),
                        );
                    }
                }
                // Note: RedisOp is handled by the VM execution path
                // This fallback is for legacy interpreter path
                if let Some(var) = output_var {
                    context.variables.insert(var.clone(), Value::Null);
                }
                last_result = Value::Null;
            }

            LogicOperation::WsOp {
                command: _,
                message: _,
                channel: _,
            } => {
                // Note: WsOp is handled by the VM execution path with WebSocket manager
                // This fallback is for legacy interpreter path (no-op)
                last_result =
                    Value::String("WebSocket operations require VM execution".to_string());
            }

            LogicOperation::CustomOp(operation_map) => {
                // Custom operations defined via UI extensions
                // Log for debugging but don't fail - allow generic walker to handle variables
                last_result = serde_json::json!({
                    "custom_operation": operation_map.keys().next().unwrap_or(&"unknown".to_string()),
                    "status": "processed_by_generic_walker"
                });
            }

            LogicOperation::AwaitAll { task_ids: _ } => {
                // steps.push("Await all tasks".to_string());
                last_result = Value::Array(vec![]);
            }
        }
    }

    Ok(last_result)
}
