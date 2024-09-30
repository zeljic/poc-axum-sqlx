use chrono::{DateTime, Utc};

pub struct DatabaseComment {
    pub id: i64,
    pub task_id: i64,
    pub user_id: i64,
    pub parent_id: Option<i64>,
    pub comment: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub delete_at: Option<DateTime<Utc>>,
}

pub struct ResponseComment {
    pub id: i64,
    pub task_id: i64,
    pub user_id: i64,
    pub parent_id: Option<i64>,
    pub comment: String,
}

impl From<DatabaseComment> for ResponseComment {
    fn from(comment: DatabaseComment) -> Self {
        ResponseComment {
            id: comment.id,
            task_id: comment.task_id,
            user_id: comment.user_id,
            parent_id: comment.parent_id,
            comment: comment.comment,
        }
    }
}
