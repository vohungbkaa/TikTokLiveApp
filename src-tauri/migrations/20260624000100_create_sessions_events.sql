CREATE TABLE live_sessions (
    id TEXT PRIMARY KEY,
    platform TEXT NOT NULL,
    platform_session_id TEXT,
    title TEXT,
    status TEXT NOT NULL,
    started_at DATETIME,
    ended_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE live_events (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    sequence_no INTEGER NOT NULL,
    source TEXT NOT NULL,
    platform_event_id TEXT,
    user_id TEXT,
    unique_id TEXT,
    display_name TEXT,
    comment TEXT,
    ts_platform DATETIME,
    ts_received DATETIME NOT NULL,
    raw_json TEXT,
    FOREIGN KEY(session_id) REFERENCES live_sessions(id) ON DELETE CASCADE
);

CREATE INDEX idx_live_events_session_sequence ON live_events(session_id, sequence_no);
CREATE INDEX idx_live_events_platform_event_id ON live_events(platform_event_id);
