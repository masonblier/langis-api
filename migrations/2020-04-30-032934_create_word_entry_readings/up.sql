-- Your SQL goes here
CREATE TABLE word_entry_readings (
    id SERIAL PRIMARY KEY,
    word_entry_id INTEGER NOT NULL,
    reading VARCHAR NOT NULL,
    reading_tag VARCHAR
);