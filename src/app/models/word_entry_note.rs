use serde::{Deserialize,Serialize};

use crate::schema::{word_entries,word_entry_notes};
use crate::app::models::word_entry::{WordEntry};

/// WordEntryNote records
#[derive(Associations, Debug, Serialize, Identifiable, Deserialize, Queryable)]
#[belongs_to(WordEntry, foreign_key = "word_entry_id")]
pub struct WordEntryNote {
    pub id: i32,
    pub word_entry_id: i32,
    pub note: String,
}

/// NewWordEntryNote struct for inserting a new word_entry_notes record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "word_entry_notes"]
pub struct NewWordEntryNote {
    pub word_entry_id: i32,
    pub note: String,
}

joinable!(word_entry_notes -> word_entries(word_entry_id));
