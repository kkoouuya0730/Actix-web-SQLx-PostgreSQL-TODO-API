// Handler層(HTTP)
// リクエスト受付
// レスポンス返却
use axum::{Json, extract::State};
use std::sync::Arc;

use crate::service::todo_service::TodoService;

pub async fn list_todo(
    State(service): State<Arc<TodoService>>,
) -> Json<Vec<crate::domain::todo::Todo>> {
    let todos = service.get_all().await.unwrap();
    Json(todos)
}
