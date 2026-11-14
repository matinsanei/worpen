# Security Policy

Worpen is built as a security-critical action plane for AI agents. We are committed to maintaining the highest security standards and welcome vulnerability reports from researchers and the open-source community.

---

## Supported Versions

Only the current active release line receives security updates and vulnerability patches:

| Version | Status | Supported Until |
| :--- | :--- | :--- |
| `0.1.x` | **Current Active** | Current |
| `< 0.1.0` | End of Life (Unreleased Dev) | Unsupported |

---

## Security Boundaries & Scope

### In-Scope Vulnerabilities
We actively investigate and patch vulnerabilities within Worpen's security contract:
* **Sandbox Escapes & SSRF:** Bypassing host allowlists, reaching loopback (`127.0.0.1`), link-local/cloud metadata (`169.254.169.254`), or private IP ranges (RFC 1918) via HTTP actions, DNS rebinding, or malicious redirects.
* **Credential & Secret Leakage:** Leaking `{{secret:*}}` credentials into agent-accessible variables, response payloads, error reflections, or audit logs.
* **SQL Sandbox Bypasses:** Executing unauthorized database mutations (`INSERT`, `UPDATE`, `DELETE`, `DROP`, `ALTER`) when `read_only_sql: true`, or accessing tables outside `allowed_tables`.
* **Approval Gate Bypasses:** Resuming, advancing, or completing a workflow run without legitimate human operator approval.
* **Audit Trail Tampering:** Modifying, inserting, or deleting hash-chained audit records without `verify_audit_chain` detecting the corruption.
* **Remote Code Execution (RCE) & Memory Safety:** Vulnerabilities in the bytecode VM, workflow compiler, expression parser, or MCP server leading to memory corruption or arbitrary host execution.

### Out-of-Scope
The following are considered out-of-scope:
* Attacks requiring physical or root/administrator access to the host machine running Worpen.
* Volumetric Denial of Service (DoS) attacks against self-hosted instances exposed without reverse proxies or rate limiting.
* Social engineering of human approvers who explicitly authorize a malicious transaction.
* Issues in third-party services, APIs, or databases called by user workflows.

---

## Reporting a Vulnerability

Please **DO NOT** report security vulnerabilities via public GitHub issues, discussions, or pull requests.

### Preferred Reporting Method
Report vulnerabilities through **GitHub Private Vulnerability Reporting**:
1. Navigate to the [Worpen Repository Security Advisories](https://github.com/matinsanei/worpen/security/advisories).
2. Click **"Report a vulnerability"**.
3. Include detailed steps to reproduce, proof-of-concept payloads, and affected versions.

### Alternative Reporting Method
If you cannot use GitHub Advisories, email your report directly to:
* **Contact:** `saneimatin33@gmail.com`
* **Subject:** `[SECURITY] Vulnerability in Worpen: <Brief Description>`

### What to Include
To expedite triage, please provide:
1. Clear description of the vulnerability and its potential impact.
2. Step-by-step reproduction instructions or a minimal reproducible workflow YAML / MCP request.
3. Your assessment of severity (CVSS score if available).
4. Any proposed remediations or patches.

---

## Response & Disclosure Process

We follow a coordinated vulnerability disclosure timeline:

1. **Acknowledgment:** Within **24 hours** of receipt.
2. **Triage & Validation:** Within **72 hours**, confirming whether the issue is reproducible and in-scope.
3. **Remediation:** A patch is developed, tested across our adversarial test suite, and prepared for release within **14 days** for critical issues.
4. **Coordinated Release:** A security advisory (GHSA) and patch release will be published simultaneously, with public credit attributed to the researcher (unless anonymity is requested).

---

## Safe Harbor

We consider security research conducted under this policy to be authorized. We will not pursue legal action against researchers who:
* Make a good faith effort to avoid privacy violations, destruction of data, and service interruption.
* Give us reasonable time to remediate vulnerabilities before publicly disclosing any details.
* Do not access or modify customer or enterprise data without authorization.
