ALTER TABLE customers ADD COLUMN note TEXT;

CREATE TABLE customer_tags (
    customer_id TEXT NOT NULL,
    tag TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY(customer_id, tag),
    FOREIGN KEY(customer_id) REFERENCES customers(id) ON DELETE CASCADE
);

CREATE TABLE customer_risks (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    risk_type TEXT NOT NULL,
    reason TEXT,
    evidence TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(customer_id) REFERENCES customers(id) ON DELETE CASCADE
);

CREATE INDEX idx_customer_risks_customer ON customer_risks(customer_id);
