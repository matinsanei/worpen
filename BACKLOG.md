# Backlog

Ordered strictly by the 10 Phases of the Worpen Action Plane Mission.

---

## Phase 0 — Audit & Groundwork (COMPLETED)
- [x] **TASK-0.1:** Codebase inspection & claim verification matrix (`AUDIT.md`).
- [x] **TASK-0.2:** Remove fabricated benchmark charts and unverified claims from `README.md`.
- [x] **TASK-0.3:** Establish `STATE.md`, `BACKLOG.md`, `DECISIONS.md`.
- [x] **TASK-0.4:** Fix critical substring match vulnerability in `is_host_allowed`.

---

## Phase 1 — Focus (COMPLETED)
*Goal: Freeze/remove non-core features from default build (WebSocket, ONNX, WASM, JIT, container/agent daemon, builder UI). Keep only: workflow engine, MCP server (STDIO + HTTP), CLI, minimal run/approval/audit UI.*
- [x] **TASK-1.1:** Deprecate/freeze legacy endpoints in `api/src/main.rs` (`/api/v1/containers/*`, `/api/v1/agents/*`, `/api/v1/terminal/*`, `/api/v1/docker/*`, `/api/v1/incidents/*`).
- [x] **TASK-1.2:** Freeze WebSocket transport (`/api/ws/*`, `ws_handler`, `dynamic_ws_handler`) and remove unused WebSocket VM opcodes or mark dormant.
- [x] **TASK-1.3:** Remove/freeze unmaintained roadmap items (WASM logic blocks, ONNX nodes, Cranelift JIT) from documentation.
- [x] **TASK-1.4:** Pass database pool to `mcp::serve_stdio` so SQL operations work out-of-the-box in STDIO mode without crashing.
- [x] **TASK-1.5:** Ensure `cargo clippy --all-targets -- -D warnings` and `cargo test` pass cleanly.

---

## Phase 2 — Workflow Spec v1 (COMPLETED)
*Goal: Strict YAML schema, validation at registration, immutable versions + rollback, dry-run mode, core ops with idempotency and retries.*
- [x] **TASK-2.1:** Implement Workflow Spec v1 strict schema parser & validator (name, description, input JSON-schema, policy, steps, return).
- [x] **TASK-2.2:** Validation at registration: type checking, undefined variable detection, unknown op detection, unbounded loop prevention.
- [x] **TASK-2.3:** Core Ops suite: `sql`, `http`, `if`/`switch`, `loop` (capped), `transform`, `try`/`catch`, `parallel`, `approval`, `return`/`fail`.
- [x] **TASK-2.4:** Per-step timeout and retry with exponential backoff.
- [x] **TASK-2.5:** Idempotency keys support for every side-effect operation.
- [x] **TASK-2.6:** Immutable versioning and rollback capability for registered workflows.
- [x] **TASK-2.7:** Dry-run execution mode (simulate execution without executing side-effects).

---

## Phase 3 — Secrets Isolation (COMPLETED)
*Goal: References only (`{{secret:name}}`), injected at runtime into permitted steps, zero agent visibility, multi-source, automatic redaction.*
- [x] **TASK-3.1:** Secret provider abstraction (sources: environment variables, secret files, OS keychain / credential store, in-memory).
- [x] **TASK-3.2:** Secret syntax parser (`{{secret:name}}`) in template & VM engine with late-binding step injection.
- [x] **TASK-3.3:** Automatic redaction engine for logs, audit trails, error strings, and VM outputs.
- [x] **TASK-3.4:** Canary secrets test suite: inject canary secrets across SQL, HTTP, errors, and assert zero leakage across all sinks.

---

## Phase 4 — Policy + Sandbox (Deny-by-Default)
*Goal: Strict host allowlist with SSRF defense, parameterized & read-only SQL guards, argument constraints, resource caps, policy packs.*
- [ ] **TASK-4.1:** SSRF defenses: DNS resolution check, blocking private/loopback/link-local/multicast IP ranges, pinned IP requests.
- [ ] **TASK-4.2:** Safe redirect policy: refuse HTTP redirects to non-allowed hosts or unvetted IPs; DNS-rebinding test suite.
- [ ] **TASK-4.3:** SQL sandbox: enforce parameterized queries only, optional read-only mode, table allowlists, query row result caps.
- [ ] **TASK-4.4:** Argument constraints (e.g. `amount <= 100`) and resource caps (execution time, variable memory limits, loop iteration bounds).
- [ ] **TASK-4.5:** Per-agent rate limits, quotas, and ready-made policy packs (`read_only_analyst`, `billing_operator`, `support_agent`).

---

## Phase 5 — Approval Gates + Durable Runs
*Goal: Approval step pauses execution, persisted state in SQLite, approve/reject via CLI + webhook, crash recovery.*
- [ ] **TASK-5.1:** Approval step opcode: pause run, generate human-readable summary, assign run ID.
- [ ] **TASK-5.2:** Persist run state machine in SQLite (`RUNNING`, `PAUSED_APPROVAL`, `APPROVED`, `REJECTED`, `EXPIRED`, `COMPLETED`, `FAILED`).
- [ ] **TASK-5.3:** CLI approval commands: `worpen approval list`, `worpen approval view <id>`, `worpen approval approve <id>`, `worpen approval reject <id>`.
- [ ] **TASK-5.4:** Approval webhook endpoints and conditional trigger logic.
- [ ] **TASK-5.5:** Expiry handling (timed-out approval = automatic rejection) and cancelation.
- [ ] **TASK-5.6:** Durable resume after server crash/restart and run ID polling.

---

## Phase 6 — Tamper-Evident Audit
*Goal: Append-only hash-chained audit log, verification CLI, provenance records, standard exports.*
- [ ] **TASK-6.1:** Hash-chained audit log structure (`SHA-256(prev_hash + entry_data)`).
- [ ] **TASK-6.2:** Per-step audit records (agent identity, redacted inputs/outputs, policy evaluation decisions, approver ID).
- [ ] **TASK-6.3:** Audit verification command: `worpen audit verify`.
- [ ] **TASK-6.4:** Export formatters for JSONL, CEF (Common Event Format), and OpenTelemetry (OTel) traces.

---

## Phase 7 — MCP Quality
*Goal: Tool annotations, identity filtering, structured errors, approval tools, client compatibility.*
- [ ] **TASK-7.1:** MCP tool annotations (`readOnly`, `destructive`, `idempotent` hints).
- [ ] **TASK-7.2:** Identity-based `tools/list` filtering based on caller credentials and permissions.
- [ ] **TASK-7.3:** Structured agent-friendly error format: `{code, message, hint, retryable, fields}`.
- [ ] **TASK-7.4:** Built-in MCP management tools: `get_run`, `list_pending_approvals`.
- [ ] **TASK-7.5:** Compatibility integration tests against real MCP client scenarios; exportable configs for Claude Desktop, Cursor, and VS Code.

---

## Phase 8 — Developer Experience
*Goal: Prebuilt binaries, Docker image, one-line install, templates, doctor, testing harnesses.*
- [ ] **TASK-8.1:** Prebuilt binary release workflows via GitHub Actions (Linux, macOS, Windows).
- [ ] **TASK-8.2:** Dockerfile with minimal scratch/distroless base.
- [ ] **TASK-8.3:** `worpen init` CLI command with starter templates (`refund`, `db-report`, `issue-triage`).
- [ ] **TASK-8.4:** `worpen doctor` enhanced system diagnostics (database, permissions, ports, environment).
- [ ] **TASK-8.5:** `worpen test` command with mock SQL/HTTP backends for CI/CD testing.

---

## Phase 9 — Security Hardening
*Goal: Threat model, adversarial testing suite, cargo-audit/deny, fuzz testing, security disclosure policy.*
- [ ] **TASK-9.1:** Author comprehensive `THREAT_MODEL.md`.
- [ ] **TASK-9.2:** Adversarial test suite (SQL injection attempts, template escapes, SSRF & DNS rebinding, secret leaks, DoS payloads).
- [ ] **TASK-9.3:** Integrate `cargo-audit` and `cargo-deny` in CI for vulnerability and license compliance checks.
- [ ] **TASK-9.4:** Fuzzing targets (`cargo-fuzz`) for YAML workflow parser and expression template engine.
- [ ] **TASK-9.5:** Rewrite `SECURITY.md` detailing security boundaries and threat mitigations honestly.

---

## Phase 10 — Launch Assets
*Goal: Clean README rewrite, reproducible 60-second demo, documented security model, verified benchmarks only.*
- [ ] **TASK-10.1:** Rewrite `README.md` with clean formatting (no decorative font artifacts), clear problem statement, and security model.
- [ ] **TASK-10.2:** Provide a verified 60-second refund demo with CLI human approval.
- [ ] **TASK-10.3:** Honest benchmarks with reproducible benchmark scripts and methodology; delete unverified comparisons.
- [ ] **TASK-10.4:** Production-ready `/docs` documentation site structure.
- [ ] **TASK-10.5:** Author comprehensive final `REPORT.md`.
