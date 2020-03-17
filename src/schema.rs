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
    word_translations (id) {
        id -> Int4,
        orth -> Varchar,
        orth_lang -> Varchar,
        quote -> Varchar,
        quote_lang -> Varchar,
        pos -> Nullable<Varchar>,
        sense -> Int4,
        source_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    sessions,
    sources,
    users,
    word_translations,
);
