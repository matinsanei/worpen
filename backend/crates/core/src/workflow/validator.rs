use proto::models::{WorkflowOpV1, WorkflowSpecV1, WorkflowStepV1};
use std::collections::HashSet;

pub struct WorkflowValidator;

impl WorkflowValidator {
    /// Strict validation of Workflow Spec v1 at registration.
    /// Checks:
    /// - Non-empty and valid workflow name
    /// - Valid input schema (if provided)
    /// - Unbounded loop prevention (must have max_iterations > 0 and within policy caps)
    /// - Undefined variable detection (every variable referenced in steps/return must be defined in input_schema or prior step outputs)
    pub fn validate(spec: &WorkflowSpecV1) -> Result<(), String> {
        // 1. Validate workflow name
        if spec.name.trim().is_empty() {
            return Err("Workflow name cannot be empty".to_string());
        }
        if !spec
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(format!(
                "Invalid workflow name '{}': only alphanumeric, '-' and '_' allowed",
                spec.name
            ));
        }

        // 2. Validate input schema if present
        if let Some(schema) = &spec.input_schema {
            jsonschema::JSONSchema::compile(schema)
                .map_err(|e| format!("Invalid input_schema JSON-Schema: {}", e))?;
        }

        // 3. Initialize defined variables set
        let mut defined_vars = HashSet::new();
        // Context built-ins
        defined_vars.insert("payload".to_string());
        defined_vars.insert("query".to_string());
        defined_vars.insert("headers".to_string());

        // Extract declared properties from input_schema
        if let Some(schema) = &spec.input_schema {
            if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                for key in props.keys() {
                    defined_vars.insert(key.clone());
                }
            }
        }

        // Policy loop cap
        let max_policy_loop = spec
            .policy
            .as_ref()
            .map(|p| p.max_loop_iterations)
            .unwrap_or(1000);

        // 4. Validate steps sequentially
        for step in &spec.steps {
            Self::validate_step(step, &mut defined_vars, max_policy_loop)?;
        }

        // 5. Validate return expression / variables
        Self::validate_value_references(&spec.r#return.value, &defined_vars, "return")?;

        Ok(())
    }

    fn validate_step(
        step: &WorkflowStepV1,
        defined_vars: &mut HashSet<String>,
        max_policy_loop: u32,
    ) -> Result<(), String> {
        if step.id.trim().is_empty() {
            return Err("Step ID cannot be empty".to_string());
        }

        match &step.op {
            WorkflowOpV1::Sql {
                query,
                args,
                output_var,
            } => {
                if query.trim().is_empty() {
                    return Err(format!("Step '{}': SQL query cannot be empty", step.id));
                }
                for arg in args {
                    Self::validate_value_references(arg, defined_vars, &step.id)?;
                }
                defined_vars.insert(output_var.clone());
            }
            WorkflowOpV1::Http {
                method,
                url,
                body,
                output_var,
                ..
            } => {
                if method.trim().is_empty() || url.trim().is_empty() {
                    return Err(format!(
                        "Step '{}': HTTP method and URL cannot be empty",
                        step.id
                    ));
                }
                Self::validate_string_references(url, defined_vars, &step.id)?;
                if let Some(b) = body {
                    Self::validate_value_references(b, defined_vars, &step.id)?;
                }
                if let Some(out) = output_var {
                    defined_vars.insert(out.clone());
                }
            }
            WorkflowOpV1::If {
                condition,
                then,
                r#else,
            } => {
                Self::validate_string_references(condition, defined_vars, &step.id)?;
                let mut then_vars = defined_vars.clone();
                for s in then {
                    Self::validate_step(s, &mut then_vars, max_policy_loop)?;
                }
                if let Some(else_steps) = r#else {
                    let mut else_vars = defined_vars.clone();
                    for s in else_steps {
                        Self::validate_step(s, &mut else_vars, max_policy_loop)?;
                    }
                }
            }
            WorkflowOpV1::Switch {
                value,
                cases,
                default,
            } => {
                Self::validate_string_references(value, defined_vars, &step.id)?;
                for branch in cases.values() {
                    let mut case_vars = defined_vars.clone();
                    for s in branch {
                        Self::validate_step(s, &mut case_vars, max_policy_loop)?;
                    }
                }
                if let Some(def_steps) = default {
                    let mut def_vars = defined_vars.clone();
                    for s in def_steps {
                        Self::validate_step(s, &mut def_vars, max_policy_loop)?;
                    }
                }
            }
            WorkflowOpV1::Loop {
                items,
                as_var,
                body,
                max_iterations,
            } => {
                if *max_iterations == 0 {
                    return Err(format!(
                        "Step '{}': loop max_iterations must be greater than 0",
                        step.id
                    ));
                }
                if *max_iterations > max_policy_loop {
                    return Err(format!(
                        "Step '{}': loop max_iterations ({}) exceeds policy cap ({})",
                        step.id, max_iterations, max_policy_loop
                    ));
                }
                Self::validate_string_references(items, defined_vars, &step.id)?;
                let mut loop_vars = defined_vars.clone();
                loop_vars.insert(as_var.clone());
                for s in body {
                    Self::validate_step(s, &mut loop_vars, max_policy_loop)?;
                }
            }
            WorkflowOpV1::Transform { expr, output_var } => {
                if expr.trim().is_empty() {
                    return Err(format!(
                        "Step '{}': Transform expr cannot be empty",
                        step.id
                    ));
                }
                Self::validate_expression_references(expr, defined_vars, &step.id)?;
                defined_vars.insert(output_var.clone());
            }
            WorkflowOpV1::Set { var, value } => {
                Self::validate_value_references(value, defined_vars, &step.id)?;
                defined_vars.insert(var.clone());
            }
            WorkflowOpV1::TryCatch {
                r#try,
                catch,
                error_var,
            } => {
                let mut try_vars = defined_vars.clone();
                for s in r#try {
                    Self::validate_step(s, &mut try_vars, max_policy_loop)?;
                }
                let mut catch_vars = defined_vars.clone();
                if let Some(err_name) = error_var {
                    catch_vars.insert(err_name.clone());
                }
                for s in catch {
                    Self::validate_step(s, &mut catch_vars, max_policy_loop)?;
                }
            }
            WorkflowOpV1::Parallel { branches, .. } => {
                for branch in branches {
                    let mut branch_vars = defined_vars.clone();
                    for s in branch {
                        Self::validate_step(s, &mut branch_vars, max_policy_loop)?;
                    }
                }
            }
            WorkflowOpV1::Approval { summary, .. } => {
                Self::validate_string_references(summary, defined_vars, &step.id)?;
                defined_vars.insert(format!("{}_approved", step.id));
                defined_vars.insert("__approved_by".to_string());
                defined_vars.insert("__approval_reason".to_string());
                defined_vars.insert("__approved_at".to_string());
            }
            WorkflowOpV1::Fail { message, .. } => {
                Self::validate_string_references(message, defined_vars, &step.id)?;
            }
        }

        Ok(())
    }

    fn validate_string_references(
        s: &str,
        defined_vars: &HashSet<String>,
        step_id: &str,
    ) -> Result<(), String> {
        let re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        for cap in re.captures_iter(s) {
            if let Some(matched) = cap.get(1) {
                let raw_var = matched.as_str().trim();
                let root_var = raw_var
                    .split('.')
                    .next()
                    .unwrap_or(raw_var)
                    .split('[')
                    .next()
                    .unwrap_or(raw_var);

                // Allow secret references like {{secret:stripe_key}} (handled in Phase 3)
                if root_var.starts_with("secret:") {
                    continue;
                }

                if !defined_vars.contains(root_var) {
                    return Err(format!(
                        "Step '{}': undefined variable '{}' referenced in template '{{{{{}}}}}'",
                        step_id, root_var, raw_var
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_expression_references(
        expr: &str,
        defined_vars: &HashSet<String>,
        step_id: &str,
    ) -> Result<(), String> {
        // First check for {{var}} syntax
        if expr.contains("{{") {
            Self::validate_string_references(expr, defined_vars, step_id)?;
        }

        // Also tokenize raw mathematical expressions
        let mut tokenizer = crate::expression::Tokenizer::new(expr);
        if let Ok(tokens) = tokenizer.tokenize() {
            for token in tokens {
                if let crate::expression::tokenizer::TokenType::Identifier(id) = token.token_type {
                    let id_str = id.as_ref();
                    // Built-in functions & math helpers
                    let builtins = [
                        "min", "max", "round", "floor", "ceil", "abs", "len", "sqrt", "keys",
                        "values", "true", "false", "null", "now", "upper", "lower", "trim",
                    ];
                    if builtins.contains(&id_str) {
                        continue;
                    }
                    let root_var = id_str
                        .split('.')
                        .next()
                        .unwrap_or(id_str)
                        .split('[')
                        .next()
                        .unwrap_or(id_str);

                    if root_var.starts_with("secret:") {
                        continue;
                    }

                    if !defined_vars.contains(root_var) {
                        return Err(format!(
                            "Step '{}': undefined variable '{}' in expression '{}'",
                            step_id, root_var, expr
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_value_references(
        val: &serde_json::Value,
        defined_vars: &HashSet<String>,
        context_name: &str,
    ) -> Result<(), String> {
        match val {
            serde_json::Value::String(s) => {
                Self::validate_string_references(s, defined_vars, context_name)
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    Self::validate_value_references(item, defined_vars, context_name)?;
                }
                Ok(())
            }
            serde_json::Value::Object(obj) => {
                for item in obj.values() {
                    Self::validate_value_references(item, defined_vars, context_name)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
