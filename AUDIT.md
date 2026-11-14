# Worpen Codebase & Claims Audit

**Date:** 2026-11-14  
**Auditor:** Lead Systems Engineer  
**Branch:** `enterprise-loop` (commit: `8256525`, base: `v0.1.0` / `7f52b36`)

---

## 1. Executive Summary

Worpen aims to be the **action plane for AI agents**: a Rust runtime enabling AI agents to execute multi-step, deterministic, human-authored workflows through a single MCP tool call, protected by secret isolation, capability sandboxing, human approval gates, and tamper-evident audit logs.

The repository contains a functional integer-indexed bytecode VM, an expression template parser and compiler, YAML/JSON route loaders, a multi-stage pipeline executor, and a dual-transport (HTTP + STDIO) MCP server. 254 test cases pass across the workspace.

However, the audit identified:
1. **Critical Security Vulnerabilities in the Sandbox:** The host allowlist check relies on substring matching (`h.contains(&a)`), permitting trivial domain spoofing (`evil-stripe.com`). There is zero DNS resolution check or private/loopback/link-local IP validation (SSRF vulnerability). SQL execution lacks table allowlists, row limits, or enforced read-only modes.
2. **Fabricated / Unverified Benchmark Claims:** The README benchmark comparison chart (Worpen 38,405 req/s vs Go 17,800 vs Node 13,000 vs Python 3,900) has no benchmark harnesses, test data, or reproducible scripts for Go, Node, or Python anywhere in the repository.
3. **Pivotal Missing Enterprise Features:** Secrets referencing (`{{secret:name}}`) and automatic redaction do not exist. Human approval gates and durable pauses do not exist. The audit log is a plain unverified append-only file with no cryptographic chaining or verification.
4. **Significant Scope Creep & Legacy Dead Code:** Remnants of an older container/host agent monitor (Docker services, agent terminals, shell execution, incident management, WebSocket multiplexing) inflate the attack surface and confuse the core mission.

---

## 2. Claim-by-Claim Verification Matrix

| Claim | Source | Status | Evidence & Reality |
| :--- | :--- | :--- | :--- |
| **"TESTS: 250 PASSING"** | README Badge | **VERIFIED** | Running `cargo test` executes 254 tests across all crates (165 core tests, 83 integration tests, 6 doc tests), all passing. |
| **"Single binary release (worpen serve)"** | README | **VERIFIED** | Compiles to single `worpen` binary (`backend/crates/api/src/main.rs`) embedding API, CLI, and optionally the static UI. |
| **"MCP Server (HTTP + STDIO)"** | README | **VERIFIED (Basic)** | Implements JSON-RPC 2.0 `initialize`, `tools/list`, and `tools/call` over HTTP (`POST /mcp`) and STDIO (`worpen mcp serve-stdio`). |
| **"Capability-based Sandbox"** | README / Docs | **PARTIAL / CRITICAL HOLES** | `SandboxPolicy` exists, but has major security flaws: substring host matching, no SSRF/DNS-rebinding protection, unconstrained SQL queries. |
| **"Host allowlist (e.g. allowed_hosts: ['api.stripe.com'])"** | README | **FALSE (VULNERABLE)** | `is_host_allowed` in `proto/src/models/routes.rs` uses `h == a \|\| h.ends_with(&format!(".{}", a)) \|\| h.contains(&a)`. `h.contains(&a)` allows any host containing the allowed host string (e.g. `api.stripe.com.attacker.com`). No DNS resolution or IP range blocking. |
| **"38,405 req/s vs Go 17.8k vs Node 13k vs Python 3.9k"** | README | **FALSE (FABRICATED)** | No Go, Node, or Python benchmark scripts exist. No benchmark methodology or reproduction code exists in the repository. Must be removed immediately. |
| **"4 milliseconds execution"** | README | **UNVERIFIED** | While simple in-memory arithmetic executes in microseconds, database/network ops depend entirely on external latency. General claim is unproven without reproduction harness. |
| **"Secrets isolation ({{secret:name}})"** | North Star / Spec | **NON-EXISTENT** | Zero implementation for secret references, runtime injection, or redaction in logs/audit/errors. |
| **"Human approval gates"** | North Star / Spec | **NON-EXISTENT** | No `approval` operation, no paused execution state, no resume mechanism. |
| **"Tamper-evident audit trail"** | README / Spec | **FALSE / PARTIAL** | `audit.log` appends plain JSON lines without cryptographic hash chaining, signature verification, or per-step agent provenance. |
| **"WASM / ONNX / JIT Roadmap"** | README | **SCOPE CREEP** | Listed on roadmap, but distracts from the core mission and enterprise sandbox stability. Must be frozen/removed. |

---

## 3. Module Map & Architecture Assessment

### 3.1 `backend/crates/core` (Core Domain & VM)
- **Status:** Core engine functional; contains legacy baggage.
- **Key Files:**
  - `src/compiler/`: `lowerer.rs`, `symbol_table.rs` — compiles route logic into integer-indexed opcodes for the VM.
  - `src/vm/`: `machine.rs`, `memory.rs`, `instructions.rs` — stack/array-based execution loop.
  - `src/expression/`: tokenizer, AST parser, evaluator with pipe filters and math functions.
  - `src/services/dynamic_routes/`: service handling route registration, file persistence (`backend/data/routes`), execution, and in-memory audit ring.
  - `src/services/pipelines.rs`: multi-stage pipeline definition and runner.
- **Scope Creep / Dead Code to Freeze/Remove:**
  - `src/websocket/`: WebSocket manager and client tracking.
  - `src/services/docker.rs`: Docker container manager.
  - `src/services/incident.rs`: Incident tracking service.
  - `src/services/agent_service.rs` & `src/services/orchestrator.rs`: Host agent monitoring and process execution.
  - `src/services/automation.rs`: Automation rules engine (superseded by routes/pipelines).

### 3.2 `backend/crates/proto` (Data Transfer Objects & Contracts)
- **Status:** Holds schema models.
- **Scope Creep / Dead Code:**
  - `src/models/docker.rs`, `terminal.rs`, `incident.rs`, `agent.rs`, `automation.rs`.
  - `proto/v1/orchestrator.proto`.

### 3.3 `backend/crates/infra` (Persistence & Repositories)
- **Status:** Manages SQLite pool.
- **Scope Creep / Dead Code:**
  - `src/repositories/sqlite_incident.rs`, `sqlite_automation.rs`.
  - `src/adapters/sqlite_agent_repository.rs`, `in_memory_agent_repository.rs`.

### 3.4 `backend/crates/api` (HTTP, STDIO MCP, & CLI)
- **Status:** Core CLI commands (`serve`, `route`, `func`, `pipeline`, `mcp`, `doctor`, `setup`) work.
- **Critical Flaw in STDIO MCP:** `serve_stdio` initializes `DynamicRouteService` without a database pool (`db_pool = None`). Any route using `sql_op` immediately crashes in STDIO mode.
- **Scope Creep / Dead Code:**
  - Endpoint routes in `main.rs`: `/api/v1/agents/*`, `/api/v1/containers/*`, `/api/v1/docker/*`, `/api/v1/incidents/*`, `/api/v1/automation/*`, `/api/v1/terminal/*`, `/api/ws/*`.

### 3.5 Frontend
- **Status:** Vite + React + Tailwind frontend built into `dist/`.
- **Scope Creep:**
  - Builder visual editor (`components/RouteBuilder/`).
  - Terminal frames and live logs for agent daemons.

---

## 4. Security Vulnerability Details

### SEC-01: Insecure Host Allowlist Bypass (CRITICAL)
- **Location:** `backend/crates/proto/src/models/routes.rs:250`
- **Code:** `h == a || h.ends_with(&format!(".{}", a)) || h.contains(&a)`
- **Vulnerability:** If `allowed_hosts` specifies `api.stripe.com`, an attacker can route requests to `api.stripe.com.evil.com` or `evil-api.stripe.com` because `h.contains(&a)` evaluates to `true`.
- **Remediation:** Remove `contains`. Only allow exact host matches or explicit subdomains (`ends_with(".stripe.com")`).

### SEC-02: SSRF & DNS Rebinding Vulnerability (HIGH)
- **Location:** `backend/crates/core/src/services/dynamic_routes/io.rs:28` & `backend/crates/core/src/vm/machine.rs:121`
- **Vulnerability:** Host validation only parses the URL string. No DNS lookup is performed prior to the request. The HTTP client follows redirects and resolves DNS dynamically, allowing attackers to target internal resources (`127.0.0.1`, `169.254.169.254`, `10.0.0.0/8`) via DNS rebinding or redirect hops.
- **Remediation:** Implement strict DNS pre-resolution, ban loopback, private, link-local, and multicast ranges, pin the resolved IP, and disable automatic redirects to unvetted domains.

### SEC-03: Unrestricted SQL Query Execution (HIGH)
- **Location:** `backend/crates/core/src/vm/machine.rs:245-263`
- **Vulnerability:** Any valid SQL query is passed directly to SQLite. No read-only validation, no table allowlists, and no query result row caps are enforced. If an agent executes `DROP TABLE` or reads arbitrary system tables, it succeeds if `allow_sql: true`.
- **Remediation:** Add SQL inspection / policy: restrict to parameterized statements, enforce read-only execution modes when configured, allow table whitelisting, and cap output row counts.

### SEC-04: Credential & Secret Exposure in Execution Logs (HIGH)
- **Location:** `backend/crates/core/src/services/dynamic_routes/service.rs:170-204`
- **Vulnerability:** Audit logs, error messages, and VM execution variables do not mask or redact secrets. Any API tokens or credentials placed in input payloads are logged in plain text to `backend/data/audit.log`.
- **Remediation:** Introduce secret reference indirection `{{secret:name}}`, environment/keychain secret providers, and comprehensive output sanitization/redaction.

---

## 5. Next Steps for Phase 1
1. Remove fabricated performance claims and unverified comparisons from `README.md`.
2. Freeze and remove dead routes/modules (Docker, agent monitoring, terminals, WebSockets) from the default build.
3. Establish `BACKLOG.md`, `STATE.md`, and `DECISIONS.md`.
