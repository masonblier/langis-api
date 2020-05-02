use serde::{Deserialize, Serialize};

use crate::security::random_token;
use super::schema::*;

/// User record with all fields
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub passhash: String,
    pub created_at: chrono::NaiveDateTime,
}

/// SlimUser user record with only session-pertinent fields
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub username: String,
}

impl From<User> for SlimUser {
    /// picks pertinent fields from User record
    fn from(user: User) -> Self {
        SlimUser { id: user.id, username: user.username }
    }
}

/// NewUser struct for fields necessary when inserting a new user record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub passhash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl NewUser {
    /// constructor method for NewUser records from registration data
    pub fn from_details<S: Into<String>, T: Into<String>>(username: S, passhash: T) -> Self {
        NewUser {
            username: username.into(),
            passhash: passhash.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}


/// Session records
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "sessions"]
pub struct Session {
    pub token: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl Session {
    /// constructor method generates new Session record objects with unique token
    pub fn create<S: Into<i32>>(user_id: S) -> Self {
        Session {
            token: random_token().unwrap(),
            user_id: user_id.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}


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
