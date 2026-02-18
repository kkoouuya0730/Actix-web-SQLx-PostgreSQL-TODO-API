use crate::models::todo::Todo;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
}

#[derive(Clone)]
pub struct PgTodoRepository {
    pool: PgPool,
}

impl PgTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for PgTodoRepository {
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
