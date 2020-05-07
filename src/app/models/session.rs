use chrono::{DateTime,Utc};
use serde::{Deserialize,Serialize};

use crate::schema::*;
use crate::app::security::random_token;
use super::user::User;

/// Session records
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Associations, PartialEq)]
#[belongs_to(User)]
#[table_name = "sessions"]
pub struct Session {
    pub token: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub accessed_by_client_ip: Option<String>,
}

impl Session {
    /// constructor method generates new Session record objects with unique token
    pub fn create<S: Into<i32>>(user_id: S) -> Self {
        Session {
            token: random_token().unwrap(),
            user_id: user_id.into(),
            created_at: Utc::now(),
            last_accessed_at: Utc::now(),
            accessed_by_client_ip: None,
        }
    }
}
