-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    passhash VARCHAR(122) NOT NULL, --argon hash
    created_at TIMESTAMP NOT NULL
);