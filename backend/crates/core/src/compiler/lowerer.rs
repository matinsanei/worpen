use crate::compiler::symbol_table::SymbolTable;
use crate::vm::instructions::{OptimizedOperation, OptimizedSwitchCase};
use proto::models::LogicOperation;
use serde_json::Value;
use regex::Regex;

pub struct LogicCompiler {
    symbol_table: SymbolTable,
}

impl LogicCompiler {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn compile(&mut self, logic: &[LogicOperation]) -> Vec<OptimizedOperation> {
        logic.iter().map(|op| self.compile_operation(op)).collect()
    }

    fn compile_operation(&mut self, op: &LogicOperation) -> OptimizedOperation {
        match op {
            LogicOperation::Return { value, status, headers, raw } => {
                self.register_variables_in_value(value);
                OptimizedOperation::Return { 
                    value: value.clone(),
                    status: *status,
                    headers: headers.clone(),
                    raw: *raw,
                }
            },
            LogicOperation::Comment { text } => {
                // Comment is a no-op in the optimized code
                OptimizedOperation::Comment { text: text.clone() }
            },
            LogicOperation::Set { var, value } => {
                self.register_variables_in_value(value);
                let var_index = self.symbol_table.register(var.clone());
                OptimizedOperation::Set { var_index, value: value.clone() }
            },
            LogicOperation::Get { var } => {
                let var_index = self.symbol_table.register(var.clone());
                OptimizedOperation::Get { var_index }
            },
            LogicOperation::QueryDb { query, params } => {
                for param in params {
                    self.register_variables_in_value(param);
                }
                OptimizedOperation::QueryDb { query: query.clone(), params: params.clone() }
            },
            LogicOperation::SqlOp { query, args, output_var } => {
                // Register variables in args
                for arg in args {
                    self.register_variables_in_value(arg);
                }
                // Register output variable
                let output_var_index = self.symbol_table.register(output_var.clone());
                
                // For args, we need to convert them to indices
                // We'll resolve the args at runtime from variable references
                let arg_indices: Vec<usize> = args.iter()
                    .filter_map(|arg| {
                        if let serde_json::Value::String(s) = arg {
                            // Extract variable name from {{var}}
                            let re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
                            if let Some(cap) = re.captures(s) {
                                if let Some(var_match) = cap.get(1) {
                                    let var_name = var_match.as_str();
                                    return Some(self.symbol_table.register(var_name.to_string()));
                                }
                            }
                        }
                        None
                    })
                    .collect();
                
                OptimizedOperation::SqlOp { 
                    query: query.clone(), 
                    arg_indices,
                    output_var_index 
                }
            },
            LogicOperation::RedisOp { command, key, value, ttl_seconds, output_var } => {
                // Register variables in key and value strings
                self.register_variables_in_string(key);
                if let Some(v) = value {
                    self.register_variables_in_string(v);
                }
                
                // Register output variable if provided
                let output_var_index = output_var.as_ref()
                    .map(|var| self.symbol_table.register(var.clone()));
                
                OptimizedOperation::RedisOp {
                    command: command.clone(),
                    key: key.clone(),
                    value: value.clone(),
                    ttl_seconds: *ttl_seconds,
                    output_var_index,
                }
            },
            LogicOperation::WsOp { command, message, channel } => {
                // Register variables in message string
                self.register_variables_in_string(message);
                if let Some(ch) = channel {
                    self.register_variables_in_string(ch);
                }
                
                OptimizedOperation::WsOp {
                    command: command.clone(),
                    message: message.clone(),
                    channel: channel.clone(),
                }
            },
            LogicOperation::HttpRequest { url, method, body, headers, timeout_ms } => {
                if let Some(b) = body {
                    self.register_variables_in_value(b);
                }
                OptimizedOperation::HttpRequest { url: url.clone(), method: method.clone(), body: body.clone(), headers: headers.clone(), timeout_ms: *timeout_ms }
            },
            LogicOperation::If { condition, then, otherwise } => {
                self.register_variables_in_string(condition);
                let then_ops = self.compile(then);
                let otherwise_ops = otherwise.as_ref().map(|ops| self.compile(ops));
                OptimizedOperation::If { condition: condition.clone(), then: then_ops, otherwise: otherwise_ops }
            },
            LogicOperation::Loop { collection, var, body } => {
                self.register_variables_in_string(collection);
                let var_index = self.symbol_table.register(var.clone());
                let body_ops = self.compile(body);
                OptimizedOperation::Loop { collection: collection.clone(), var_index, body: body_ops }
            },
            LogicOperation::Switch { value, cases, default } => {
                self.register_variables_in_string(value);
                let cases_ops = cases.iter().map(|case| {
                    self.register_variables_in_value(&case.value);
                    OptimizedSwitchCase {
                        value: case.value.clone(),
                        operations: self.compile(&case.operations),
                    }
                }).collect();
                let default_ops = default.as_ref().map(|ops| self.compile(ops));
                OptimizedOperation::Switch { value: value.clone(), cases: cases_ops, default: default_ops }
            },
            LogicOperation::While { condition, body, max_iterations } => {
                self.register_variables_in_string(condition);
                let body_ops = self.compile(body);
                OptimizedOperation::While { condition: condition.clone(), body: body_ops, max_iterations: *max_iterations }
            },
            LogicOperation::Break => OptimizedOperation::Break,
            LogicOperation::Continue => OptimizedOperation::Continue,
            LogicOperation::Try { body, catch, finally } => {
                let body_ops = self.compile(body);
                let catch_ops = self.compile(catch);
                let finally_ops = finally.as_ref().map(|ops| self.compile(ops));
                OptimizedOperation::Try { body: body_ops, catch: catch_ops, finally: finally_ops }
            },
            LogicOperation::Throw { message, code } => {
                OptimizedOperation::Throw { message: message.clone(), code: code.clone() }
            },
            LogicOperation::Parallel { tasks, max_concurrent } => {
                let tasks_ops = tasks.iter().map(|task| self.compile(task)).collect();
                OptimizedOperation::Parallel { tasks: tasks_ops, max_concurrent: *max_concurrent }
            },
            LogicOperation::AwaitAll { task_ids } => {
                OptimizedOperation::AwaitAll { task_ids: task_ids.clone() }
            },
            LogicOperation::DefineFunction { name, params, body } => {
                let param_indices = params.iter().map(|p| self.symbol_table.register(p.clone())).collect();
                let body_ops = self.compile(body);
                OptimizedOperation::DefineFunction { name: name.clone(), param_indices, body: body_ops }
            },
            LogicOperation::CallFunction { name, args, output_var } => {
                for arg in args {
                    self.register_variables_in_value(arg);
                }
                let output_var_index = self.symbol_table.register(output_var.clone());
                OptimizedOperation::CallFunction { name: name.clone(), args: args.clone(), output_var_index }
            },
            LogicOperation::Map { input, transform } => {
                self.register_variables_in_string(input);
                self.register_variables_in_string(transform);
                OptimizedOperation::Map { input: input.clone(), transform: transform.clone() }
            },
            LogicOperation::Filter { input, condition } => {
                self.register_variables_in_string(input);
                self.register_variables_in_string(condition);
                OptimizedOperation::Filter { input: input.clone(), condition: condition.clone() }
            },
            LogicOperation::Aggregate { input, operation } => {
                self.register_variables_in_string(input);
                OptimizedOperation::Aggregate { input: input.clone(), operation: operation.clone() }
            },
            LogicOperation::StringOp { operation, input, args } => {
                self.register_variables_in_string(input);
                for arg in args {
                    self.register_variables_in_value(arg);
                }
                OptimizedOperation::StringOp { operation: operation.clone(), input: input.clone(), args: args.clone() }
            },
            LogicOperation::MathOp { operation, args } => {
                for arg in args {
                    self.register_variables_in_value(arg);
                }
                OptimizedOperation::MathOp { operation: operation.clone(), args: args.clone() }
            },
            LogicOperation::DateOp { operation, args } => {
                for arg in args {
                    self.register_variables_in_value(arg);
                }
                OptimizedOperation::DateOp { operation: operation.clone(), args: args.clone() }
            },
            LogicOperation::JsonOp { operation, input, args } => {
                self.register_variables_in_string(input);
                for arg in args {
                    self.register_variables_in_value(arg);
                }
                OptimizedOperation::JsonOp { operation: operation.clone(), input: input.clone(), args: args.clone() }
            },
            LogicOperation::Log { level, message } => {
                self.register_variables_in_string(message);
                OptimizedOperation::Log { level: level.clone(), message: message.clone() }
            },
            LogicOperation::Sleep { duration_ms } => {
                OptimizedOperation::Sleep { duration_ms: *duration_ms }
            },
            LogicOperation::ExecuteScript { language, code } => {
                OptimizedOperation::ExecuteScript { language: language.clone(), code: code.clone() }
            },
        }
    }

    fn register_variables_in_value(&mut self, value: &Value) {
        if let Value::String(s) = value {
            self.register_variables_in_string(s);
        }
        // For nested objects/arrays, we could recurse, but for now, just strings
    }

    fn register_variables_in_string(&mut self, s: &str) {
        // Use regex to find {{variable}} patterns
        let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        for cap in re.captures_iter(s) {
            if let Some(var) = cap.get(1) {
                self.symbol_table.register(var.as_str().to_string());
            }
        }
    }

    pub fn get_symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
}

impl Default for LogicCompiler {
    fn default() -> Self {
        Self::new()
    }
}