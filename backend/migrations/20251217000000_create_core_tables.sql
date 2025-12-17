-- Create Incidents Table
CREATE TABLE IF NOT EXISTS incidents (
    id TEXT PRIMARY KEY NOT NULL,
    service TEXT NOT NULL,
    node TEXT NOT NULL,
    issue TEXT NOT NULL,
    action TEXT NOT NULL,
    status TEXT NOT NULL,
    time TEXT NOT NULL
);

-- Create Automation Rules Table
CREATE TABLE IF NOT EXISTS automation_rules (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    trigger_event TEXT NOT NULL,
    target_service TEXT NOT NULL,
    script TEXT NOT NULL,
    last_run TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 0
);

-- Create Logs Table
CREATE TABLE IF NOT EXISTS logs (
    id TEXT PRIMARY KEY NOT NULL,
    timestamp TEXT NOT NULL,
    level TEXT NOT NULL,
    source TEXT NOT NULL,
    message TEXT NOT NULL
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_incidents_status ON incidents(status);
CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs(timestamp);
