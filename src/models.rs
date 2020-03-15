use serde::{Deserialize, Serialize};

use crate::security::random_token;
use super::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub passhash: String,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub username: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { id: user.id, username: user.username }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub passhash: String,
    pub created_at: chrono::NaiveDateTime,
}

impl NewUser {
    pub fn from_details<S: Into<String>, T: Into<String>>(username: S, passhash: T) -> Self {
        NewUser {
            username: username.into(),
            passhash: passhash.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "sessions"]
pub struct Session {
    pub token: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl Session {
    pub fn create<S: Into<i32>>(user_id: S) -> Self {
        Session {
            token: random_token().unwrap(),
            user_id: user_id.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}