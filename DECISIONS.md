# Architecture Decision Records (ADR)

## ADR-0001: Focus on Worpen as the Action Plane for AI Agents
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** The codebase originally grew from an infrastructure management and host/container agent monitor into a dynamic route and workflow execution runtime. It accumulated legacy endpoints (Docker control, agent shells, telemetry, WebSocket multiplexing) that are orthogonal to running deterministic, sandboxed workflows for AI agents.
- **Decision:** All development must strictly align with the North Star: "An agent completes a real multi-step job in one call, never sees a credential, can only do what policy allows, risky steps pause for human approval, and every action is provably recorded." Non-core services (container ops, agent shells, WebSockets) will be frozen/removed from the default workflow build in Phase 1.
- **Consequences:** Simplifies codebase, reduces attack surface, enhances security auditability, and sharpens product definition.

## ADR-0002: Elimination of Fabricated Benchmark Comparisons
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** `README.md` contained an ASCII comparison chart claiming Worpen executes 38,405 req/s vs Go (17,800), Node (13,000), and Python (3,900). No code or methodology existed in the repository to reproduce these figures.
- **Decision:** Remove all unverified comparison numbers and benchmark claims from documentation until reproducible scripts and rigorous benchmarking harnesses are integrated into the repository.
- **Consequences:** Adheres strictly to the honesty principle. Prevents misleading users and enterprise security teams.

## ADR-0003: Strict Host Allowlist Matching
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** `SandboxPolicy::is_host_allowed` used `h.contains(&a)` to match target hosts, allowing spoofed domains like `api.stripe.com.evil.com` or `evil-api.stripe.com` to bypass sandbox restrictions.
- **Decision:** Restrict host matching strictly to exact equality or explicit subdomain suffixes (`h == a || h.ends_with(&format!(".{}", a))`). Reject any substring-only matching.
- **Consequences:** Closes critical security hole SEC-01. Prevents domain bypasses in outbound HTTP actions.

## ADR-0004: Freeze Legacy Endpoints & Wire Database Pool in STDIO MCP
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** The default HTTP router exposed dozens of unmaintained agent/container monitoring and WebSocket endpoints. Concurrently, `DynamicRouteService` never received the initialized database pool in `build_state` or `serve_stdio`, causing any route executing `SqlOp` to crash.
- **Decision:** Freeze all legacy daemon endpoints (Docker, fleet agents, incidents, terminals, dynamic WebSockets) from the active server router. Inject the initialized SQLite connection pool into `DynamicRouteService` across both HTTP serve and STDIO MCP modes, and ensure execution errors in VM are surfaced directly without silent fallback.
- **Consequences:** Hardens the attack surface, aligns the runtime strictly with workflow execution, and enables full database operations for local agents via STDIO MCP.

## ADR-0005: Workflow Spec v1, Strict Validation & Immutable Versioning
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** AI agents and human operators require deterministic multi-step workflow execution defined in clean, human-readable YAML with schema validation, bounded loop iterations, undefined variable rejection, dry-run simulation, and rollback protection.
- **Decision:** Introduce `WorkflowSpecV1` (`name`, `description`, `input_schema`, `policy`, `steps`, `return`). Validate variable scopes strictly at registration to prevent runtime crashes from undefined variables. Enforce `max_iterations` caps on all loops. Provide simulation in `ExecutionMode::DryRun` where side-effects (`sql`, `http`, `approval`) are mocked with `_worpen_dry_run: true`. Store each registered version immutably in `<data_dir>/workflows/<name>/v<version>.yaml` and enable zero-downtime rollback. Automatically synchronize registered workflows into the integer-indexed VM and MCP server.
- **Consequences:** Provides a production-grade action plane contract for AI agents. Prevents dangerous unbounded loops, catches syntax/variable errors before deployment, and enables risk-free dry runs.

## ADR-0006: Secret Isolation, Late-Binding Injection, and Automatic Canary Redaction
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** To satisfy the North Star principle ("An agent never sees a credential"), agents must never receive plaintext API keys, database passwords, or auth tokens. If an agent could inspect runtime memory or variable tables, credentials would leak into prompt context. Furthermore, if external APIs or databases echo secrets in responses or error logs, credentials could leak into audit files or agent responses.
- **Decision:**
  1. Secret references strictly use syntax `{{secret:name}}`.
  2. The compiler and lowerer never register `secret:*` references into agent variable memory or symbol tables.
  3. Secrets are resolved at the last possible moment right before step execution (e.g. binding SQL query parameters, injecting HTTP request headers/URLs).
  4. Pluggable `SecretSource` implementations support environment variables (`WORPEN_SECRET_*`), files (e.g. Docker/Kubernetes secrets), in-memory vaults, and OS keychain lookups.
  5. `RedactionEngine` automatically tracks all resolved secret values and replaces them with `[REDACTED]` across all workflow output values, error strings, and audit logs (`audit.log` and in-memory ring).
  6. Canary tests verify that even if an outbound HTTP server echoes the credential back, or a database error reflects the query, zero secret tokens leak to outputs or audit files.
- **Consequences:** Agents are completely isolated from credentials while retaining the ability to execute privileged operations through policy-governed steps. Zero credential leakage across outputs, logs, and audit trails.

## ADR-0007: Deny-by-Default Policy, SSRF Guards, and IP Pinning
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** AI workflows interacting with external HTTP and database resources are vulnerable to Server-Side Request Forgery (SSRF), DNS rebinding, cloud metadata exfiltration (169.254.169.254), internal port scanning, SQL injection, and data tampering.
- **Decision:**
  1. Default policy denies all unlisted network access.
  2. Implement strict SSRF defenses in `worpen_core::sandbox::ssrf` blocking private (RFC 1918), link-local, loopback, CGNAT, broadcast, and multicast IP ranges (including IPv4-mapped IPv6).
  3. Pre-resolve DNS and pin the outbound socket connection to the vetted IP, preventing time-of-check to time-of-use (TOCTOU) DNS rebinding attacks.
  4. Enforce safe redirect policies: abort HTTP redirects if target host is not in allowlist or resolves to a private IP.
  5. Provide SQL sandbox with `read_only_sql` enforcement, table allowlists, and row result caps to guard against data destruction and memory exhaustion.
  6. Provide ready-made policy packs (`read_only_analyst`, `billing_operator`, `support_agent`).
- **Consequences:** Makes workflow execution impervious to SSRF, DNS rebinding, and unauthorized database writes.

## ADR-0008: Durable Workflow Runs and Human Approval Gates
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** Mission-critical AI workflows cannot execute dangerous actions (e.g. issuing financial refunds or modifying database schemas) without human oversight. If the server process restarts while waiting for approval, the execution state must not be lost.
- **Decision:**
  1. Introduce `WorkflowOpV1::Approval` gate that pauses workflow execution and transitions the run into `WaitingForApproval`.
  2. Implement SQLite-persisted `DurableRunStore` (`workflow_runs` table) tracking run ID, status, step index, variables, and approval metadata.
  3. Provide `approve_run`, `reject_run`, and `cancel_run` APIs with configurable timeout / expiry handling (`now > expires_at` marks run as `Expired`).
  4. Resuming an approved run restores execution memory and variables, advancing execution until the next gate or completion.
  5. State survives full application restarts and provides CLI / REST endpoints (`/api/v1/runs/*`).
- **Consequences:** Guarantees deterministic, restart-safe human approval gates for high-risk agent operations.

## ADR-0009: Tamper-Evident Hash-Chained Audit Log and SIEM Export Formats
- **Date:** 2026-11-14
- **Status:** Accepted
- **Context:** To satisfy the North Star principle ("every action is provably recorded"), audit records must be cryptographically chained to prevent retroactive modification, deletion, or reordering by attackers or compromised accounts. Security teams also require integration with external SIEMs and telemetry pipelines.
- **Decision:**
  1. Implement append-only hash-chained audit logging in `worpen_core::audit::chain`.
  2. Each record computes a SHA-256 `entry_hash` over canonical fields: `index:timestamp:prev_hash:action:actor:route_or_workflow:details:policy_decision`.
  3. The first record links to a canonical `GENESIS_HASH` (`0000000000000000000000000000000000000000000000000000000000000000`). Each subsequent record references the previous entry's hash.
  4. Provide `verify_audit_chain` which detects any index jump, broken hash pointer, or modified payload, pinpointing the exact broken record index.
  5. Automatically append audit records for all workflow lifecycles (start, approval request, step failure, completion, approval, rejection, cancellation) and route executions.
  6. Provide export endpoints and converters for industry-standard SIEM formats: JSON Lines (JSONL), Common Event Format (CEF), and OpenTelemetry LogRecords (OTel).
  7. Expose REST endpoints `GET /api/v1/audit/verify` and `GET /api/v1/audit/export?format=jsonl|cef|otel`.
- **Consequences:** Provides mathematical proof of audit trail integrity and seamless ingestion into enterprise SIEMs (Splunk, Datadog, Elastic).



