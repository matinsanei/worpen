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
### **ᴛʜᴇ ᴀᴄᴛɪᴏɴ ᴘʟᴀɴᴇ ꜰᴏʀ ᴀɪ ᴀɢᴇɴᴛs** • *ᴘᴏᴡᴇʀᴇᴅ ʙʏ ʀᴜsᴛ* 🦀

[![MCP](https://img.shields.io/badge/MCP-SERVER_%2B_STDIO-purple?style=for-the-badge&logo=openai)](https://modelcontextprotocol.io/)
[![Core](https://img.shields.io/badge/CORE-RUST-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Sandbox](https://img.shields.io/badge/SANDBOX-CAPABILITY--BASED-red?style=for-the-badge&logo=shield)](documentation/01-introduction.md)
[![Tests](https://img.shields.io/badge/TESTS-250_PASSING-blue?style=for-the-badge&logo=github-actions)](backend/crates/core/tests)
[![License](https://img.shields.io/badge/LICENSE-MIT-blue?style=for-the-badge&logo=open-source-initiative)](LICENSE)

<p align="center">
  <b>MCP Server</b> • <b>Sandboxed VM</b> • <b>Multi-Step Pipelines</b> • <b>Single Binary</b>
</p>

> *"Don't make your LLM juggle ten tools. Give it one Worpen tool that runs the whole job — branched, looped, cached and sandboxed."*

[Why Worpen for AI](#-why-ai-agents-need-worpen) • [60-Second Demo](#-60-second-demo) • [MCP Setup](#-mcp--give-your-agent-hands) • [Sandbox](#-sandbox--let-agents-touch-prod-safely) • [Docs](#-documentation)

</div>

---

## 🧠 𝐖𝐇𝐘 𝐀𝐈 𝐀𝐆𝐄𝐍𝐓𝐒 𝐍𝐄𝐄𝐃 𝐖𝐎𝐑𝐏𝐄𝐍

Every MCP server today is an **island**: one talks to the DB, one sends email, one hits Stripe. To finish a real task (*"refund order #42 and notify the customer"*), the LLM must chain 5+ tool calls itself — burning tokens, latency and patience, with full credentials in every hop.

**Worpen is the layer between your agent and the mess:**

```text
  BEFORE (LLM does everything)              AFTER (Worpen does the job)

  LLM → db_lookup  ─┐                          LLM ──► refund_order ──► Worpen VM
  LLM → calc_refund ─┤ 5 round-trips,               ├─ SQL (allow-listed)
  LLM → stripe_api  ─┤ 5x latency,                  ├─ if/else + math
  LLM → send_email  ─┤ credentials everywhere       ├─ http → Stripe
  LLM → log_event   ─┘                              └─ email + audit log
                                                    1 call. Sandboxed. Deterministic.
```

- **Routes are tools.** Every YAML/JSON route you define instantly becomes an MCP tool (`tools/list` → `tools/call`). No glue code.
- **Pipelines are jobs.** Multi-stage workflows with shared scope, per-stage timeouts and `allow_failure` become namespaced tools (`pipeline:<name>`) — the thing MCP ecosystems are missing.
- **Sandboxed by design.** Capability-based policy per route: allowed hosts, SQL/Redis on-off, script execution deny-by-default, timeouts, rate limits, memory caps. The agent gets a leash, not the keys.
- **Rust speed.** Logic compiles to bytecode and runs on an integer-indexed VM with direct array memory. Your agent waits milliseconds, not seconds.

---

## ⚡ 60-𝐒𝐄𝐂𝐎𝐍𝐃 𝐃𝐄𝐌𝐎

```bash
# 1. One file, one port — API + UI + MCP + CLI
npm run build:release
./backend/target/release/worpen serve --port 3000
# Open http://127.0.0.1:3000
```

```bash
# 2. Define an action in YAML (hot-swappable, zero downtime)
curl -X POST http://127.0.0.1:3000/api/v1/dynamic-routes/register \
  -H 'Content-Type: application/x-yaml' -d '
name: refund_order
path: /api/refund
method: POST
logic:
  - sql_op:
      query: "SELECT total FROM orders WHERE id = ?"
      args: ["{{order_id}}"]
      output_var: order
  - math_op:
      operation: mul
      args: ["{{order}}", 0.9]
  - http_request:
      url: "https://api.stripe.com/v1/refunds"
      method: POST
  - return:
      value: "{{http_response}}"
'
```

```bash
# 3. Your agent calls it as ONE MCP tool
curl -X POST http://127.0.0.1:3000/mcp \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call",
       "params":{"name":"refund_order","arguments":{"payload":{"order_id":"42"}}}}}'
```

A 4-step job — SQL, branching math, external API, structured return — in **one tool call**.

---

## 🔌 𝐌𝐂𝐏 — 𝐆𝐈𝐕𝐄 𝐘𝐎𝐔𝐑 𝐀𝐆𝐄𝐍𝐓 𝐇𝐀𝐍𝐃𝐒

| Transport | How | Use for |
|:----------|:----|:--------|
| **HTTP** | `POST /mcp` — JSON-RPC (`initialize`, `tools/list`, `tools/call`) | OpenAI-style clients, webhooks, `curl` |
| **REST** | `GET /mcp/tools`, `GET /mcp` (info) | Debugging, discovery |
| **STDIO** | `worpen mcp serve-stdio` | **Claude Desktop**, agent harnesses |

**Claude Desktop** (`claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "worpen": {
      "command": "/path/to/worpen",
      "args": ["mcp", "serve-stdio"],
      "env": { "DATA_DIR": "/path/to/backend/data" }
    }
  }
}
```

Tool naming: routes keep their names (`refund_order`), pipelines are namespaced (`pipeline:nightly_reconcile`). Auth-gated routes (`auth_required: true` + `WORPEN_API_TOKEN`) are enforced on every transport — including MCP.

---

## 🛡️ 𝐒𝐀𝐍𝐃𝐁𝐎𝐗 — 𝐋𝐄𝐓 𝐀𝐆𝐄𝐍𝐓𝐒 𝐓𝐎𝐔𝐂𝐇 𝐏𝐑𝐎𝐃 𝐒𝐀𝐅𝐄𝐋𝐘

```yaml
name: ai_generated_route
sandbox:
  allow_net: true
  allowed_hosts: ["api.stripe.com"]  # nothing else reachable
  allow_sql: false                    # no DB for this one
  allow_scripts: false                # never arbitrary code
  max_timeout_ms: 10000
  max_loop_iterations: 500
  max_variables: 500
```

Plus per-route **rate limits** (429 when exceeded) and a full **audit trail** (`GET /api/v1/dynamic-routes/audit`) — every agent action is recorded with timing and outcome. The UI's *Error Logs* page streams failures live from that trail.

Generate routes with `--locked` and the CLI applies the strict policy automatically:
```bash
worpen route add --file agent_task.yaml --locked
```

---

## 🖥️ 𝐂𝐋𝐈 (fully headless)

```bash
worpen setup                                  # interactive first-run wizard → writes .env
worpen serve [--port 3000] [--no-ui]          # API + UI + MCP on one port
worpen route list | add | exec | delete | audit
worpen func list | add | delete
worpen pipeline list | define | run | runs    # multi-stage jobs
worpen mcp tools | call --tool <name>
worpen doctor
```

---

## 🏗️ 𝐇𝐎𝐖 𝐈𝐓 𝐖𝐎𝐑𝐊𝐒

Not a JSON parser — a **runtime compiler**:

1. **Lowering:** `{{user_price}}` becomes an integer index into a symbol table — `O(1)` access, no hashmaps on the hot path.
2. **Inlining:** global functions are recursively flattened into the route at registration. Call overhead: zero.
3. **VM execution:** a flat bytecode loop over direct array memory.

| Op family | Examples |
|:----------|:---------|
| Memory / Logic | `set`, `get`, `if`, `switch`, `loop`, `while`, `try/catch`, `parallel` |
| Data | `math_op`, `string_op`, `date_op`, `json_op`, `map`, `filter`, `aggregate` |
| I/O | `sql_op` (parameterized), `redis_op` (pooled), `http_request` |
| Meta | `comment` (zero-cost docs), `sleep`, `log` |

---

## 📚 𝐃𝐎𝐂𝐔𝐌𝐄𝐍𝐓𝐀𝐓𝐈𝐎𝐍

- 📖 [Introduction](documentation/01-introduction.md) • [Variables & VM](documentation/03-variables-basics.md) • [Loops](documentation/05-loops.md) • [Functions](documentation/06-functions.md)
- 🧩 [Custom Operations](documentation/CUSTOM_OPERATIONS_GUIDE.md) • 📦 [Schema Examples](documentation/SCHEMA_EXAMPLES.md) • 🚀 [Extensions Quickstart](documentation/EXTENSIONS_QUICKSTART.md)
- 🔄 [JSON→YAML Migration](documentation/15-migration-guide.md) • ✨ [Best Practices](documentation/16-best-practices.md) • 📝 [Changelog](CHANGELOG.md)

### Roadmap — toward the enterprise action plane
- [ ] **Workflow Spec v1:** strict validation, immutable versions, dry-run mode
- [ ] **Secret Isolation:** runtime references (`{{secret:name}}`), multi-source, automatic redaction
- [ ] **SSRF & Sandbox:** pre-resolution DNS filtering, private IP blocking, parameterized/read-only SQL guards
- [ ] **Approval Gates:** pause workflow run for human sign-off via CLI/webhook, durable SQLite state
- [ ] **Tamper-Evident Audit:** cryptographically chained audit log (`worpen audit verify`)

---

## 🤝 𝐂𝐎𝐍𝐓𝐑𝐈𝐁𝐔𝐓𝐈𝐍𝐆

We welcome performance hackers, compiler nerds and agent-infra builders.

1. **Fork** & **Clone**.
2. `cargo test` (250+ tests guard the VM) • `cargo clippy --all-targets -- -D warnings` (must be clean).
3. Submit a PR with benchmarks.

---

## 👨‍💻 𝐀𝐔𝐓𝐇𝐎𝐑

**Matin Sanei**
- **GitHub:** [@matinsanei](https://github.com/matinsanei)
- **Email:** saneimatin33@gmail.com

*"The best code is the one you never compile."*

**MIT Licensed** • Copyright © 2025 Worpen Engine

</div>
