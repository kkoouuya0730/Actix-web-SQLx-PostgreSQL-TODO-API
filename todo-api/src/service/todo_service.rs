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
}
