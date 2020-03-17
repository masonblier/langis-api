-- Your SQL goes here
CREATE TABLE word_translations (
    id SERIAL PRIMARY KEY,
    orth VARCHAR NOT NULL,
    orth_lang VARCHAR NOT NULL,
    quote VARCHAR NOT NULL,
    quote_lang VARCHAR NOT NULL,
    pos VARCHAR,
    sense INTEGER NOT NULL DEFAULT 0,
    source_id INTEGER NOT NULL
);
CREATE INDEX idx_word_translations_orth ON word_translations(orth,orth_lang);
