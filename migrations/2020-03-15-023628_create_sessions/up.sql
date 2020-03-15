-- Your SQL goes here
CREATE TABLE sessions (
    token VARCHAR(32) NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL
);