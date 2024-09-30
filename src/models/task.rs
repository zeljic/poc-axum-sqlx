use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DatabaseTask {
    pub id: i64,
    pub user_id: i64,
    pub status_id: i64,
    pub parent_id: Option<i64>,
    pub task_type_id: i64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: i64,
    pub user_id: i64,
    pub status_id: i64,
    pub parent_id: Option<i64>,
    pub task_type_id: i64,
    pub title: String,
    pub description: String,
}

impl From<DatabaseTask> for ResponseTask {
    fn from(task: DatabaseTask) -> Self {
        ResponseTask {
            id: task.id,
            user_id: task.user_id,
            status_id: task.status_id,
            parent_id: task.parent_id,
            task_type_id: task.task_type_id,
            title: task.title,
            description: task.description,
        }
    }
}
