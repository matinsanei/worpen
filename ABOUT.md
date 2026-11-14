# About Worpen

### Repository Description
> The Action Plane for AI Agents. Sandboxed multi-step workflows, zero visible credentials, human approval gates, and tamper-evident audit trails. Built with Rust and MCP.

---

### Suggested GitHub Topics
`ai-agents` `mcp` `model-context-protocol` `action-plane` `workflow-engine` `sandboxing` `security` `ssrf-defense` `audit-trail` `rust` `human-in-the-loop`

---

### The Problem We Solve
Every Model Context Protocol (MCP) server today gives LLMs direct, granular tool access (raw SQL queries, individual HTTP mutations). This forces LLMs to orchestrate complex multi-step business logic across multiple tool calls—burning tokens, increasing latency, seeing plaintext credentials at every hop, and executing irreversible mutations without human oversight.

### The Solution: Worpen Action Plane
Worpen shifts multi-step orchestration from probabilistic LLM loops into a deterministic, high-speed Rust runtime:
1. **One Call, Full Job:** An agent calls `workflow:refund` once. Worpen executes SQL lookups, calculations, conditional branches, and external APIs in milliseconds.
2. **Zero Visible Credentials:** Credentials use `{{secret:name}}` late-binding injection. An agent never sees an API token or database password, and the engine automatically sanitizes `[REDACTED]` across all logs and outputs.
3. **Capability-Based Sandboxing:** Built-in SSRF defenses block private IP ranges (`127.0.0.1`, `169.254.169.254`, RFC 1918), pin DNS sockets against rebinding, and enforce read-only SQL with table whitelists.
4. **Human Approval Gates:** High-risk actions pause execution cleanly in a durable SQLite state machine. Runs survive process restarts and can be inspected and approved via CLI or MCP.
5. **Tamper-Evident Audit:** Every lifecycle event is cryptographically hash-chained (`SHA-256`) from genesis and exportable to enterprise SIEMs in CEF, JSONL, or OpenTelemetry formats.
