use diesel::prelude::*;

use crate::app::models::{NewWordEntry, WordEntry, NewWordEntryNote,
    NewWordEntryReading, NewWordEntryTag, NewSource, Source};
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

/// writes a word_entries entry to the database table, returning the row id
pub fn insert_word_entry<'a>(conn: &PgConnection, new_entry: NewWordEntry) -> i32 {
    use schema::word_entries;

    let inserted: WordEntry = diesel::insert_into(word_entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving word_entries record");

    inserted.id
}

/// writes a word_entry_notes entry to the database table
pub fn insert_word_entry_note<'a>(conn: &PgConnection, word_entry_id: i32, note: String) {
    use schema::word_entry_notes;

    let new_record = NewWordEntryNote {word_entry_id, note};

    diesel::insert_into(word_entry_notes::table)
        .values(&new_record)
        .execute(conn)
        .expect("Error saving word_entry_notes record");
}

/// writes a word_entry_readings entry to the database table
pub fn insert_word_entry_reading<'a>(conn: &PgConnection, word_entry_id: i32, reading: String, reading_tag: Option<String>) {
    use schema::word_entry_readings;

    let new_record = NewWordEntryReading {word_entry_id, reading, reading_tag};

    diesel::insert_into(word_entry_readings::table)
        .values(&new_record)
        .execute(conn)
        .expect("Error saving word_entry_readings record");
}

/// writes a word_entry_tags entry to the database table
pub fn insert_word_entry_tag<'a>(conn: &PgConnection, word_entry_id: i32, tag: String) {
    use schema::word_entry_tags;

    let new_record = NewWordEntryTag {word_entry_id, tag};

    diesel::insert_into(word_entry_tags::table)
        .values(&new_record)
        .execute(conn)
        .expect("Error saving word_entry_tags record");
}
