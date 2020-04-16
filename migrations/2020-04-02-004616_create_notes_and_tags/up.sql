-- Your SQL goes here
CREATE TABLE word_entry_notes (
    id SERIAL PRIMARY KEY,
    word_entry_id INTEGER NOT NULL,
    note VARCHAR NOT NULL
);
CREATE TABLE word_entry_tags (
    id SERIAL PRIMARY KEY,
    word_entry_id INTEGER NOT NULL,
    tag VARCHAR NOT NULL
);
