use serde::{Deserialize,Serialize};

use crate::schema::word_entries;

// reference item model
#[derive(Debug, Deserialize, Serialize, Identifiable, Queryable, AsChangeset, Associations)]
#[table_name="word_entries"]
pub struct WordEntry {
    pub id: i32,
    pub orth: String,
    pub orth_lang: String,
    pub quote: String,
    pub quote_lang: String,
    pub sense: i32,
    pub group_id: i32,
}

/// NewWordEntry struct for inserting a new word_entries record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "word_entries"]
pub struct NewWordEntry {
    pub orth: String,
    pub orth_lang: String,
    pub quote: String,
    pub quote_lang: String,
    pub sense: i32,
    pub group_id: i32
}
