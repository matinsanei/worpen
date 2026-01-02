use crate::vm::memory::ExecutionMemory;
use crate::compiler::symbol_table::SymbolTable;
use crate::vm::instructions::OptimizedOperation;
use proto::models::LoopControl;
use serde_json::Value;
use regex::Regex;
use sqlx::{Row, Column};

pub struct VirtualMachine {
    pub memory: ExecutionMemory,
    symbol_table: SymbolTable,
    db_pool: Option<sqlx::Pool<sqlx::Sqlite>>,
}

impl VirtualMachine {
    pub fn new(memory: ExecutionMemory, symbol_table: SymbolTable) -> Self {
        Self { 
            memory, 
            symbol_table,
            db_pool: None,
        }
    }
    
    pub fn with_db_pool(memory: ExecutionMemory, symbol_table: SymbolTable, db_pool: sqlx::Pool<sqlx::Sqlite>) -> Self {
        Self { 
            memory, 
            symbol_table,
            db_pool: Some(db_pool),
        }
    }

    pub async fn execute(&mut self, program: &[OptimizedOperation]) -> Result<Value, String> {
        let mut result = Value::Null;
        let mut loop_control = LoopControl::default();

        for op in program.iter() {
            // Handle loop control
            if loop_control.should_break {
                break;
            }
            if loop_control.should_continue {
                loop_control.should_continue = false;
                continue;
            }

            match op {
                OptimizedOperation::Return { value } => {
                    result = self.resolve_value(value)?;
                    break;
                },
                OptimizedOperation::Set { var_index, value } => {
                    let resolved = self.resolve_value(value)?;
                    self.memory.set(*var_index, resolved);
                },
                OptimizedOperation::Get { var_index } => {
                    result = self.memory.get(*var_index).cloned().unwrap_or(Value::Null);
                },
                OptimizedOperation::MathOp { operation, args } => {
                    let resolved_args: Result<Vec<Value>, String> = args.iter()
                        .map(|arg| self.resolve_value(arg))
                        .collect();
                    let resolved_args = resolved_args?;
                    result = self.execute_math_op(operation, &resolved_args)?;
                },
                OptimizedOperation::If { condition, then, otherwise } => {
                    let resolved_condition = self.resolve_string(condition)?;
                    let condition_result = self.evaluate_condition(&resolved_condition)?;
                    if condition_result {
                        result = Box::pin(self.execute(then)).await?;
                    } else if let Some(else_ops) = otherwise {
                        result = Box::pin(self.execute(else_ops)).await?;
                    }
                },
                OptimizedOperation::SqlOp { query, arg_indices, output_var_index } => {
                    // Resolve all args from memory
                    let resolved_args: Vec<sqlx::Either<String, i64>> = arg_indices.iter()
                        .map(|idx| {
                            if let Some(val) = self.memory.get(*idx) {
                                // Convert Value to sqlx compatible type
                                match val {
                                    Value::Number(n) => {
                                        if let Some(i) = n.as_i64() {
                                            Ok(sqlx::Either::Right(i))
                                        } else if let Some(f) = n.as_f64() {
                                            Ok(sqlx::Either::Left(f.to_string()))
                                        } else {
                                            Ok(sqlx::Either::Left(n.to_string()))
                                        }
                                    },
                                    Value::String(s) => Ok(sqlx::Either::Left(s.clone())),
                                    _ => Ok(sqlx::Either::Left(val.to_string())),
                                }
                            } else {
                                Err(format!("Variable at index {} not set", idx))
                            }
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    
                    // Execute SQL query
                    if let Some(pool) = &self.db_pool {
                        let mut query_builder = sqlx::query(query);
                        
                        // Bind all arguments
                        for arg in resolved_args {
                            match arg {
                                sqlx::Either::Left(s) => {
                                    query_builder = query_builder.bind(s);
                                },
                                sqlx::Either::Right(i) => {
                                    query_builder = query_builder.bind(i);
                                },
                            }
                        }
                        
                        // Execute and fetch results
                        let rows = query_builder.fetch_all(pool).await
                            .map_err(|e| format!("SQL execution error: {}", e))?;
                        
                        // Convert rows to JSON array
                        let json_rows: Vec<Value> = rows.iter()
                            .map(|row| {
                                let mut obj = serde_json::Map::new();
                                for (i, col) in row.columns().iter().enumerate() {
                                    let col_name = col.name().to_string();
                                    
                                    // Try to get the value as different types
                                    let val = if let Ok(v) = row.try_get::<String, _>(i) {
                                        Value::String(v)
                                    } else if let Ok(v) = row.try_get::<i64, _>(i) {
                                        Value::Number(v.into())
                                    } else if let Ok(v) = row.try_get::<f64, _>(i) {
                                        Value::Number(serde_json::Number::from_f64(v).unwrap_or(0.into()))
                                    } else if let Ok(v) = row.try_get::<bool, _>(i) {
                                        Value::Bool(v)
                                    } else {
                                        Value::Null
                                    };
                                    
                                    obj.insert(col_name, val);
                                }
                                Value::Object(obj)
                            })
                            .collect();
                        
                        // Store result in memory
                        self.memory.set(*output_var_index, Value::Array(json_rows.clone()));
                        result = Value::Array(json_rows);
                    } else {
                        return Err("Database pool not available for SqlOp".to_string());
                    }
                },
                // Add more operations as needed
                _ => {
                    // For now, skip unimplemented operations
                }
            }
        }

        Ok(result)
    }

    fn resolve_value(&self, value: &Value) -> Result<Value, String> {
        match value {
            Value::String(s) => {
                let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
                let mut result = s.clone();

                for cap in re.captures_iter(s) {
                    if let (Some(full_match), Some(var_match)) = (cap.get(0), cap.get(1)) {
                        let var_name = var_match.as_str();
                        if let Some(index) = self.symbol_table.get_index(var_name) {
                            if let Some(val) = self.memory.get(index) {
                                let val_str = val.to_string();
                                result = result.replace(full_match.as_str(), &val_str);
                            } else {
                                return Err(format!("Variable '{}' not set", var_name));
                            }
                        } else {
                            return Err(format!("Unknown variable '{}'", var_name));
                        }
                    }
                }

                // Try to parse as JSON, otherwise keep as string
                if let Ok(parsed) = serde_json::from_str(&result) {
                    Ok(parsed)
                } else {
                    Ok(Value::String(result))
                }
            },
            Value::Array(arr) => {
                let resolved: Result<Vec<Value>, String> = arr.iter()
                    .map(|v| self.resolve_value(v))
                    .collect();
                Ok(Value::Array(resolved?))
            },
            Value::Object(obj) => {
                let resolved: Result<serde_json::Map<String, Value>, String> = obj.iter()
                    .map(|(k, v)| {
                        self.resolve_value(v).map(|rv| (k.clone(), rv))
                    })
                    .collect();
                Ok(Value::Object(resolved?))
            },
            _ => Ok(value.clone()),
        }
    }

    fn resolve_string(&self, s: &str) -> Result<String, String> {
        let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let mut result = s.to_string();

        for cap in re.captures_iter(s) {
            if let (Some(full_match), Some(var_match)) = (cap.get(0), cap.get(1)) {
                let var_name = var_match.as_str();
                if let Some(index) = self.symbol_table.get_index(var_name) {
                    if let Some(val) = self.memory.get(index) {
                        let val_str = val.to_string();
                        result = result.replace(full_match.as_str(), &val_str);
                    } else {
                        return Err(format!("Variable '{}' not set", var_name));
                    }
                } else {
                    return Err(format!("Unknown variable '{}'", var_name));
                }
            }
        }

        Ok(result)
    }

    fn evaluate_condition(&self, condition: &str) -> Result<bool, String> {
        // Simple condition evaluation - for now, assume it's a boolean string or comparison
        // This is a placeholder; in real implementation, you'd parse and evaluate expressions
        match condition.trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => {
                // Try to parse as JSON boolean
                if let Ok(val) = serde_json::from_str::<Value>(condition) {
                    match val {
                        Value::Bool(b) => Ok(b),
                        _ => Err(format!("Cannot evaluate condition: {}", condition)),
                    }
                } else {
                    Err(format!("Cannot evaluate condition: {}", condition))
                }
            }
        }
    }

    fn execute_math_op(&self, operation: &str, args: &[Value]) -> Result<Value, String> {
        match operation {
            "add" => {
                if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (Value::Number(a), Value::Number(b)) => {
                            let sum = a.as_f64().unwrap_or(0.0) + b.as_f64().unwrap_or(0.0);
                            Ok(Value::Number(serde_json::Number::from_f64(sum).unwrap_or(serde_json::Number::from(0))))
                        },
                        _ => Err("Add operation requires two numbers".to_string()),
                    }
                } else {
                    Err("Add operation requires exactly two arguments".to_string())
                }
            },
            _ => Err(format!("Unsupported math operation: {}", operation)),
        }
    }
}