use serde::{Deserialize,Serialize};

use crate::schema::word_entry_groups;

// WordEntryGroup model
#[derive(Debug, Deserialize, Serialize, Identifiable, Queryable, AsChangeset, Associations)]
#[table_name="word_entry_groups"]
pub struct WordEntryGroup {
    pub id: i32,
    pub source_id: i32,
}

/// NewWordEntryGroup struct for inserting a new word_entry_groups record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "word_entry_groups"]
pub struct NewWordEntryGroup {
    pub source_id: i32
}
