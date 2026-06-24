CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE connector_health_logs (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    connector_type TEXT NOT NULL,
    status TEXT NOT NULL,
    error_message TEXT,
    logged_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(session_id) REFERENCES live_sessions(id) ON DELETE CASCADE
);
