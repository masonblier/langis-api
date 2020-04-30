table! {
    sessions (token) {
        token -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    sources (id) {
        id -> Int4,
        name -> Varchar,
        last_updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        passhash -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    word_entries (id) {
        id -> Int4,
        orth -> Varchar,
        orth_lang -> Varchar,
        quote -> Varchar,
        quote_lang -> Varchar,
        sense -> Int4,
        source_id -> Int4,
    }
}

table! {
    word_entry_notes (id) {
        id -> Int4,
        word_entry_id -> Int4,
        note -> Varchar,
    }
}

table! {
    word_entry_readings (id) {
        id -> Int4,
        word_entry_id -> Int4,
        reading -> Varchar,
    }
}

table! {
    word_entry_tags (id) {
        id -> Int4,
        word_entry_id -> Int4,
        tag -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    sessions,
    sources,
    users,
    word_entries,
    word_entry_notes,
    word_entry_readings,
    word_entry_tags,
);
