use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DatabaseStatus {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseStatus {
    pub id: i64,
    pub name: String,
    pub description: String,
}

impl From<DatabaseStatus> for ResponseStatus {
    fn from(status: DatabaseStatus) -> Self {
        ResponseStatus {
            id: status.id,
            name: status.name,
            description: status.description,
        }
    }
}
