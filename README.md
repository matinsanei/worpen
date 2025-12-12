# WORPEN // THE DIGITAL NERVOUS SYSTEM

```text
██     ██  ██████  ██████  ██████  ███████ ███    ██ 
██     ██ ██    ██ ██   ██ ██   ██ ██      ████   ██ 
██  █  ██ ██    ██ ██████  ██████  █████   ██ ██  ██ 
██ ███ ██ ██    ██ ██   ██ ██      ██      ██  ██ ██ 
 ███ ███   ██████  ██   ██ ██      ███████ ██   ████ 
```

> **SYSTEM_STATUS:** `ONLINE`  
> **ENCRYPTION:** `AES-256-GCM`  
> **PROTOCOL:** `HIVE_MIND_V2`  
> **CLEARANCE:** `LEVEL 5 (GOD MODE)`

---

## 🌌 IDENTITY_MATRIX

**Worpen** is not just a tool; it is a **living layer** for your infrastructure.

> *"Legacy systems wait for humans to fix them. Worpen fixes itself."*

We replace the bloat of Kubernetes and the fragility of Bash scripts with a **nanosecond-latency** Rust middleware that turns your disconnected servers into a unified, self-healing organism.

---

## 🏗️ SYSTEM ARCHITECTURE [HIVE_TOPOLOGY]

The system operates on a decentralized **Hive & Agent** topology, designed for zero-trust environments and edge computing.

### 🐝 THE AGENT (The Worker)
A hyper-lightweight binary installed on target nodes (VPS, Raspberry Pi, Robotics).
*   **Core:** Rust 🦀 + Tokio
*   **Footprint:** < 15MB RAM
*   **Privileges:** Direct Kernel Ring 0 & `docker.sock` access.
*   **Directive:** "Observe, Report, Heal."

### 🧠 THE CORE (The Brain)
The central command node managing the fleet.
*   **Protocol:** gRPC + mTLS (Streaming bidirectional telemetry).
*   **Role:** Strategic decision making, mass deployment, and pattern recognition.

### 🖥️ THE COCKPIT (The Interface)
You are currently viewing the **Cockpit**. A TUI-styled (Text User Interface) web dashboard for high-efficiency monitoring and manual intervention.

---

## ⚡ ACTIVE MODULES

| MODULE | DESCRIPTION | STATUS |
| :--- | :--- | :--- |
| **DASHBOARD** | High-level topology map & telemetry aggregation. | `ONLINE` |
| **FLEET** | Real-time pressure gauges (CPU/RAM) & remote shell. | `ONLINE` |
| **DOCKER** | Direct container orchestration via **Bollard** crate. | `STABLE` |
| **AUTOMATION** | JavaScript/DSL engine for defining self-healing rules. | `BETA` |

### 🛑 SELF-HEALING SCENARIO EXAMPLE

1.  **EVENT:** Service `saleor-api` crashes with exit code `137` (OOM).
2.  **DETECTION:** Agent captures `docker.die` event in **< 5ms**.
3.  **ANALYSIS:** Local strategy table checked: `IF exit_code == 137 THEN restart`.
4.  **ACTION:** Container restarted immediately.
5.  **REPORT:** Incident logged to Core. **Human intervention: 0**.

---

## 🚀 INITIALIZATION SEQUENCE

Jack into the matrix:

```bash
# 1. Clone the neural pathways
git clone https://github.com/worpen/cockpit.git

# 2. Install synaptic relays
npm install

# 3. Ignite the system
npm start
```

---

> "The system is self-correcting. We are just the architects."

*WORPEN_OS // END OF FILE*