<div align="center">

```text
██╗    ██╗ ██████╗ ██████╗ ██████╗ ███████╗███╗   ██╗
██║    ██║██╔═══██╗██╔══██╗██╔══██╗██╔════╝████╗  ██║
██║ █╗ ██║██║   ██║██████╔╝██████╔╝█████╗  ██╔██╗ ██║
██║███╗██║██║   ██║██╔══██╗██╔═══╝ ██╔══╝  ██║╚██╗██║
╚███╔███╔╝╚██████╔╝██║  ██║██║     ███████╗██║ ╚████║
 ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═══╝
                                                       
    🔥 LOGIC-AS-DATA • ZERO-COST ABSTRACTIONS • RUST CORE 🔥
```

# ⚡ **W O R P E N** ⚡
### **The Dynamic Logic Engine** • *Powered by Rust* 🦀

[![Core](https://img.shields.io/badge/CORE-RUST_1.75+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Performance](https://img.shields.io/badge/BENCHMARK-~9k_REQ/S-green?style=for-the-badge&logo=speedtest)](https://github.com/matinsanei/worpen)
[![Tests](https://img.shields.io/badge/TESTS-262_PASSING-blue?style=for-the-badge&logo=github-actions)](backend/tests)
[![License](https://img.shields.io/badge/LICENSE-MIT-blue?style=for-the-badge&logo=open-source-initiative)](LICENSE)

<p align="center">
  <b>Dynamic Routes</b> • <b>Function Inlining</b> • <b>Hot-Swap Logic</b> • <b>Zero Downtime</b>
</p>

> *"The flexibility of Python, with the soul of Rust."*

[Quick Start](#-quick-start) • [Benchmarks](#-performance--benchmarks) • [Features](#-core-capabilities) • [Roadmap](ROADMAP.md)

</div>

---

## 🌌 **OVERVIEW**

**WORPEN** is a high-performance **Dynamic API Engine** that allows you to define backend logic using **JSON/YAML** instead of compiled code.

Unlike traditional interpreters, Worpen uses a **Zero-Cost Inlining Compiler**. When you define a function or logic flow, Worpen compiles it into a linear execution path in memory. This means you get the **flexibility of a dynamic language** without the runtime overhead of function calls or context switching.

### 🎯 **Why Worpen?**

- **🔥 Logic-as-Data**: Define business logic as data. Update rules instantly without recompiling.
- **🚀 Zero-Cost Functions**: Functions are "inlined" at registration time. Calling a function has **0ms runtime overhead**.
- **⚡ Rust Performance**: Built on `Axum` and `Tokio`. Handles **~9,000 to 20,000 req/s** on standard hardware.
- **🛡️ Type-Safe & Secure**: Variable scoping and memory management are handled by Rust's strict safety guarantees.
- **🔌 Service Mesh Ready**: Can orchestrate calls to other microservices directly from logic.

---

## 📊 **PERFORMANCE & BENCHMARKS**

We believe in honest engineering. Here is how Worpen compares to industry standards in a realistic **End-to-End Benchmark** (not just internal parsing).

**Test Environment:** Windows Localhost, 100k Requests, 125 Concurrency.

| Framework | Architecture | Throughput (Req/s) | Latency (Avg) | Analysis |
| :--- | :--- | :--- | :--- | :--- |
| **Go (Gin)** | Native Compiled | **~17,000** | ~7ms | The gold standard for raw static performance. |
| **Worpen (Rust)** | **Dynamic Engine** | **~9,000** | **~14ms** | **~50-60% the speed of Go**, but fully dynamic! |
| **Node.js (Fastify)** | JIT Compiled | ~3,500 | ~30ms | Worpen is **~2.5x faster**. |
| **Python (FastAPI)** | Interpreted | ~1,500 | ~60ms | Worpen is **~6x faster**. |

> **Conclusion:** Worpen creates a new category. It is significantly faster than dynamic runtimes (Node/Python) and approaches the performance of static binaries (Go), while offering "Hot-Swap" capabilities that static languages lack.

---

## 🚀 **QUICK START**

### 1. Run the Engine

```bash
git clone https://github.com/matinsanei/worpen.git
cd worpen/backend
cargo run --release
```
*Server starts on `127.0.0.1:3000`*

### 2. Define a Function (Global Logic)

Let's define a reusable logic block for calculating tax.

```bash
curl -X POST http://127.0.0.1:3000/api/v1/global-functions \
  -H "Content-Type: application/json" \
  -d '{
    "name": "add_tax",
    "params": ["price"],
    "logic": [
      {
        "math_op": { "operation": "multiply", "args": ["{{price}}", 1.09] }
      },
      {
        "set": { "var": "result", "value": "{{math_result}}" }
      }
    ]
  }'
```

### 3. Create a Route (The API)

Now create an API that uses this function. Worpen will **inline** the logic automatically.

```bash
curl -X POST http://127.0.0.1:3000/api/v1/dynamic-routes \
  -H "Content-Type: application/json" \
  -d '{
    "name": "checkout",
    "path": "/api/checkout",
    "method": "POST",
    "enabled": true,
    "logic": [
      { "set": { "var": "base", "value": 100 } },
      {
        "call_function": {
          "name": "add_tax",
          "args": ["{{base}}"],
          "output_var": "final_amount"
        }
      },
      {
        "return": { "value": { "total": "{{final_amount}}" } }
      }
    ]
  }'
```

### 4. Call It!

```bash
curl -X POST http://127.0.0.1:3000/api/checkout
# Output: {"total": "109.0"}
```

---

## 🧬 **CORE CAPABILITIES**

### 🔧 **Operations**

| Operation | Description | Example |
| :--- | :--- | :--- |
| `Set / Get` | Variable management | `var: "user_id", value: "123"` |
| `MathOp` | Arithmetic (+, -, *, /) | `sum(price, tax)` |
| `If / Switch` | Control Flow | `if total > 100 then discount` |
| `Loop / While` | Iteration | `foreach item in cart` |
| `CallFunction` | **Zero-Cost** calls | `call validate_user(token)` |
| `HttpRequest` | External API calls | `POST https://stripe.com/api` |
| `Parallel` | Async Execution | Run 5 DB queries at once |

### 🧠 **Advanced Architecture**

1.  **Inlining Compiler:** When a route is registered, Worpen recursively flattens all function calls into a single list of operations.
2.  **Variable Mangling:** To prevent conflicts, variables in functions are scoped (e.g., `_0_price`) automatically.
3.  **Hot-Route Caching:** Compiled execution plans are cached in memory. The second request to an API skips parsing entirely.

---

## 🤝 **CONTRIBUTING**

Worpen is an ambitious project to redefine how backends are built. We welcome contributions!

1.  **Fork** the repo.
2.  **Create** a branch (`feature/new-operation`).
3.  **Commit** & **Push**.
4.  **Open a PR**.

### Roadmap 🗺️
- [x] Core Engine & Logic Interpreter
- [x] Variable Scoping & Global Memory
- [x] Zero-Cost Function Inlining
- [x] HTTP Client & Orchestration
- [ ] Support for Embedded Scripting (Lua/JS)
- [ ] Database ORM Integration

---

## 📞 **CONTACT & AUTHOR**

**Matin Sanei**
- **GitHub:** [@matinsanei](https://github.com/matinsanei)
- **Email:** matinsanei@gmail.com

---

<div align="center">

**Built with 🦀 Rust • Designed for Speed • Engineered for Flexibility**

*"The best API is the one you don't have to code."*

</div>
