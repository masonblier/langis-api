-- Your SQL goes here
CREATE TABLE sources (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    last_updated_at TIMESTAMP NOT NULL
);
