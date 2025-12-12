# WORPEN // THE DIGITAL NERVOUS SYSTEM

```text
 __      __  ____  _______   _______   ______  _____  ___   
|" \    /  \/  " \/    "  \ /"      \ /" _  "\(("   \|"  \  
\   \  //   \  (:   \___/ |:        |(: ( \___)|.\\   \    | 
 \\  \/.    //  \     \   |_____/   ) \/ \     |: \.   \\  | 
  \.        //   //  ___  \  //      /  //  \ _  |.  \    \. | 
   \   \   /    (:  (   ) :)|:  __   \ (:   _) \ |    \    \ | 
    \__//__\     \__|   |__||__|  \___) \_______) \___|\____\) 
```

> **CODENAME:** THE DIGITAL NERVOUS SYSTEM  
> **STATUS:** BETA  
> **CLEARANCE:** LEVEL 5 (GOD MODE)

---

## 🌌 MISSION: SELF-HEALING INFRASTRUCTURE

**Worpen** is a distributed orchestration and **Self-Healing** middleware designed for modern infrastructure. It solves the complexity of managing large fleets and edge devices by introducing a lightweight, intelligent layer that repairs problems automatically.

**The Philosophy:**
Legacy tools (Kubernetes) are too heavy for the Edge. Simple scripts (Bash) are too dumb. Worpen is the "Digital Nervous System" that connects, monitors, and heals your fleet with nanosecond efficiency.

## 🏗️ SYSTEM ARCHITECTURE [HIVE_MIND]

The system operates on a **Hive & Agent** topology, built for speed and resilience.

### 🐝 THE AGENT (The Worker)
A hyper-lightweight binary (written in **Rust**) installed on target nodes (VPS, Raspberry Pi, Robotics).
- **Tech:** Rust, Tokio, Bollard.
- **Access:** Direct Linux Kernel & `docker.sock` access.
- **Role:** Listens for events, executes strategies locally.

### 🧠 THE CORE (The Brain)
The central server managing thousands of agents.
- **Tech:** gRPC + mTLS.
- **Role:** Strategic decision making, mass deployment, telemetry aggregation.

### 🖥️ THE COCKPIT (The Interface)
You are currently viewing the **Cockpit**. A TUI-styled (Text User Interface) web dashboard for high-efficiency monitoring and manual intervention.

---

## ⚡ MODULES & CAPABILITIES

### 1. FLEET_COMMAND
Real-time visibility into every node in the mesh.
- Live CPU/Memory pressure gauges.
- Instant connection status (Online, Healing, Critical).
- Remote trigger capabilities (Restart, SSH).

### 2. DOCKER_ORCHESTRATION
Direct control over the container runtime without CLI latency.
- View real-time container states.
- Perform lifecycle actions: `STOP`, `START`, `KILL`, `PRUNE`.
- Powered by the **Bollard** Rust crate for direct API interaction.

### 3. AUTOMATION_MATRIX (Self-Healing)
The core value proposition. Define logic to handle incidents without human intervention.
- **Trigger:** `CONTAINER_OOM_KILLED`
- **Action:** `docker.restart(id)` + `notify.slack()`
- **Result:** System uptime maintained automatically.

---

## 🎨 AESTHETICS & UX

Designed for operators who live in the terminal.
- **Visual Style:** Retro DOS / Green Phosphor / Cyberpunk.
- **Effects:** CRT Scanlines, Text Glow, Blinking Cursors.
- **Font:** `Fira Code` & `VT323`.
- **Palette:** `#00ff41` (Terminal Green) on `#000000` (Void Black).

## 🚀 INSTALLATION

```bash
# Clone the neural pathway
git clone https://github.com/worpen/cockpit.git

# Initialize dependencies
npm install

# Jack into the matrix
npm start
```

---
*SYSTEM STATUS: NOMINAL // END OF FILE*