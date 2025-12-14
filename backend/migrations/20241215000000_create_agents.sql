CREATE TABLE agents (
    id TEXT PRIMARY KEY NOT NULL,
    hostname TEXT NOT NULL,
    ip_address TEXT,
    status TEXT NOT NULL,
    last_heartbeat DATETIME NOT NULL,
    version TEXT NOT NULL,
    metadata JSON
);
