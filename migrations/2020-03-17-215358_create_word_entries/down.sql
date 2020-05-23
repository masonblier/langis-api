-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_word_entries_orth;
DROP INDEX IF EXISTS idx_word_entries_group;
DROP TABLE IF EXISTS word_entries;
DROP TABLE IF EXISTS word_entry_groups;
