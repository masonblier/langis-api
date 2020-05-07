use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};

use crate::schema::*;

/// User record with all fields
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub passhash: String,
    pub created_at: DateTime<Utc>,
}

/// SlimUser user record with only session-pertinent fields
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub name: String,
}

impl From<User> for SlimUser {
    /// picks pertinent fields from User record
    fn from(user: User) -> Self {
        SlimUser { id: user.id, name: user.name }
    }
}

/// NewUser struct for fields necessary when inserting a new user record
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub passhash: String,
    pub created_at: DateTime<Utc>,
}

impl NewUser {
    /// constructor method for NewUser records from registration data
    pub fn from_details<S: Into<String>, T: Into<String>>(name: S, passhash: T) -> Self {
        NewUser {
            name: name.into(),
            passhash: passhash.into(),
            created_at: Utc::now(),
        }
    }
}
