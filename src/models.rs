use serde::{Deserialize, Serialize};

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
    pub username: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { username: user.username }
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
