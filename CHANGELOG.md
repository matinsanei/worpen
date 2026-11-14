# 📝 Changelog

## [Unreleased] - 2026-11-14 (Enterprise Action Plane Loop)

### 🔒 Security - Sandbox & Allowlist
- **SEC-01 Fix:** Eliminated unsafe substring match (`h.contains(&a)`) in `SandboxPolicy::is_host_allowed`.
- Enforced strict exact domain or explicit subdomain suffix matching (`h == domain || h.ends_with(&format!(".{}", domain))`).
- Added regression unit tests against domain spoofing and substring attacks in `proto::models::routes`.

### 📚 Documentation & Audit
- Published `AUDIT.md`: Complete codebase and claim verification against actual implementation.
- Established `BACKLOG.md` ordered across 10 mission phases.
- Established `STATE.md` and `DECISIONS.md` (ADR-0001, ADR-0002, ADR-0003).
- Cleaned `README.md`: Removed unverified comparison bar charts and fabricated performance claims; aligned roadmap with enterprise action plane.

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
