use diesel::prelude::*;

use crate::models::{NewNotesAndTags, NewSource, NewWordTranslation, Source, WordTranslation};
use crate::schema;

/// finds or creates a sources record citing the dictionary import file
pub fn find_or_create_source<'a>(conn: &PgConnection, source_name: String) -> Source {
    use schema::sources;
    use schema::sources::dsl::*;

    // check for existing result
    let results = sources.filter(name.eq(source_name.clone()))
        .limit(1)
        .load::<Source>(conn)
        .expect("Error checking sources table");
    
    if results.len() < 1 {

        let new_source = NewSource::from_name(source_name);

        diesel::insert_into(sources::table)
            .values(&new_source)
            .get_result(conn)
            .expect("Error saving sources record")

    } else {
        results[0].clone()
    }
}

/// update sources record with last_updated_at date
pub fn update_source<'a>(conn: &PgConnection, source_id: i32) {
    use schema::sources::dsl::*;

    diesel::update(sources.find(source_id))
        .set(last_updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)
        .expect(&format!("Unable to update source {}", source_id));
}

/// writes a word_translation entry to the database table, returning the row id
pub fn insert_word_translations<'a>(conn: &PgConnection, new_entry: NewWordTranslation) -> i32 {
    use schema::word_translations;

    let inserted: WordTranslation = diesel::insert_into(word_translations::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving word_translations record");

    inserted.id
}

/// writes a notes_and_tags entry to the database table
pub fn insert_notes_and_tags<'a>(conn: &PgConnection, word_translation_id: i32, note: String) {
    use schema::notes_and_tags;

    let new_entry = NewNotesAndTags {
        word_translation_id,
        note
    };

    diesel::insert_into(notes_and_tags::table)
        .values(&new_entry)
        .execute(conn)
        .expect("Error saving notes_and_tags record");
}