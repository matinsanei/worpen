use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizedOperation {
    // Data Operations
    #[serde(rename = "return")]
    Return { 
        value: Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        raw: Option<bool>,
    },
    
    #[serde(rename = "comment")]
    Comment { text: String },

    #[serde(rename = "query_db")]
    QueryDb { query: String, params: Vec<Value> },
    
    #[serde(rename = "sql_op")]
    SqlOp { query: String, arg_indices: Vec<usize>, output_var_index: usize },
    
    #[serde(rename = "redis_op")]
    RedisOp {
        command: String,  // "GET", "SET", "DEL", "EXPIRE", "INCR", "DECR"
        key: String,      // Template string with {{vars}}
        value: Option<String>, // Template string with {{vars}}
        ttl_seconds: Option<u64>,
        output_var_index: Option<usize> // Where to store result
    },
    
    #[serde(rename = "ws_op")]
    WsOp {
        command: String,  // "send" | "broadcast"
        message: String,  // Template string with {{vars}}
        channel: Option<String>, // Optional channel
    },

    #[serde(rename = "http_request")]
    HttpRequest { url: String, method: String, body: Option<Value>, headers: Option<HashMap<String, String>>, timeout_ms: Option<u64> },

    // Control Flow - Basic
    #[serde(rename = "if")]
    If { condition: String, then: Vec<OptimizedOperation>, otherwise: Option<Vec<OptimizedOperation>> },

    #[serde(rename = "loop")]
    Loop { collection: String, var_index: usize, body: Vec<OptimizedOperation> },

    // Control Flow - Advanced
    #[serde(rename = "switch")]
    Switch { value: String, cases: Vec<OptimizedSwitchCase>, default: Option<Vec<OptimizedOperation>> },

    #[serde(rename = "while")]
    While { condition: String, body: Vec<OptimizedOperation>, max_iterations: Option<u32> },

    #[serde(rename = "break")]
    Break,

    #[serde(rename = "continue")]
    Continue,

    // Error Handling
    #[serde(rename = "try")]
    Try { body: Vec<OptimizedOperation>, catch: Vec<OptimizedOperation>, finally: Option<Vec<OptimizedOperation>> },

    #[serde(rename = "throw")]
    Throw { message: String, code: Option<String> },

    // Parallel Execution
    #[serde(rename = "parallel")]
    Parallel { tasks: Vec<Vec<OptimizedOperation>>, max_concurrent: Option<usize> },

    #[serde(rename = "await_all")]
    AwaitAll { task_ids: Vec<String> },

    // Function Operations
    #[serde(rename = "define_function")]
    DefineFunction { name: String, param_indices: Vec<usize>, body: Vec<OptimizedOperation> },

    #[serde(rename = "call_function")]
    CallFunction { name: String, args: Vec<Value>, output_var_index: usize },

    // Data Transformations
    #[serde(rename = "map")]
    Map { input: String, transform: String },

    #[serde(rename = "filter")]
    Filter { input: String, condition: String },

    #[serde(rename = "aggregate")]
    Aggregate { input: String, operation: String }, // sum, count, avg, min, max

    // Variable Operations
    #[serde(rename = "set")]
    Set { var_index: usize, value: Value },

    #[serde(rename = "get")]
    Get { var_index: usize },

    // Helper Functions
    #[serde(rename = "string_op")]
    StringOp { operation: String, input: String, args: Vec<Value> }, // split, join, upper, lower, trim, replace, regex_match

    #[serde(rename = "math_op")]
    MathOp { operation: String, args: Vec<Value> }, // sum, avg, min, max, round, ceil, floor, abs, pow, sqrt

    #[serde(rename = "date_op")]
    DateOp { operation: String, args: Vec<Value> }, // now, parse, format, add, diff

    #[serde(rename = "json_op")]
    JsonOp { operation: String, input: String, args: Vec<Value> }, // parse, stringify, merge, get_path, set_path

    // Logging & Debugging
    #[serde(rename = "log")]
    Log { level: String, message: String },

    #[serde(rename = "sleep")]
    Sleep { duration_ms: u64 },

    // Custom Script Execution
    #[serde(rename = "execute_script")]
    ExecuteScript { language: String, code: String },
    
    // Custom Operations (for user-defined extensions)
    #[serde(untagged)]
    CustomOp(HashMap<String, Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedSwitchCase {
    pub value: Value,
    pub operations: Vec<OptimizedOperation>,
}