<div align="center">

```text
██╗    ██╗ ██████╗ ██████╗ ██████╗ ███████╗███╗   ██╗
██║    ██║██╔═══██╗██╔══██╗██╔══██╗██╔════╝████╗  ██║
██║ █╗ ██║██║   ██║██████╔╝██████╔╝█████╗  ██╔██╗ ██║
██║███╗██║██║   ██║██╔══██╗██╔═══╝ ██╔══╝  ██║╚██╗██║
╚███╔███╔╝╚██████╔╝██║  ██║██║     ███████╗██║ ╚████║
 ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═══╝
                                                       
    🔥 LOGIC-AS-DATA • ZERO-COST INLINING • BYTECODE VM 🔥
```

# ⚡ **W O R P E N** ⚡
### **ᴛʜᴇ ᴅʏɴᴀᴍɪᴄ ᴀᴘɪ ᴇɴɢɪɴᴇ** • *ᴘᴏᴡᴇʀᴇᴅ ʙʏ ʀᴜsᴛ* 🦀

[![Core](https://img.shields.io/badge/CORE-RUST_1.80+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Performance](https://img.shields.io/badge/SPEED-38k%2B_REQ/S-green?style=for-the-badge&logo=speedtest)](https://github.com/matinsanei/worpen)
[![Tests](https://img.shields.io/badge/TESTS-255_PASSING-blue?style=for-the-badge&logo=github-actions)](backend/tests)
[![License](https://img.shields.io/badge/LICENSE-MIT-blue?style=for-the-badge&logo=open-source-initiative)](LICENSE)
[![YAML](https://img.shields.io/badge/YAML-SUPPORTED-blue?style=for-the-badge&logo=yaml)](documentation/13-yaml-syntax.md)
[![WebSocket](https://img.shields.io/badge/WEBSOCKET-SUPPORTED-green?style=for-the-badge&logo=socket.io)](WEBSOCKET_GUIDE.md)

<p align="center">
  <b>Logic-as-Data</b> • <b>Bytecode VM</b> • <b>Zero-Cost Inlining</b> • <b>Hot-Swap</b> • <b>WebSocket</b>
</p>

> *"Forget about middleware. Define your business logic as data, and let the Worpen VM execute it at metal speed."*

[Quick Start](#-quick-start) • [Benchmarks](#-performance--benchmarks) • [Architecture](#-architecture-the-internal-magic) • [Roadmap](ROADMAP.md)

</div>

---

## 🌌 𝐎𝐕𝐄𝐑𝐕𝐈𝐄𝐖

**WORPEN** is a high-performance **Dynamic API Engine** that allows you to build complex backends using **YAML or JSON** without writing a single line of compiled code. 

Unlike traditional interpreters that are slow and memory-heavy, Worpen compiles your logic into a specialized **Bytecode** and executes it via an **Integer-based Virtual Machine**. This gives you the **flexibility of Python** (Hot-swapping logic on the fly) with the **soul of Rust** (safety and raw speed).

### 🎯 What Makes Worpen Special?

- **🔥 Logic-as-Data**: Business rules are stored as data, not code. Change logic in production with zero downtime.
- **🚀 Zero-Cost Inlining**: Functions are recursively flattened into the main execution path at load-time. Call overhead is **0ns**.
- **🧠 Integer-based VM**: No HashMap lookups at runtime. All variables are mapped to direct memory indices (`O(1)` access).
- **⚡ Async Persistence**: Routes are compiled once and cached in memory. Changes are persisted to disk asynchronously without blocking the hot path.
- **🔌 Service Mesh Ready**: Native HTTP orchestration to call and pipe other microservices.
- **🌐 WebSocket Support**: Real-time bidirectional communication with event-driven hooks and channel broadcasting.

---

## 📊 𝐏𝐄𝐑𝐅𝐎𝐑𝐌𝐀𝐍𝐂𝐄 & 𝐁𝐄𝐍𝐂𝐇𝐌𝐀𝐑𝐊𝐒

We believe in honest, end-to-end benchmarking. Worpen bridges the gap between dynamic languages and static binaries.

**Test Environment:** Windows (Localhost), 100k Requests, 125 Concurrency.

| Framework | Architecture | Throughput (Req/s) | Latency (Avg) | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Worpen (Rust VM)** | **Dynamic Logic Engine** | **~38,405** 🚀 | **~4.1ms** | **The New King 👑** |
| **Go (Gin)** | Native Static Compiled | ~17,800 | ~7.3ms | 1.7x Slower |
| **Node.js (Fastify)** | JIT Compiled (V8) | ~13,000 | ~9.8ms | 2.3x Slower |
| **Python (FastAPI)** | Interpreted | ~3,900 | ~32.0ms | **7.7x Slower** |

```text
Performance Visualization:
WORPEN   ████████████████████████████████ 38,405 (100%)
GO (GIN) ███████████████████░░░░░░░░░░░░░░░░░░ 17,800 (46%)
NODE.JS  ███████████████░░░░░░░░░░░░░░░░░░░░░ 13,000 (34%)
PYTHON   ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3,900  (10%)
```

---

## 🏗️ 𝐀𝐑𝐂𝐇𝐈𝐓𝐄𝐂𝐓𝐔𝐑𝐄: 𝐓𝐇𝐄 𝐈𝐍𝐓𝐄𝐑𝐍𝐀𝐋 𝐌𝐀𝐆𝐈𝐂

Worpen functions as a **Runtime Compiler**, not just a simple JSON parser.

1.  **Lowering Phase:** Upon route registration, the `LogicCompiler` transforms human-readable strings (`"user_price"`) into a **Symbol Table** of integers (`index 4`).
2.  **Inlining Phase:** All `CallFunction` operations are recursively injected into the parent logic, eliminating the need for a Stack Frame or Context Switch during execution.
3.  **VM Execution:** The `VirtualMachine` iterates over a flat `Vec<OptimizedOperation>`. It accesses variables via **Direct Array Indexing**, which is highly friendly to CPU L1/L2 caches.

---

## 🚀 𝐐𝐔𝐈𝐂𝐊 𝐒𝐓𝐀𝐑𝐓

### 1. Run the Server
```bash
git clone https://github.com/matinsanei/worpen.git
cd worpen/backend
cargo run --release
```

### 2. Define a Global Function (Reusable Logic)
```bash
curl -X POST http://127.0.0.1:3000/api/v1/global-functions \
  -H "Content-Type: application/json" \
  -d '{
    "name": "calculate_tax",
    "params": ["price"],
    "logic": [
      { "math_op": { "operation": "sum", "args": ["{{price}}", 10] } },
      { "set": { "var": "result", "value": "{{math_result}}" } }
    ]
  }'
```

### 3. Create a Route (The API)
Worpen will **inline** the function logic at this stage.
```bash
curl -X POST http://127.0.0.1:3000/api/v1/dynamic-routes \
  -H "Content-Type: application/json" \
  -d '{
    "name": "checkout_api",
    "path": "/api/checkout",
    "method": "POST",
    "logic": [
      { "set": { "var": "base", "value": 50 } },
      { "call_function": { "name": "calculate_tax", "args": ["{{base}}"], "output_var": "final" } },
      { "return": { "value": "{{final}}" } }
    ]
  }'
```

### 4. Call your API
```bash
curl -X POST http://127.0.0.1:3000/api/checkout
# Response: 60.0
```

---

## 🧬 𝐂𝐎𝐑𝐄 𝐂𝐀𝐏𝐀𝐁𝐈𝐋𝐈𝐓𝐈𝐄𝐒

| Category | Operations | Description |
|:---------|:-----------|:------------|
| **Memory** | `Set`, `Get` | **O(1) Access** via Integer Indices |
| **Logic** | `If`, `Switch`, `Loop` | Full control flow support |
| **Math** | `sum`, `mul`, `div`, etc. | Blazing fast binary operations |
| **Database** | `SqlOp` ⚡ | VM-optimized parameterized SQL queries |
| **Caching** | `RedisOp` ⚡ | High-speed Redis operations with pooling |
| **Networking**| `HttpRequest` | Built-in async HTTP client for Service Mesh |
| **Documentation** | `Comment` 📝 NEW | Add inline documentation to your routes |
| **Optimization**| `Inlining` | Compile-time function flattening |

### 💾 SQL Operations Example
```yaml
logic:
  - set:
      var: user_id
      value: 123
  - sql_op:
      query: "SELECT * FROM users WHERE id = ?"
      args: ["{{user_id}}"]
      output_var: user_data
```

### ⚡ Redis Caching Example (NEW)
```yaml
logic:
  - set:
      var: user_id
      value: 123
  - redis_op:
      command: GET
      key: "user:{{user_id}}:profile"
      output_var: cached_profile
  - if:
      condition: "{{cached_profile}} == null"
      then:
        - sql_op:
            query: "SELECT * FROM users WHERE id = ?"
            args: ["{{user_id}}"]
            output_var: user_data
        - redis_op:
            command: SET
            key: "user:{{user_id}}:profile"
            value: "{{json_stringify(user_data)}}"
            ttl_seconds: 300
            output_var: cache_status
        - return:
            value: "{{user_data}}"
      otherwise:
        - return:
            value: "{{cached_profile}}"
```

**Features:**
- ✅ Parameterized queries (SQL injection safe)
- ✅ Variable interpolation from execution context
- ✅ Full VM optimization (3-5µs overhead)
- ✅ Direct memory indexing (O(1) access)

---

## � 𝐂𝐎𝐌𝐌𝐄𝐍𝐓𝐒 𝐀𝐍𝐃 𝐃𝐎𝐂𝐔𝐌𝐄𝐍𝐓𝐀𝐓𝐈𝐎𝐍

Worpen now supports **inline comments** in your route definitions. Comments are treated as **no-op operations** during VM execution, making them perfect for documenting complex logic flows.

### Using Comments in JSON
```json
{
  "name": "documented_route",
  "path": "/api/order",
  "method": "POST",
  "logic": [
    {
      "comment": {
        "text": "--- Initialize variables ---"
      }
    },
    {
      "set": {
        "var": "base_price",
        "value": 100
      }
    },
    {
      "comment": {
        "text": "--- Calculate tax (10%) ---"
      }
    },
    {
      "math_op": {
        "operation": "mul",
        "args": ["{{base_price}}", 1.1]
      }
    },
    {
      "return": {
        "value": "{{math_result}}"
      }
    }
  ]
}
```

### Using Comments in YAML
```yaml
name: documented_route
path: /api/order
method: POST
logic:
  - comment:
      text: "--- Initialize variables ---"
  - set:
      var: base_price
      value: 100
  - comment:
      text: "--- Calculate tax (10%) ---"
  - math_op:
      operation: mul
      args:
        - "{{base_price}}"
        - 1.1
  - return:
      value: "{{math_result}}"
```

**Benefits:**
- ✅ Zero runtime overhead (comments are skipped during execution)
- ✅ Better code maintainability and team collaboration
- ✅ Support for Persian/Arabic and all UTF-8 characters
- ✅ Works in both JSON and YAML formats

---

## �📚 𝐃𝐎𝐂𝐔𝐌𝐄𝐍𝐓𝐀𝐓𝐈𝐎𝐍

- 📖 **[Introduction](documentation/01-introduction.md)**
- 🏗️ **[Variable Scoping & VM](documentation/03-variables-basics.md)**
- 🔁 **[Advanced Loops & Logic](documentation/05-loops.md)**
- 📞 **[Zero-Cost Functions](documentation/06-functions.md)**
- 🔌 **[WebSocket Support](WEBSOCKET_GUIDE.md)** ⭐ NEW
  - 📋 **[Quick Reference](WEBSOCKET_QUICKREF.md)**
- 🔄 **[Migration from JSON to YAML](documentation/15-migration-guide.md)**
- ✨ **[Performance Best Practices](documentation/16-best-practices.md)**
- 📝 **[Changelog](CHANGELOG.md)**

---

## 🎯 𝐂𝐔𝐑𝐑𝐄𝐍𝐓 𝐒𝐓𝐀𝐓𝐔𝐒
6)
- **Phase 1-3:** Core Engine, Operations & Persistence.
- **Phase 4:** **Global Functions & Recursive Inlining.**
- **Phase 5:** **Bytecode VM & Symbol Table Architecture.** (Speed: 38k+ Req/s)
- **Phase 6:** **WebSocket Support with Event Hooks.** ⭐ NEW
- **Phase 5:** **Bytecode VM & Symbol Table Architecture.** (Current Speed: 30k+ Req/s)

### 🚀 Roadmap
- [ ] **AI Inference Node:** Direct execution of ONNX models within the VM.
- [ ] **JIT Native Compilation:** Using `Cranelift` to compile Bytecode to x86 Assembly.
- [ ] **WASM Logic Blocks:** Import logic from any language (Go, C++, Rust).
- [ ] **Visual Logic Editor:** Drag-and-drop IDE for Worpen.

---

## 🤝 𝐂𝐎𝐍𝐓𝐑𝐈𝐁𝐔𝐓𝐈𝐍𝐆

Worpen is an ambitious project to redefine backend infrastructure. We welcome performance hackers, compiler designers, and Rust enthusiasts.
55
1.  **Fork** & **Clone**.
2.  `cargo test` (We maintain 262+ tests guarding the VM).
3.  Submit a PR with performance benchmarks.

---

## 👨‍💻 𝐀𝐔𝐓𝐇𝐎𝐑

**Matin Sanei**
- **GitHub:** [@matinsanei](https://github.com/matinsanei)
- **Email:** saneimatin33@gmail.com

*"The best code is the one you never compile."*

**MIT Licensed** • Copyright © 2025 Worpen Engine

</div>