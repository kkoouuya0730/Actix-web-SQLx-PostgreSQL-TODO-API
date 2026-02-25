// リポジトリ層(抽象)
// DBアクセスの抽象化
// Serviceが依存する契約(trait)
use crate::domain::todo::Todo;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, sqlx::Error>;
    async fn create(&self, title: String) -> Result<Todo, anyhow::Error>;
    async fn update_completed(
        &self,
        id: i32,
        completed: bool,
    ) -> Result<Option<Todo>, anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<bool, anyhow::Error>;
}
