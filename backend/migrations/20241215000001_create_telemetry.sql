CREATE TABLE telemetry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id TEXT NOT NULL,
    cpu_usage REAL NOT NULL,
    memory_usage INTEGER NOT NULL,
    total_memory INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    FOREIGN KEY(agent_id) REFERENCES agents(id)
);
