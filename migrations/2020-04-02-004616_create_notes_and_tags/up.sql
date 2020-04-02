-- Your SQL goes here
CREATE TABLE notes_and_tags (
    id SERIAL PRIMARY KEY,
    word_translation_id INTEGER NOT NULL,
    note VARCHAR NOT NULL
);
