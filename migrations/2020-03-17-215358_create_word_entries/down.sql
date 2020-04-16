-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_word_entries_orth;
DROP TABLE IF EXISTS word_entries;
