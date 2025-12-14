# 🧠 𝐂𝐎𝐑𝐄_𝐁𝐑𝐀𝐈𝐍 [𝐁𝐀𝐂𝐊𝐄𝐍𝐃]

> *"The synaptic center of the Worpen nervous system."*

The **Backend** is the central nervous system of Worpen, implemented as a high-performance **Rust** workspace. It orchestrates the Hive, processes telemetry, and serves the Cockpit UI.

---

## 🏗️ 𝐀𝐑𝐂𝐇𝐈𝐓𝐄𝐂𝐓𝐔𝐑𝐄 [𝐖𝐎𝐑𝐊𝐒𝐏𝐀𝐂𝐄]

The backend follows a modular **Clean Architecture** approach using Rust workspaces:

| CRATE | PATH | TYPE | DESCRIPTION |
| :--- | :--- | :--- | :--- |
| **API** | [`crates/api`](crates/api) | `Entry Point` | The neuron interface. Exposes HTTP/REST/gRPC endpoints (Axum) for the Cockpit and external systems. |
| **CORE** | [`crates/core`](crates/core) | `Domain` | Pure business logic and domain entities. The "thought" processing center. Zero external dependencies where possible. |
| **INFRA** | [`crates/infra`](crates/infra) | `Infrastructure` | Database connections, external adapters, and system I/O. The physical link to the digital world. |
| **PROTO** | [`crates/proto`](crates/proto) | `Shared` | Protocol definitions, shared types, and DTOs. The "language" of the Hive. |

```mermaid
graph TD
    API[🔌 API LAYER] --> Service[⚙️ SERVICE LAYER]
    Service --> Core[🧠 CORE DOMAIN]
    Service --> Repo[🗄️ REPOSITORY PORT]
    
    subgraph INFRA [🏗️ INFRA LAYER]
        Sqlite[💾 SQLITE ADAPTER] -- implements --> Repo
    end
    
    API --> Sqlite
    
    subgraph SHARED [📦 SHARED RESOURCES]
        Proto[📝 PROTO DEFS]
    end
    
    API -.-> Proto
    Core -.-> Proto
    Infra -.-> Proto
```

### Key Components

*   **Service Layer (`AgentService`)**: Encapsulates business logic, ensuring the API doesn't talk directly to the database.
*   **Repository Pattern**: Abstracts data storage. We switched from Memory to **SQLite** without touching Core logic.
*   **Database**: Uses `sqlx` with automatic migrations on startup (`worpen.db`).

---

## 🔋 𝐒𝐘𝐒𝐓𝐄𝐌_𝐒𝐓𝐀𝐓𝐔𝐒

[![Rust](https://img.shields.io/badge/RUST-1.75+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/FRAMEWORK-AXUM-red?style=for-the-badge&logo=rust)](https://github.com/tokio-rs/axum)
[![Tokio](https://img.shields.io/badge/RUNTIME-TOKIO-blue?style=for-the-badge&logo=rust)](https://tokio.rs/)

---

## 🚀 𝐈𝐍𝐈𝐓𝐈𝐀𝐋𝐈𝐙𝐀𝐓𝐈𝐎𝐍

To ignite the Core Brain, ensure you have the **Rust Toolchain** installed.

### 1. 🔍 Verification
Run a neural link check across all modules:

```bash
cargo check
```

### 2. ⚡ Ignition
Start the API server (Core Brain):

```bash
cargo run -p api
```

The system will come online at:
> `http://127.0.0.1:3000`

### 3. 🩺 Diagnostics
Ping the health status to verify neural pathways:

```bash
curl http://127.0.0.1:3000/health
```
> Response: `OK`

---

## 🛠️ 𝐃𝐄𝐕𝐄𝐋𝐎𝐏𝐌𝐄𝐍𝐓

### Adding a New Dependency
To inject a new capability into a specific module (e.g., adding `serde` to `core`):

```bash
cargo add serde -p core --features derive
```

### Running Tests
Execute the comprehensive test suite:

```bash
cargo test --workspace
```

---

<div align="center">
  <sub>Authorized Personnel Only. Worpen Corp.</sub>
</div>
