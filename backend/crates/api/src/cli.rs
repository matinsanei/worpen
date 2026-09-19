use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(name = "worpen", version, about = "Worpen Dynamic API Engine — single binary (API + UI + CLI + MCP)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Run the HTTP server (API + optional embedded UI)
    Serve(ServeArgs),
    /// Manage dynamic routes (talks to a running server)
    Route {
        #[command(subcommand)]
        cmd: RouteCmd,
    },
    /// Manage global functions
    Func {
        #[command(subcommand)]
        cmd: FuncCmd,
    },
    /// Health / diagnostics
    Doctor(DoctorArgs),
    /// MCP (Model Context Protocol) tools
    Mcp {
        #[command(subcommand)]
        cmd: McpCmd,
    },
    /// Multi-stage pipelines (Logic-as-Data jobs)
    Pipeline {
        #[command(subcommand)]
        cmd: PipeCmd,
    },
    /// Interactive first-run wizard (port, token, ui) — writes .env
    Setup,
}

#[derive(Parser, Debug, Clone)]
pub struct ServeArgs {
    #[arg(long, env = "HOST", default_value = "127.0.0.1")]
    pub host: String,
    #[arg(long, env = "PORT", default_value_t = 3000)]
    pub port: u16,
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite:worpen.db?mode=rwc")]
    pub database_url: String,
    /// Data dir (routes/functions/pipelines JSON). "auto" = backend/data from repo root, ./data from backend/.
    #[arg(long, env = "DATA_DIR", default_value = "auto")]
    pub data_dir: String,
    /// Disable serving the embedded frontend UI
    #[arg(long, env = "WORPEN_NO_UI", default_value_t = false)]
    pub no_ui: bool,
    /// Directory of prebuilt frontend (dist). Defaults to ../dist, ./dist, /app/ui
    #[arg(long, env = "FRONTEND_DIR")]
    pub ui_dir: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RouteCmd {
    /// List routes from server
    List(ServerRef),
    /// Add route from JSON/YAML file
    Add(AddRouteArgs),
    /// Export route by id
    Export { #[arg(long)] id: String, #[command(flatten)] server: ServerRef },
    /// Delete route by id
    Delete { #[arg(long)] id: String, #[command(flatten)] server: ServerRef },
    /// Execute route by id with JSON payload
    Exec(ExecRouteArgs),
    /// Show recent execution audit entries
    Audit(AuditArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct ServerRef {
    #[arg(long, env = "WORPEN_SERVER", default_value = "http://127.0.0.1:3000")]
    pub server: String,
    /// API token (also via WORPEN_TOKEN env). Sent as Authorization: Bearer.
    #[arg(long, env = "WORPEN_TOKEN")]
    pub token: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct AddRouteArgs {
    #[arg(long)]
    pub file: String,
    #[command(flatten)]
    pub server: ServerRef,
    /// Apply restrictive sandbox (recommended for AI-generated routes)
    #[arg(long, default_value_t = false)]
    pub locked: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct ExecRouteArgs {
    #[arg(long)]
    pub id: String,
    #[arg(long, default_value = "{}")]
    pub payload: String,
    #[command(flatten)]
    pub server: ServerRef,
}

#[derive(Parser, Debug, Clone)]
pub struct AuditArgs {
    #[arg(long, default_value_t = 50)]
    pub limit: usize,
    #[command(flatten)]
    pub server: ServerRef,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FuncCmd {
    List(ServerRef),
    Add(AddFuncArgs),
    Delete { #[arg(long)] name: String, #[command(flatten)] server: ServerRef },
}

#[derive(Parser, Debug, Clone)]
pub struct AddFuncArgs {
    #[arg(long)]
    pub file: String,
    #[command(flatten)]
    pub server: ServerRef,
}

#[derive(Parser, Debug, Clone)]
pub struct DoctorArgs {
    #[arg(long, env = "DATA_DIR", default_value = "auto")]
    pub data_dir: String,
}

/// Resolve the effective data dir with the same rule as the server:
/// explicit value wins, "auto" = ./data inside backend/, ./backend/data otherwise.
pub fn resolve_data_dir(requested: &str) -> String {
    if requested != "auto" {
        return requested.to_string();
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    if cwd.ends_with("backend") || cwd.join("crates").exists() {
        "data".to_string()
    } else {
        "backend/data".to_string()
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum McpCmd {
    /// Serve MCP over STDIO (for Claude Desktop / agents)
    ServeStdio,
    /// List MCP tools from a running server
    Tools(ServerRef),
    /// Call an MCP tool (route) by name
    Call(McpCallArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct McpCallArgs {
    #[arg(long)]
    pub tool: String,
    #[arg(long, default_value = "{}")]
    pub args: String,
    #[command(flatten)]
    pub server: ServerRef,
}

#[derive(Subcommand, Debug, Clone)]
pub enum PipeCmd {
    /// List pipelines with last-run status
    List(ServerRef),
    /// Define a pipeline from JSON/YAML file
    Define(AddPipeArgs),
    /// Run a pipeline by id or name
    Run(RunPipeArgs),
    /// Delete a pipeline definition
    Delete { #[arg(long)] id: String, #[command(flatten)] server: ServerRef },
    /// Show run history
    Runs(RunsArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct AddPipeArgs {
    #[arg(long)]
    pub file: String,
    #[command(flatten)]
    pub server: ServerRef,
}

#[derive(Parser, Debug, Clone)]
pub struct RunPipeArgs {
    /// Pipeline id or name
    #[arg(long)]
    pub name: String,
    #[arg(long, default_value = "{}")]
    pub payload: String,
    #[command(flatten)]
    pub server: ServerRef,
}

#[derive(Parser, Debug, Clone)]
pub struct RunsArgs {
    #[arg(long)]
    pub pipeline_id: Option<String>,
    #[arg(long, default_value_t = 20)]
    pub limit: usize,
    #[command(flatten)]
    pub server: ServerRef,
}

pub fn parse() -> Cli {
    Cli::parse()
}
