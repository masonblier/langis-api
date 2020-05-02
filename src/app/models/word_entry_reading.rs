use serde::{Deserialize,Serialize};

use crate::schema::{word_entries,word_entry_readings};
use crate::app::models::word_entry::{WordEntry};

/// WordEntryReading records
#[derive(Associations, Debug, Serialize, Identifiable, Deserialize, Queryable)]
#[belongs_to(WordEntry, foreign_key = "word_entry_id")]
pub struct WordEntryReading {
    pub id: i32,
    pub word_entry_id: i32,
    pub reading: String,
}

/// NewWordEntryReading struct for inserting a new word_entry_readings record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "word_entry_readings"]
pub struct NewWordEntryReading {
    pub word_entry_id: i32,
    pub reading: String,
}

joinable!(word_entry_readings -> word_entries(word_entry_id));
