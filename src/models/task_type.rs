use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DatabaseTaskType {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTaskType {
    pub id: i64,
    pub name: String,
    pub description: String,
}

impl From<DatabaseTaskType> for ResponseTaskType {
    fn from(task_type: DatabaseTaskType) -> Self {
        ResponseTaskType {
            id: task_type.id,
            name: task_type.name,
            description: task_type.description,
        }
    }
}
