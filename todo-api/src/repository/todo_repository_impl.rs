// リポジトリ層(実装)
// sqlxを使った具体実装
// DB依存コードはここに閉じ込める
use crate::{domain::todo::Todo, repository::todo_repository::TodoRepository};
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pool: PgPool,
}

impl TodoRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, title, completed, created_at
            FROM todos
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }
}
