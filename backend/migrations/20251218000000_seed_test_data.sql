-- Seed test data for Dynamic Routes testing

-- Insert test orders
INSERT INTO orders (id, user_id, status, total_amount, created_at) VALUES 
('ORD-001', 'USER-123', 'pending', 150.50, datetime('now')),
('ORD-002', 'USER-456', 'completed', 299.99, datetime('now', '-1 day')),
('ORD-003', 'USER-789', 'processing', 75.25, datetime('now', '-2 hours'));

-- Insert test order items
INSERT INTO order_items (order_id, product_id, quantity, unit_price, created_at) VALUES
('ORD-001', 'PROD-A', 2, 50.25, datetime('now')),
('ORD-001', 'PROD-B', 1, 50.00, datetime('now')),
('ORD-002', 'PROD-C', 3, 99.99, datetime('now', '-1 day')),
('ORD-003', 'PROD-A', 1, 50.25, datetime('now', '-2 hours')),
('ORD-003', 'PROD-D', 1, 25.00, datetime('now', '-2 hours'));

-- Insert test inventory
INSERT INTO inventory (product_id, available_quantity, reserved_quantity, updated_at) VALUES
('PROD-A', 100, 3, datetime('now')),
('PROD-B', 50, 1, datetime('now')),
('PROD-C', 200, 3, datetime('now')),
('PROD-D', 75, 1, datetime('now'));
