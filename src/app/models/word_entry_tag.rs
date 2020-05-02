use serde::{Deserialize,Serialize};

use crate::schema::{word_entries,word_entry_tags};
use crate::app::models::word_entry::{WordEntry};

/// WordEntryTag records
#[derive(Associations, Debug, Serialize, Identifiable, Deserialize, Queryable)]
#[belongs_to(WordEntry, foreign_key = "word_entry_id")]
pub struct WordEntryTag {
    pub id: i32,
    pub word_entry_id: i32,
    pub tag: String,
}

/// NewWordEntryTag struct for inserting a new word_entry_tags record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "word_entry_tags"]
pub struct NewWordEntryTag {
    pub word_entry_id: i32,
    pub tag: String,
}

joinable!(word_entry_tags -> word_entries(word_entry_id));
