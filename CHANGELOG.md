# 📝 Changelog

## [Unreleased] - 2026-11-14 (Enterprise Action Plane Loop)

### 🛡️ Security Hardening & Threat Modeling (Phase 9)
- **Comprehensive STRIDE Threat Model:**
  - Published `THREAT_MODEL.md` analyzing all 5 runtime trust boundaries, protect surfaces (credentials, enterprise databases, internal networks, execution state, audit trails), and mitigations.
- **Truthful Vulnerability Disclosure Policy:**
  - Rewrote `SECURITY.md` defining supported versions (`0.1.x`), eliminating fictional bounty programs and placeholder keys, establishing clear in-scope and out-of-scope boundaries, and detailing a 24h/72h/14d coordinated disclosure SLA.
- **Adversarial Security Test Suite:**
  - Created `backend/crates/core/tests/adversarial_security_test.rs` validating protections against loopback/0.0.0.0, cloud metadata, RFC 1918, CGNAT, IPv4-mapped IPv6, domain spoofing attacks, destructive SQL statements & comments, path traversal in workflow names, and unbounded loop attacks.

### 🛠️ Developer Experience & Tooling (Phase 8)
- **`worpen init` Project Scaffolding:**
  - Added CLI scaffolding command creating starter workflows (`refund.yaml` with approval gate, `db_report.yaml` with read-only SQL, `issue_triage.yaml` with SSRF policy), `.cursor/mcp.json`, `.env.example`, and `worpen.yaml`.
- **`worpen doctor` System Diagnostics:**
  - Added comprehensive health inspection checking filesystem permissions, database connectivity, audit chain status, SSRF sandbox defenses, secrets sources, and auth state.
- **CLI Workflow & Audit Commands:**
  - Added `worpen workflow register/run/pending/approve/reject` for headless CLI workflow management.
  - Added `worpen audit verify` and `worpen audit export` for cryptographic audit integrity checking and SIEM log export directly from CLI.
- **Hardened Multi-Stage Dockerfile:**
  - Created minimal multi-stage Docker build with non-root user (`UID 10001`), state and secret volume mounts, and built-in healthchecks.
- **CLI Test Suite:**
  - Created `backend/crates/api/tests/cli_test.rs` validating project scaffolding, starter template syntax, doctor checks, and local/remote audit verification.

### 🤖 MCP Quality & Agent Tooling (Phase 7)
- **Standard Tool Annotations:**
  - Added `readOnly`, `destructive`, and `idempotent` hints across dynamic routes, pipelines, workflows, and built-in management tools.
- **Identity & Role-Based Filtering:**
  - Dynamic tool filtering in `tools/list` based on caller role (`X-Agent-Role`, e.g. `read_only_analyst` filtering out mutating tools) and token authentication.
- **Built-in Workflow & Approval Management Tools:**
  - Added `get_run`, `list_pending_approvals`, `approve_run`, `reject_run`, and `verify_audit` to MCP server for both HTTP and STDIO modes.
- **Structured Agent-Friendly Errors:**
  - Standardized error envelopes with `{ code, message, hint, retryable, fields }` returned in both text payloads and typed `structuredError` objects.
- **Ready-to-Use Client Configurations:**
  - Added exportable client configs for Claude Desktop (`clients/claude_desktop_config.json`), Cursor (`clients/cursor_mcp.json`, `.cursor/mcp.json`), and VS Code (`clients/vscode_mcp.json`).
- **MCP Test Suite:**
  - Created `backend/crates/api/tests/mcp_test.rs` validating initialize/ping, annotations, identity filtering, structured errors, and an end-to-end workflow pause, pending approval inspection, approval resume, and audit verification cycle.

### 🔗 Tamper-Evident Audit & Verification (Phase 6)
- **Hash-Chained Audit Trail (`worpen-core::audit::chain`):**
  - Cryptographically chained audit logging where each record computes a SHA-256 hash over canonical fields: `index`, `timestamp`, `prev_hash`, `action`, `actor`, `route_or_workflow`, `details`, and `policy_decision`.
  - Links to a canonical genesis hash (`0000000000000000000000000000000000000000000000000000000000000000`).
  - Implemented `verify_audit_chain` engine detecting any payload mutation, hash pointer breaks, or index deletions, returning the exact broken index.
- **Durable Engine & Dynamic Route Integration:**
  - Automatically records every workflow lifecycle step (start, approval request, execution failure, workflow completion, approval, rejection, cancellation) into the tamper-evident chain.
  - Automatically records all dynamic route executions and policy decisions.
- **Standard SIEM Exporters (`worpen-core::audit::export`):**
  - Export to JSON Lines (`export_jsonl`), Common Event Format (`export_cef`), and OpenTelemetry LogRecords (`export_otel`).
- **Audit REST Endpoints:**
  - Added `GET /api/v1/audit/verify` returning cryptographic verification reports.
  - Added `GET /api/v1/audit/export?format=jsonl|cef|otel` for SIEM ingestion.
- **Audit Tamper Test Suite:**
  - Created `backend/crates/core/tests/audit_tamper_test.rs` validating clean chain verification, detection of tampered details, broken hash pointers, index gaps, disk persistence across restarts, and CEF/OTel export outputs.

### 🔒 Security - Sandbox & Allowlist
- **SEC-01 Fix:** Eliminated unsafe substring match (`h.contains(&a)`) in `SandboxPolicy::is_host_allowed`.
- Enforced strict exact domain or explicit subdomain suffix matching (`h == domain || h.ends_with(&format!(".{}", domain))`).
- Added regression unit tests against domain spoofing and substring attacks in `proto::models::routes`.

### 📚 Documentation & Audit
- Published `AUDIT.md`: Complete codebase and claim verification against actual implementation.
- Established `BACKLOG.md` ordered across 10 mission phases.
- Established `STATE.md` and `DECISIONS.md` (ADR-0001, ADR-0002, ADR-0003).
- Cleaned `README.md`: Removed unverified comparison bar charts and fabricated performance claims; aligned roadmap with enterprise action plane.

### ⏸️ Approval Gates & Durable Runs (Phase 5)
- **Durable Runs State Machine (`worpen-core::workflow::durable`):**
  - Introduced `WorkflowRun` and `RunStatus` (`Pending`, `Running`, `WaitingForApproval`, `Completed`, `Failed`, `Rejected`, `Cancelled`, `Expired`).
  - Persisted runs in SQLite `workflow_runs` table with run metadata, step index, variables JSON, pending approval, and results.
- **Human Approval Gates (`WorkflowOpV1::Approval`):**
  - Pauses execution and transitions run to `WaitingForApproval` with human-readable summary and configurable expiration timeout (`expires_at`).
  - Implemented `approve_run`: verifies decision window, injects `__approved_by` and `__approval_reason` into variables, and resumes execution to subsequent steps.
  - Implemented `reject_run` and `cancel_run` for human or administrative termination.
- **Crash/Restart Recovery:**
  - Preserves run state across process crashes and restarts, allowing operators to list pending approvals and resume execution seamlessly.
- **Durable REST Endpoints:**
  - Added `/api/v1/runs`, `/api/v1/runs/pending`, `/api/v1/runs/:id`, `/approve`, `/reject`, `/cancel`.
- **Durable Test Suite:**
  - Created `backend/crates/core/tests/approval_durable_test.rs` covering pause-and-approve cycles, rejection, and recovery across process restarts.

### 🛡️ Policy & Sandbox (Phase 4)
- **SSRF Defenses & IP Pinning (`worpen-core::sandbox::ssrf`):**
  - Implemented IP range blocklist covering RFC 1918 private IPs, loopback (`127.0.0.0/8`, `::1`), link-local (`169.254.0.0/16`, `fe80::/10`), CGNAT (`100.64.0.0/10`), broadcast, multicast, and IPv4-mapped IPv6.
  - Implemented asynchronous DNS resolution with IP validation before socket creation.
  - Pinned outbound HTTP requests to the validated IP on `reqwest::ClientBuilder::resolve` to defeat DNS rebinding attacks.
  - Configured safe HTTP redirect policy that validates every redirect target against host allowlist and blocked IPs.
- **SQL Sandboxing (`worpen-core::sandbox::sql`):**
  - Added `read_only_sql` policy enforcement rejecting destructive DDL/DML statements (`INSERT`, `UPDATE`, `DELETE`, `DROP`, `ALTER`, `CREATE`, `TRUNCATE`, `REPLACE`).
  - Added `allowed_tables` enforcement parsing and validating table access in SQL queries.
  - Added `max_sql_rows` capping result sets to prevent memory exhaustion DoS.
- **Ready-Made Policy Packs:**
  - Added `SandboxPolicy::read_only_analyst()`, `SandboxPolicy::billing_operator()`, `SandboxPolicy::support_agent()`.
- **Policy Test Suite:**
  - Created `backend/crates/core/tests/policy_sandbox_test.rs` validating SSRF blocking, DNS resolution guards, SQL restrictions, and policy packs.

### 🔑 Secrets Isolation & Redaction (Phase 3)
- **Secret Architecture (`worpen-core::secrets`):**
  - References strictly use `{{secret:name}}` format, never exposed in agent prompts or accessible via runtime memory inspection.
  - Multi-source `SecretStore` supporting environment variables (`WORPEN_SECRET_*`), files (e.g. Kubernetes/Docker secrets), in-memory vaults, and OS keychain lookups.
  - Variable lowerer avoids registering `secret:*` tokens into symbol table or execution memory.
- **Late-Binding Injection:**
  - Injected at the final execution boundary into outbound HTTP request headers/URLs and parameterized SQL query arguments.
- **Automated Redaction Engine:**
  - `RedactionEngine` registers all resolved secret values and automatically sanitizes them to `[REDACTED]` across output values, error strings, and audit logs.
- **Canary Secrets Test Suite:**
  - Created `backend/crates/core/tests/canary_secrets_test.rs` asserting zero secret leakage across SQL, HTTP reflection, error responses, in-memory audit rings, and `audit.log` disk persistence.

### 📜 Workflow Spec v1 (Phase 2)
- **Implemented WorkflowSpecV1:** Defined strict YAML specification with `version`, `name`, `description`, `input_schema` (JSON Schema), `policy`, `steps`, and structured `return`.
- **Registration Validation:** Enforced strict variable scope checking (rejection of undefined variables), mandatory loop iteration bounds (`max_iterations > 0` and bounded by policy), and JSON Schema verification.
- **Side-Effect Protections & Idempotency:** Added support for per-step `timeout_ms`, retry with backoff, and automatic `Idempotency-Key` injection on HTTP steps.
- **Dry-Run Simulation:** Introduced `ExecutionMode::DryRun` allowing agents to simulate workflow execution without executing real side-effects (mocked SQL, HTTP, and approvals labeled with `_worpen_dry_run: true`).
- **Immutable Versioning & Rollback:** Added versioned file persistence (`<data_dir>/workflows/<name>/v<version>.yaml`), listing historical versions, and rollback capability via `WorkflowService`.
- **Integrated Endpoints:** Added `/api/v1/workflows` REST API endpoints (`list`, `register`, `get`, `execute`, `versions`, `rollback`) synchronized with the integer-indexed VM and MCP server.

### 🎯 Focus & Architecture (Phase 1)
- **Frozen Legacy Daemon Endpoints:** Deactivated unmaintained Docker container, agent monitoring, terminal websockets, and dynamic WebSocket endpoints from the active server router in `api/src/main.rs`.
- **Database Pool Integration in STDIO MCP & Serve:** Fixed missing database pool injection in `DynamicRouteService`, allowing SQL operations (`sql_op`) to execute deterministically in both HTTP server and STDIO MCP mode.
- **Direct VM Error Reporting:** Removed silent fallback from VM execution errors to legacy interpreter, ensuring real operational and SQL errors are accurately surfaced.
- **Added Integration Test:** Added `test_dynamic_route_service_with_sql` verifying end-to-end route execution with SQLite querying.

### 🧹 Code Quality
- Formatted entire Rust codebase across `core`, `proto`, `infra`, and `api` using `cargo fmt`.


### ✨ Added - WebSocket Support (Phase 6)
- **Real-time WebSocket support** with event-driven architecture
  - `on_connect`, `on_message`, `on_disconnect` hooks
  - VM-powered logic execution in WebSocket handlers
  - Automatic variable injection (`{{message}}`, `{{connection_id}}`)
  - Connection management with DashMap (thread-safe, lock-free)
  - Channel-based broadcasting support
  - `WsOp` operation with `send` and `broadcast` commands
  
- **New Files:**
  - `backend/crates/core/src/websocket/manager.rs` - WebSocket connection manager
  - `backend/crates/core/src/websocket/mod.rs` - Module exports
  - `backend/crates/api/src/handlers/dynamic_ws.rs` - WebSocket route handler
  - `backend/crates/core/tests/vm_websocket_test.rs` - 5 comprehensive tests
  - `documentation/WEBSOCKET_GUIDE.md` - Complete WebSocket documentation

- **Modified Files:**
  - `backend/crates/proto/src/models/routes.rs` - Added RouteType enum and WebSocketHooks
  - `backend/crates/core/src/vm/instructions.rs` - Added WsOp to OptimizedOperation
  - `backend/crates/core/src/compiler/lowerer.rs` - WsOp compilation logic
  - `backend/crates/core/src/vm/machine.rs` - WsOp execution with variable scoping
  - `backend/crates/api/src/main.rs` - WebSocket route registration
  
- **Dependencies:**
  - `tokio-tungstenite = "0.24"` - WebSocket protocol implementation
  - `dashmap = "6.1"` - Concurrent connection registry

### 🔧 Changed - Crate Renaming
- **Renamed crate `core` → `worpen-core`** to avoid collision with `std::core`
  - Fixed tokio::test macro expansion issues
  - Updated all imports from `use core::` to `use worpen_core::`
  - Updated `Cargo.toml` in api and infra crates
  - Fixed all doctests to use new crate name

### 📚 Documentation
- Added comprehensive [WEBSOCKET_GUIDE.md](documentation/WEBSOCKET_GUIDE.md) with:
  - Event hook examples (on_connect, on_message, on_disconnect)
  - WsOp command reference (send, broadcast)
  - Complete examples (echo server, chat room, game server)
  - Client connection examples (JavaScript, Rust, Python)
  - Architecture overview and best practices
  
- Updated [README.md](README.md):
  - Added WebSocket badge and feature mention
  - Updated test count (255 passing)
  - Updated performance metrics (38k+ req/s)
  - Added Phase 6 completion status
  
- Updated [DYNAMIC_ROUTES_GUIDE.md](documentation/DYNAMIC_ROUTES_GUIDE.md):
  - Marked WebSocket support as completed
  - Added link to WebSocket guide

- Updated [backend/README.md](backend/README.md):
  - Added note about crate renaming

### ✅ Tests
- **All 255 tests passing** (249 unit tests + 6 doctests)
- 5 new WebSocket-specific tests:
  - `test_websocket_send_operation`
  - `test_websocket_broadcast_operation`
  - `test_websocket_variable_interpolation`
  - `test_websocket_channel_broadcast`
  - `test_websocket_route_with_hooks`

### 🐛 Fixed
- Resolved 15+ compilation errors related to crate naming collision
- Fixed ExecutionMemory::new() signature mismatches in test files
- Updated all test imports to use `worpen_core`
- Fixed all doctests with incorrect crate imports

---

## [Previous Releases]

### Phase 5 - Bytecode VM & Symbol Table
- Integer-based VM with O(1) variable access
- Symbol table for compile-time optimization
- Zero-cost function inlining
- Performance: 38k+ req/s

### Phase 4 - Global Functions
- Reusable function definitions
- Recursive function inlining
- Parameter passing and scoping

### Phase 3 - Expression Engine
- Custom expression parser with 40+ helper functions
- Advanced loop constructs (foreach, while, until, range)
- SQL and Redis operations with named parameters
- Pipe filters and ternary operators

### Phase 2 - YAML Support
- YAML and JSON format detection
- Expression templates with `{{ }}` syntax
- 50-70% reduction in code verbosity

### Phase 1 - Dynamic Routes
- Runtime route registration
- VM-based logic execution
- HTTP/SQL/Redis operations
- Conditional logic and variables

---

## 🎯 Next Up

### Planned Features
- [ ] WebSocket authentication/authorization hooks
- [ ] Rate limiting per connection
- [ ] Message queue integration (Redis Pub/Sub)
- [ ] WebSocket metrics and monitoring
- [ ] Binary message support with custom codecs
- [ ] Compression (permessage-deflate)
- [ ] JIT compilation to native code (Cranelift)
- [ ] WASM logic blocks
- [ ] AI inference nodes (ONNX)
- [ ] Visual logic editor

---

**Format:** This changelog follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) principles.
