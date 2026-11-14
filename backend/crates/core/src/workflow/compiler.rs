use proto::models::{LogicOperation, WorkflowOpV1, WorkflowSpecV1, WorkflowStepV1};

pub struct WorkflowCompiler;

impl WorkflowCompiler {
    /// Compile a valid WorkflowSpecV1 into executable LogicOperation sequence.
    pub fn compile(spec: &WorkflowSpecV1) -> Vec<LogicOperation> {
        let mut ops = Vec::new();

        // 1. Compile each step
        for step in &spec.steps {
            ops.extend(Self::compile_step(step));
        }

        // 2. Append final Return operation
        ops.push(LogicOperation::Return {
            value: spec.r#return.value.clone(),
            status: Some(spec.r#return.status),
            headers: spec.r#return.headers.clone(),
            raw: None,
        });

        ops
    }

    fn compile_step(step: &WorkflowStepV1) -> Vec<LogicOperation> {
        let mut step_ops = Vec::new();

        match &step.op {
            WorkflowOpV1::Sql {
                query,
                args,
                output_var,
            } => {
                step_ops.push(LogicOperation::SqlOp {
                    query: query.clone(),
                    args: args.clone(),
                    output_var: output_var.clone(),
                });
            }
            WorkflowOpV1::Http {
                method,
                url,
                body,
                headers,
                output_var,
            } => {
                let mut req_headers = headers.clone().unwrap_or_default();

                // Inject Idempotency-Key header if specified
                if let Some(ref key) = step.idempotency_key {
                    req_headers.insert("Idempotency-Key".to_string(), key.clone());
                }

                step_ops.push(LogicOperation::HttpRequest {
                    url: url.clone(),
                    method: method.clone(),
                    body: body.clone(),
                    headers: if req_headers.is_empty() {
                        None
                    } else {
                        Some(req_headers)
                    },
                    timeout_ms: step.timeout_ms,
                });

                // If output_var is specified, capture {{http_response}} into output_var
                if let Some(ref out) = output_var {
                    step_ops.push(LogicOperation::Set {
                        var: out.clone(),
                        value: serde_json::json!("{{http_response}}"),
                    });
                }
            }
            WorkflowOpV1::If {
                condition,
                then,
                r#else,
            } => {
                let then_ops = then.iter().flat_map(Self::compile_step).collect();
                let else_ops = r#else
                    .as_ref()
                    .map(|e| e.iter().flat_map(Self::compile_step).collect());

                step_ops.push(LogicOperation::If {
                    condition: condition.clone(),
                    then: then_ops,
                    otherwise: else_ops,
                });
            }
            WorkflowOpV1::Switch {
                value,
                cases,
                default,
            } => {
                let mut compiled_cases = Vec::new();
                for (case_val, case_steps) in cases {
                    compiled_cases.push(proto::models::SwitchCase {
                        value: serde_json::Value::String(case_val.clone()),
                        operations: case_steps.iter().flat_map(Self::compile_step).collect(),
                    });
                }
                let compiled_default = default
                    .as_ref()
                    .map(|d| d.iter().flat_map(Self::compile_step).collect());

                step_ops.push(LogicOperation::Switch {
                    value: value.clone(),
                    cases: compiled_cases,
                    default: compiled_default,
                });
            }
            WorkflowOpV1::Loop {
                items,
                as_var,
                body,
                ..
            } => {
                let body_ops = body.iter().flat_map(Self::compile_step).collect();
                step_ops.push(LogicOperation::Loop {
                    collection: items.clone(),
                    var: as_var.clone(),
                    body: body_ops,
                });
            }
            WorkflowOpV1::Transform { expr, output_var } => {
                let trimmed = expr.trim();
                let val = if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
                    serde_json::json!(trimmed)
                } else if let Ok(n) = trimmed.parse::<i64>() {
                    serde_json::json!(n)
                } else if let Ok(f) = trimmed.parse::<f64>() {
                    serde_json::json!(f)
                } else if let Ok(b) = trimmed.parse::<bool>() {
                    serde_json::json!(b)
                } else {
                    serde_json::json!(format!("{{{{{}}}}}", trimmed))
                };
                step_ops.push(LogicOperation::Set {
                    var: output_var.clone(),
                    value: val,
                });
            }
            WorkflowOpV1::Set { var, value } => {
                step_ops.push(LogicOperation::Set {
                    var: var.clone(),
                    value: value.clone(),
                });
            }
            WorkflowOpV1::TryCatch {
                r#try,
                catch,
                error_var: _,
            } => {
                let try_ops = r#try.iter().flat_map(Self::compile_step).collect();
                let catch_ops = catch.iter().flat_map(Self::compile_step).collect();
                step_ops.push(LogicOperation::Try {
                    body: try_ops,
                    catch: catch_ops,
                    finally: None,
                });
            }
            WorkflowOpV1::Parallel {
                branches,
                max_concurrent,
            } => {
                let branch_ops = branches
                    .iter()
                    .map(|b| b.iter().flat_map(Self::compile_step).collect())
                    .collect();
                step_ops.push(LogicOperation::Parallel {
                    tasks: branch_ops,
                    max_concurrent: *max_concurrent,
                });
            }
            WorkflowOpV1::Approval { summary, .. } => {
                step_ops.push(LogicOperation::Comment {
                    text: format!("APPROVAL_GATE: {}", summary),
                });
                step_ops.push(LogicOperation::Set {
                    var: format!("{}_approved", step.id),
                    value: serde_json::json!(true),
                });
            }
            WorkflowOpV1::Fail { message, code } => {
                step_ops.push(LogicOperation::Throw {
                    message: message.clone(),
                    code: code.clone(),
                });
            }
        }

        step_ops
    }
}
