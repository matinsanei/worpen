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

## Phase 4 — Policy + Sandbox (Deny-by-Default) (COMPLETED)
*Goal: Strict host allowlist with SSRF defense, parameterized & read-only SQL guards, argument constraints, resource caps, policy packs.*
- [x] **TASK-4.1:** SSRF defenses: DNS resolution check, blocking private/loopback/link-local/multicast IP ranges, pinned IP requests.
- [x] **TASK-4.2:** Safe redirect policy: refuse HTTP redirects to non-allowed hosts or unvetted IPs; DNS-rebinding test suite.
- [x] **TASK-4.3:** SQL sandbox: enforce parameterized queries only, optional read-only mode, table allowlists, query row result caps.
- [x] **TASK-4.4:** Argument constraints (JSON schema verification) and resource caps (execution time, variable memory limits, loop iteration bounds).
- [x] **TASK-4.5:** Per-agent rate limits, quotas, and ready-made policy packs (`read_only_analyst`, `billing_operator`, `support_agent`).

---

## Phase 5 — Approval Gates + Durable Runs (COMPLETED)
*Goal: Approval step pauses execution, persisted state in SQLite, approve/reject via CLI + webhook, crash recovery.*
- [x] **TASK-5.1:** Approval step opcode: pause run, generate human-readable summary, assign run ID.
- [x] **TASK-5.2:** Persist run state machine in SQLite (`Pending`, `Running`, `WaitingForApproval`, `Completed`, `Failed`, `Rejected`, `Cancelled`, `Expired`).
- [x] **TASK-5.3:** Run management API & CLI foundations: `/api/v1/runs`, `/api/v1/runs/pending`, `/api/v1/runs/:id`, `/approve`, `/reject`, `/cancel`.
- [x] **TASK-5.4:** Approval webhook endpoints and conditional trigger logic.
- [x] **TASK-5.5:** Expiry handling (timed-out approval = automatic rejection) and cancelation.
- [x] **TASK-5.6:** Durable resume after server crash/restart and run ID polling.

---

## Phase 6 — Tamper-Evident Audit
*Goal: Append-only hash-chained audit log, verification CLI, provenance records, standard exports.*
- [x] **TASK-6.1:** Hash-chained audit log structure (`SHA-256(prev_hash + entry_data)`).
- [x] **TASK-6.2:** Per-step audit records (agent identity, redacted inputs/outputs, policy evaluation decisions, approver ID).
- [x] **TASK-6.3:** Audit verification command: `worpen audit verify` & `/api/v1/audit/verify`.
- [x] **TASK-6.4:** Export formatters for JSONL, CEF (Common Event Format), and OpenTelemetry (OTel) traces.

---

## Phase 7 — MCP Quality
*Goal: Tool annotations, identity filtering, structured errors, approval tools, client compatibility.*
- [x] **TASK-7.1:** MCP tool annotations (`readOnly`, `destructive`, `idempotent` hints).
- [x] **TASK-7.2:** Identity-based `tools/list` filtering based on caller credentials and permissions.
- [x] **TASK-7.3:** Structured agent-friendly error format: `{code, message, hint, retryable, fields}`.
- [x] **TASK-7.4:** Built-in MCP management tools: `get_run`, `list_pending_approvals`, `approve_run`, `verify_audit`.
- [x] **TASK-7.5:** Compatibility integration tests against real MCP client scenarios; exportable configs for Claude Desktop, Cursor, and VS Code.

---

## Phase 8 — Developer Experience
*Goal: Prebuilt binaries, Docker image, one-line install, templates, doctor, testing harnesses.*
- [x] **TASK-8.1:** Prebuilt binary release workflows via GitHub Actions (Linux, macOS, Windows).
- [x] **TASK-8.2:** Dockerfile with minimal multi-stage base and unprivileged user.
- [x] **TASK-8.3:** `worpen init` CLI command with starter templates (`refund`, `db-report`, `issue-triage`).
- [x] **TASK-8.4:** `worpen doctor` enhanced system diagnostics (database, permissions, audit, SSRF sandbox, secrets, auth).
- [x] **TASK-8.5:** CLI workflow and audit management (`worpen workflow register/run/pending/approve`, `worpen audit verify/export`).

---

## Phase 9 — Security Hardening
*Goal: Threat model, adversarial testing suite, cargo-audit/deny, fuzz testing, security disclosure policy.*
- [x] **TASK-9.1:** Author comprehensive `THREAT_MODEL.md` (STRIDE analysis, 5 trust boundaries, protect surfaces).
- [x] **TASK-9.2:** Adversarial test suite (`adversarial_security_test.rs`: SSRF obfuscations, SQL injection tricks, path traversal, unbounded loops).
- [x] **TASK-9.3:** Integrate vulnerability and security auditing in CI pipeline.
- [x] **TASK-9.4:** Fuzzing targets & validation guarantees for YAML workflow parser and sandbox engine.
- [x] **TASK-9.5:** Rewrite `SECURITY.md` detailing security boundaries, explicit in-scope/out-of-scope, and coordinated disclosure process.

---

## Phase 10 — Launch Assets
*Goal: Clean README rewrite, reproducible 60-second demo, documented security model, verified benchmarks only.*
- [ ] **TASK-10.1:** Rewrite `README.md` with clean formatting (no decorative font artifacts), clear problem statement, and security model.
- [ ] **TASK-10.2:** Provide a verified 60-second refund demo with CLI human approval.
- [ ] **TASK-10.3:** Honest benchmarks with reproducible benchmark scripts and methodology; delete unverified comparisons.
- [ ] **TASK-10.4:** Production-ready `/docs` documentation site structure.
- [ ] **TASK-10.5:** Author comprehensive final `REPORT.md`.
