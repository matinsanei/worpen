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

-- Create Orders Table (for Dynamic Routes testing)
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    status TEXT NOT NULL,
    total_amount REAL NOT NULL,
    last_processed_at TEXT,
    created_at TEXT NOT NULL
);

-- Create Order Items Table
CREATE TABLE IF NOT EXISTS order_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id)
);

-- Create Inventory Table
CREATE TABLE IF NOT EXISTS inventory (
    product_id TEXT PRIMARY KEY NOT NULL,
    available_quantity INTEGER NOT NULL,
    reserved_quantity INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL
);

-- Create Error Logs Table
CREATE TABLE IF NOT EXISTS error_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id TEXT,
    error_message TEXT NOT NULL,
    error_code TEXT,
    created_at TEXT NOT NULL
);

-- Indexes for new tables
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_error_logs_order_id ON error_logs(order_id);
