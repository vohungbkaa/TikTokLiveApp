CREATE TABLE session_products (
    session_id TEXT NOT NULL,
    product_id TEXT NOT NULL,
    initial_stock_qty INTEGER NOT NULL,
    current_stock_qty INTEGER NOT NULL,
    price REAL NOT NULL,
    selling_mode TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (session_id, product_id),
    FOREIGN KEY(session_id) REFERENCES live_sessions(id) ON DELETE CASCADE,
    FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE RESTRICT
);
