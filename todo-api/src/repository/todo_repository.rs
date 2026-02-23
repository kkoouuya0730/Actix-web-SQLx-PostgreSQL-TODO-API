// リポジトリ層(抽象)
// DBアクセスの抽象化
// Serviceが依存する契約(trait)
use crate::domain::todo::Todo;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
}
