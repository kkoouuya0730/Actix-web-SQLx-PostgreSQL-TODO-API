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

    async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, sqlx::Error> {
        sqlx::query_as!(
            Todo,
            r#"
            SELECT id, title, completed, created_at
            FROM todos
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create(&self, title: String) -> Result<Todo, anyhow::Error> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (title, completed)
            VALUES ($1, false)
            RETURNING id, title, completed, created_at
            "#,
            title
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    async fn update_completed(
        &self,
        id: i32,
        completed: bool,
    ) -> Result<Option<Todo>, anyhow::Error> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
            UPDATE todos
            SET completed = $1
            WHERE id = $2
            RETURNING id, title, completed, created_at
            "#,
            completed,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(todo)
    }
}
