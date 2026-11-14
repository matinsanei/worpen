# Worpen Threat Model & Security Architecture

**Document Version:** 1.0.0  
**Target System:** Worpen Action Plane for AI Agents (`v0.1.1`)  
**Methodology:** STRIDE (Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of Privilege)

---

## 1. System Mission & Core Security Contract

Worpen is designed from the ground up as a hardened, policy-governed action plane for autonomous AI agents:
> *"An agent completes a real multi-step job in one call, never sees a credential, can only do what policy allows, risky steps pause for human approval, and every action is provably recorded."*

In this architecture, autonomous agents (such as Claude, Cursor, or custom LLM harnesses) are treated as **untrusted or semi-trusted callers**. They are never granted direct access to underlying credentials, arbitrary network sockets, or raw database connections.

---

## 2. Assets & Protect Surfaces

| Asset | Description | Security Objective |
| :--- | :--- | :--- |
| **Credentials & Secrets** | Database passwords, third-party API tokens, OAuth keys. | Confidentiality: Zero visibility to LLMs, prompts, or response logs. |
| **Enterprise Data Stores** | Relational databases and backend microservices. | Integrity & Availability: Read-only enforcement, table allowlists, row caps. |
| **Internal Networks** | Cloud metadata endpoints (`169.254.169.254`), private VPCs, localhost. | Isolation: Zero SSRF reachability; strict DNS pre-resolution & IP pinning. |
| **Execution State** | Workflow variables, run state, and pending approvals. | Integrity: Durable SQLite state machine immune to restart loss or bypass. |
| **Audit Trails** | Complete record of agent and human actions. | Non-Repudiation: Cryptographic SHA-256 hash chaining with genesis anchor. |

---

## 3. Trust Boundaries & Data Flow

```
   ┌─────────────────────────────────────────────────────────────┐
   │ Untrusted External Domain (LLMs, Agents, Webhooks, Clients) │
   └──────────────────────────────┬──────────────────────────────┘
                                  │ JSON-RPC / REST / Headers
                        [Trust Boundary 1: Ingress & Auth]
                                  ▼
   ┌─────────────────────────────────────────────────────────────┐
   │ Worpen Action Plane Runtime                                  │
   │                                                             │
   │  ┌────────────────────┐   ┌──────────────────────────────┐  │
   │  │ Workflow Engine    │──▶│ Sandbox Policy Engine        │  │
   │  │ (Durable State)    │   │ (SSRF Guard, SQL AST Guard)  │  │
   │  └─────────┬──────────┘   └──────────────┬───────────────┘  │
   │            │                             │                  │
   │   [Trust Boundary 2]            [Trust Boundary 3]          │
   │            │                             │                  │
   │            ▼                             ▼                  │
   │  ┌────────────────────┐   ┌──────────────────────────────┐  │
   │  │ SecretStore        │   │ Outbound HTTP / SQL Client   │  │
   │  │ (Late-Binding &    │   │ (IP-Pinned Socket, Safe Red.)│  │
   │  │  RedactionEngine)  │   └──────────────┬───────────────┘  │
   │  └────────────────────┘                  │                  │
   │            │                    [Trust Boundary 4]          │
   │            ▼                             │                  │
   │  ┌────────────────────┐                  ▼                  │
   │  │ Hash-Chained Audit │       Outbound Private/Public Sinks │
   │  │ (SHA-256 Chain)    │       (External APIs, Enterprise DB)│
   │  └────────────────────┘                                     │
   └─────────────────────────────────────────────────────────────┘
```

- **Boundary 1 (Client Ingress):** Enforces token authentication (`WORPEN_API_TOKEN`) and caller identity/role extraction (`X-Agent-Role`).
- **Boundary 2 (Secret Isolation):** `{{secret:name}}` tokens are never stored in symbol tables or memory visible to the caller; resolved strictly at execution boundaries.
- **Boundary 3 (Policy Sandbox):** Validates SQL statements against `read_only_sql` and `allowed_tables`; pre-resolves DNS and checks IP allow/blocklists before socket connection.
- **Boundary 4 (Outbound Sinks):** Outbound connections are pinned to validated IPs to prevent DNS rebinding; HTTP redirects are strictly re-vetted against policy.

---

## 4. STRIDE Threat Analysis

### 4.1 Spoofing Identity
* **Threat:** A compromised agent or external caller impersonates an authorized supervisor or self-approves high-risk operations (e.g. issuing financial refunds).
* **Mitigation:**
  - **Dedicated Operator Channel:** Approval tools (`approve_run`, `reject_run`) are strictly forbidden via MCP and removed from agent toolsets (`APPROVAL_CHANNEL_SEPARATION`).
  - **Strict Anti-Self-Approval Enforcement:** The durable workflow engine records the run `initiator`. If the approver matches the initiator, the engine strictly rejects the approval.
  - **Authenticated Management Channels:** Approvals require dedicated operator credentials via CLI (`worpen workflow approve`) or authenticated REST endpoints with `WORPEN_API_TOKEN`.

### 4.2 Tampering with Data
* **Threat 1:** An agent constructs malicious SQL injection queries (e.g. CTEs `WITH x AS (...) DELETE`, comment obfuscation `/* ... */ DROP`, comma joins) to alter database records or access unauthorized tables.
  * **Mitigation:** 
    - Full comment and string literal stripping before tokenization.
    - Whole-query token stream inspection denying all DDL/DML mutation keywords (`INSERT`, `UPDATE`, `DELETE`, `DROP`, `ALTER`, `TRUNCATE`, `REPLACE`, `ATTACH`, `PRAGMA`).
    - Multi-table extraction handling comma joins (`FROM t1, t2`) and checking all referenced tables against `allowed_tables`.
    - Runtime SQLite C-engine enforcement via `PRAGMA query_only = ON;` on the active connection.
* **Threat 2:** An attacker with filesystem write access tampers with on-disk execution logs or recomputes SHA-256 hash chains to erase evidence.
  * **Mitigation:** 
    - Keyed HMAC-SHA256 cryptographic chaining (`WORPEN_AUDIT_KEY`). Every block's entry hash is cryptographically signed with the HMAC secret key.
    - An attacker who recalculates SHA-256 hashes or truncates the log cannot forge valid HMAC signatures without the audit secret key.

### 4.3 Repudiation
* **Threat:** An operator denies having approved a risky financial transaction or privileged action.
* **Mitigation:**
  - Approvals require identity and justification recorded in SQLite `workflow_runs` table with timestamps.
  - Provable hash-chained audit log records all lifecycle events (`workflow.approval_requested`, `workflow.approved`, `workflow.completed`) with exportability to CEF (Common Event Format) and OpenTelemetry.

### 4.4 Information Disclosure
* **Threat:** Secrets (e.g. Stripe API keys, database credentials) leak into LLM prompts, agent context windows, error reflections, or audit logs through encoding transformations (Base64, Hex, URL percent-encoding).
* **Mitigation:**
  - Variable lowerer explicitly strips and refuses to register `secret:*` tokens into symbol tables or agent-accessible variable memory.
  - Secrets are injected late at the transport boundary into HTTP headers or parameterized SQL query binds.
  - `RedactionEngine` automatically registers and redacts multiple representations of every secret: raw plaintext, Base64 (padded and unpadded), Hexadecimal (lowercase and uppercase), and URL percent-encoding.

### 4.5 Denial of Service (DoS)
* **Threat 1:** An agent generates unbounded loops or recursive calls that hang the runtime or exhaust CPU.
  * **Mitigation:** Workflow registration validator rejects any workflow with loops lacking explicit `max_iterations`, and enforces an upper bound defined by policy.
* **Threat 2:** SQL queries return millions of rows, exhausting host memory.
  * **Mitigation:** Sandboxed SQL execution enforces `max_sql_rows` (default 500-1,000 rows) with strict fetch limits.

### 4.6 Elevation of Privilege (SSRF & Sandbox Escapes)
* **Threat:** An agent instructs Worpen to make outbound HTTP requests to internal IP addresses (e.g. `127.0.0.1`, AWS metadata `169.254.169.254`, router management interfaces) or executes DNS rebinding attacks.
* **Mitigation:**
  - **Deny-by-Default Policy:** `allow_net: false` and `allow_sql: false` by default; an empty `allowed_hosts` list strictly rejects all outbound network calls.
  - Outbound IP range blocklist covering RFC 1918 private subnets, loopback (`127.0.0.0/8`, `::1`), link-local (`169.254.0.0/16`, `fe80::/10`), CGNAT (`100.64.0.0/10`), and multicast.
  - Pre-flight DNS resolution with IP pinning on the outbound `reqwest` client, defeating TOCTOU DNS rebinding.
  - Strict redirect validation ensuring redirect targets do not resolve to blocked IPs or unlisted domains.

---

## 5. Security Verification & Test Coverage

All mitigations described in this threat model are continuously verified by automated regression test suites:
- `backend/crates/core/tests/policy_sandbox_test.rs`: SSRF IP ranges, DNS pre-resolution, redirect checks, SQL table allowlists, and read-only enforcement.
- `backend/crates/core/tests/canary_secrets_test.rs`: Secret reflection redaction, zero symbol-table leakage, and canary token isolation.
- `backend/crates/core/tests/audit_tamper_test.rs`: Detection of tampered details, broken hash pointers, and index deletions.
- `backend/crates/core/tests/approval_durable_test.rs`: Approval gate timeouts, crash recovery, and state transitions.
- `backend/crates/api/tests/mcp_test.rs`: MCP tool annotations, identity filtering, and structured agent error responses.
- `backend/crates/core/tests/adversarial_security_test.rs`: Attack payload regression suite.
