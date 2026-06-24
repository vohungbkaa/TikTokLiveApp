CREATE TABLE customers (
    id TEXT PRIMARY KEY,
    platform_user_id TEXT,
    unique_id TEXT,
    display_name TEXT,
    phone TEXT,
    address TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE order_claims (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    sku TEXT NOT NULL,
    variant_id TEXT,
    qty INTEGER NOT NULL,
    confidence REAL,
    status TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES live_sessions(id) ON DELETE CASCADE,
    FOREIGN KEY(event_id) REFERENCES live_events(id) ON DELETE CASCADE,
    FOREIGN KEY(customer_id) REFERENCES customers(id) ON DELETE RESTRICT
);

CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    status TEXT NOT NULL,
    total_amount REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES live_sessions(id) ON DELETE CASCADE,
    FOREIGN KEY(customer_id) REFERENCES customers(id) ON DELETE RESTRICT
);

CREATE TABLE order_items (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL,
    claim_id TEXT,
    sku TEXT NOT NULL,
    variant_id TEXT,
    qty INTEGER NOT NULL,
    price REAL NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(order_id) REFERENCES orders(id) ON DELETE CASCADE,
    FOREIGN KEY(claim_id) REFERENCES order_claims(id) ON DELETE SET NULL
);

CREATE INDEX idx_order_claims_session_sku ON order_claims(session_id, sku);
CREATE INDEX idx_orders_session_customer ON orders(session_id, customer_id);
