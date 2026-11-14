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
