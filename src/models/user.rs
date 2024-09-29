use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DatabaseUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl From<DatabaseUser> for ResponseUser {
    fn from(user: DatabaseUser) -> Self {
        ResponseUser {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
