// Service層
use crate::domain::todo::Todo;
use crate::repository::todo_repository::TodoRepository;
use std::sync::Arc;

pub struct TodoService {
    repo: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(repo: Arc<dyn TodoRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Todo>, sqlx::Error> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(&self, title: String) -> Result<Todo, anyhow::Error> {
        if title.trim().is_empty() {
            anyhow::bail!("title must not be empty");
        }

        self.repo.create(title).await
    }
}
