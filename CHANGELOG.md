# 📝 Changelog

## [Unreleased] - 2026-01-03

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
  - `WEBSOCKET_GUIDE.md` - Complete WebSocket documentation

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
- Added comprehensive [WEBSOCKET_GUIDE.md](WEBSOCKET_GUIDE.md) with:
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
  
- Updated [DYNAMIC_ROUTES_GUIDE.md](DYNAMIC_ROUTES_GUIDE.md):
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
