-- Your SQL goes here
CREATE TABLE word_entries (
    id SERIAL PRIMARY KEY,
    orth VARCHAR NOT NULL,
    orth_lang VARCHAR NOT NULL,
    quote VARCHAR NOT NULL,
    quote_lang VARCHAR NOT NULL,
    sense INTEGER NOT NULL DEFAULT 0,
    group_id INTEGER NOT NULL
);
CREATE TABLE word_entry_group (
    id SERIAL PRIMARY KEY,
    source_id INTEGER NOT NULL
);
CREATE INDEX idx_word_entries_orth ON word_entries(orth,orth_lang);
CREATE INDEX idx_word_entries_group ON word_entries(group_id);
