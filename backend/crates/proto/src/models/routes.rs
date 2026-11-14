use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum RouteType {
    #[default]
    Http,
    WebSocket,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum LogicOperation {
    // Data Operations
    #[serde(rename = "return")]
    Return {
        value: serde_json::Value,
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
    QueryDb {
        query: String,
        params: Vec<serde_json::Value>,
    },

    #[serde(rename = "sql_op")]
    SqlOp {
        query: String,
        args: Vec<serde_json::Value>,
        output_var: String,
    },

    #[serde(rename = "redis_op")]
    RedisOp {
        command: String,            // "GET", "SET", "DEL", "EXPIRE", "INCR", "DECR"
        key: String,                // Target key (supports {{vars}})
        value: Option<String>,      // Value for SET (supports {{vars}})
        ttl_seconds: Option<u64>,   // For EXPIRE or SETEX
        output_var: Option<String>, // Where to store result
    },

    #[serde(rename = "ws_op")]
    WsOp {
        command: String,         // "send" | "broadcast"
        message: String,         // Message to send (supports {{vars}})
        channel: Option<String>, // Optional channel for targeted broadcast
    },

    #[serde(rename = "http_request")]
    HttpRequest {
        url: String,
        method: String,
        body: Option<serde_json::Value>,
        headers: Option<HashMap<String, String>>,
        timeout_ms: Option<u64>,
    },

    // Control Flow - Basic
    #[serde(rename = "if")]
    If {
        condition: String,
        then: Vec<LogicOperation>,
        otherwise: Option<Vec<LogicOperation>>,
    },

    #[serde(rename = "loop")]
    Loop {
        collection: String,
        var: String,
        body: Vec<LogicOperation>,
    },

    // Control Flow - Advanced
    #[serde(rename = "switch")]
    Switch {
        value: String,
        cases: Vec<SwitchCase>,
        default: Option<Vec<LogicOperation>>,
    },

    #[serde(rename = "while")]
    While {
        condition: String,
        body: Vec<LogicOperation>,
        max_iterations: Option<u32>,
    },

    #[serde(rename = "break")]
    Break,

    #[serde(rename = "continue")]
    Continue,

    // Error Handling
    #[serde(rename = "try")]
    Try {
        body: Vec<LogicOperation>,
        catch: Vec<LogicOperation>,
        finally: Option<Vec<LogicOperation>>,
    },

    #[serde(rename = "throw")]
    Throw {
        message: String,
        code: Option<String>,
    },

    // Parallel Execution
    #[serde(rename = "parallel")]
    Parallel {
        tasks: Vec<Vec<LogicOperation>>,
        max_concurrent: Option<usize>,
    },

    #[serde(rename = "await_all")]
    AwaitAll { task_ids: Vec<String> },

    // Function Operations
    #[serde(rename = "define_function")]
    DefineFunction {
        name: String,
        params: Vec<String>,
        body: Vec<LogicOperation>,
    },

    #[serde(rename = "call_function")]
    CallFunction {
        name: String,
        args: Vec<serde_json::Value>,
        output_var: String,
    },

    // Data Transformations
    #[serde(rename = "map")]
    Map { input: String, transform: String },

    #[serde(rename = "filter")]
    Filter { input: String, condition: String },

    #[serde(rename = "aggregate")]
    Aggregate { input: String, operation: String }, // sum, count, avg, min, max

    // Variable Operations
    #[serde(rename = "set")]
    Set {
        var: String,
        value: serde_json::Value,
    },

    #[serde(rename = "get")]
    Get { var: String },

    // Helper Functions
    #[serde(rename = "string_op")]
    StringOp {
        operation: String,
        input: String,
        args: Vec<serde_json::Value>,
    }, // split, join, upper, lower, trim, replace, regex_match

    #[serde(rename = "math_op")]
    MathOp {
        operation: String,
        args: Vec<serde_json::Value>,
    }, // sum, avg, min, max, round, ceil, floor, abs, pow, sqrt

    #[serde(rename = "date_op")]
    DateOp {
        operation: String,
        args: Vec<serde_json::Value>,
    }, // now, parse, format, add, diff

    #[serde(rename = "json_op")]
    JsonOp {
        operation: String,
        input: String,
        args: Vec<serde_json::Value>,
    }, // parse, stringify, merge, get_path, set_path

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
    CustomOp(HashMap<String, serde_json::Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SwitchCase {
    pub value: serde_json::Value,
    pub operations: Vec<LogicOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WebSocketHooks {
    #[serde(default)]
    pub on_connect: Vec<LogicOperation>,
    pub on_message: Vec<LogicOperation>,
    #[serde(default)]
    pub on_disconnect: Vec<LogicOperation>,
}

/// Capability-based sandbox policy per route (Phase 1).
/// `None` on a route = legacy permissive mode (backward compatible).
/// New routes created via CLI get `SandboxPolicy::restrictive_default()`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SandboxPolicy {
    /// Allow outbound HTTP requests at all
    #[serde(default = "default_true")]
    pub allow_net: bool,
    /// Allowed URL hosts (substring/suffix match). Empty = allow all (if allow_net).
    /// Example: ["api.stripe.com", "127.0.0.1"]
    #[serde(default)]
    pub allowed_hosts: Vec<String>,
    /// Allow SQL operations
    #[serde(default = "default_true")]
    pub allow_sql: bool,
    /// Allow Redis operations
    #[serde(default = "default_true")]
    pub allow_redis: bool,
    /// Allow arbitrary script execution (`execute_script`)
    #[serde(default)]
    pub allow_scripts: bool,
    /// Max total execution time per request (ms)
    #[serde(default = "default_timeout")]
    pub max_timeout_ms: u64,
    /// Max sleep per `sleep` op (ms) — DoS guard
    #[serde(default = "default_max_sleep")]
    pub max_sleep_ms: u64,
    /// Default cap for `while` loops without explicit max_iterations
    #[serde(default = "default_max_iters")]
    pub max_loop_iterations: u32,
    /// Max distinct variables per execution (memory DoS guard)
    #[serde(default = "default_max_vars")]
    pub max_variables: usize,
}

fn default_true() -> bool {
    true
}
fn default_timeout() -> u64 {
    10_000
}
fn default_max_sleep() -> u64 {
    5_000
}
fn default_max_iters() -> u32 {
    1000
}
fn default_max_vars() -> usize {
    5000
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            allow_net: true,
            allowed_hosts: vec![],
            allow_sql: true,
            allow_redis: true,
            allow_scripts: false,
            max_timeout_ms: 10_000,
            max_sleep_ms: 5_000,
            max_loop_iterations: 1000,
            max_variables: 5000,
        }
    }
}

impl SandboxPolicy {
    /// Strict default for AI-generated / untrusted routes.
    pub fn restrictive_default() -> Self {
        Self {
            allow_net: false,
            allowed_hosts: vec![],
            allow_sql: false,
            allow_redis: false,
            allow_scripts: false,
            max_timeout_ms: 10_000,
            max_sleep_ms: 1_000,
            max_loop_iterations: 500,
            max_variables: 500,
        }
    }

    /// Host allowlist check. Supports exact, suffix (".example.com") and substring.
    pub fn is_host_allowed(&self, host: &str) -> bool {
        if !self.allow_net {
            return false;
        }
        if self.allowed_hosts.is_empty() {
            return true; // legacy permissive
        }
        let h = host.to_lowercase();
        self.allowed_hosts.iter().any(|a| {
            let a = a.to_lowercase();
            h == a || h.ends_with(&format!(".{}", a)) || h.contains(&a)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub method: HttpMethod,
    #[serde(default)]
    pub route_type: RouteType,
    pub logic: Vec<LogicOperation>,
    pub ws_hooks: Option<WebSocketHooks>,
    pub parameters: Vec<RouteParameter>,
    pub response_schema: Option<serde_json::Value>,
    #[serde(default)]
    pub auth_required: bool,
    pub rate_limit: Option<u32>,
    pub enabled: bool,
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: String,
    /// Sandbox policy. None = legacy permissive (backward compatible).
    #[serde(default)]
    pub sandbox: Option<SandboxPolicy>,
}

/// Request body for registering a new route (id, timestamps, created_by are auto-generated)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterRouteRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub path: String,
    pub method: HttpMethod,
    #[serde(default)]
    pub route_type: RouteType,
    pub logic: Vec<LogicOperation>,
    pub ws_hooks: Option<WebSocketHooks>,
    #[serde(default)]
    pub parameters: Vec<RouteParameter>,
    pub response_schema: Option<serde_json::Value>,
    #[serde(default)]
    pub auth_required: bool,
    pub rate_limit: Option<u32>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_version")]
    pub version: String,
    /// Optional sandbox policy. If omitted, server applies safe default.
    #[serde(default)]
    pub sandbox: Option<SandboxPolicy>,
}

fn default_enabled() -> bool {
    true
}

fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteParameter {
    pub name: String,
    pub param_type: String, // "path", "query", "body"
    pub data_type: String,  // "string", "number", "boolean", "object", "array"
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation: Option<String>, // regex or validation rule
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteTestRequest {
    pub route_id: String,
    pub test_payload: Option<serde_json::Value>,
    pub test_params: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RouteTestResponse {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub steps_executed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DynamicRouteExecutionContext {
    pub route_id: String,
    pub variables: HashMap<String, serde_json::Value>,
    pub request_payload: Option<serde_json::Value>,
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub functions: HashMap<String, FunctionDefinition>,
    pub loop_control: LoopControl,
    pub error_context: Option<ErrorContext>,
    /// Active sandbox policy for this execution (None = legacy permissive).
    #[serde(default)]
    pub sandbox: Option<SandboxPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FunctionDefinition {
    pub params: Vec<String>,
    pub body: Vec<LogicOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<String>,
    pub logic: Vec<LogicOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct LoopControl {
    pub should_break: bool,
    pub should_continue: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorContext {
    pub message: String,
    pub code: Option<String>,
    pub stack: Vec<String>,
}
