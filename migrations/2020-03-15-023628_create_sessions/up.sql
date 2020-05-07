-- Your SQL goes here
CREATE TABLE sessions (
    token VARCHAR(64) NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    last_accessed_at TIMESTAMPTZ NOT NULL,
    accessed_by_client_ip TEXT
);
