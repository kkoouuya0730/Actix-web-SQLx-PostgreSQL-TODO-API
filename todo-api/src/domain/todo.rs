// ドメイン層
// ビジネスの中心モデル
// DBやHTTPに依存しない

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub completed: bool,
}
