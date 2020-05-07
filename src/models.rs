use serde::{Deserialize, Serialize};

use super::schema::*;


/// Sources records
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub last_updated_at: chrono::NaiveDateTime,
}

/// NewSource struct for inserting a new sources record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "sources"]
pub struct NewSource {
    pub name: String,
    pub last_updated_at: chrono::NaiveDateTime,
}

impl NewSource {
    /// constructor method for sources records
    pub fn from_name<S: Into<String>>(name: S) -> Self {
        NewSource {
            name: name.into(),
            last_updated_at: chrono::Local::now().naive_local(),
        }
    }
}
