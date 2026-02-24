// Handler層(HTTP)
// リクエスト受付
// レスポンス返却
use axum::extract::State;
use axum::{Json, extract::Path};
use std::sync::Arc;

use crate::domain::todo::Todo;
use crate::service::todo_service::TodoService;

pub async fn list_todo(
    State(service): State<Arc<TodoService>>,
) -> Json<Vec<crate::domain::todo::Todo>> {
    let todos = service.get_all().await.unwrap();
    Json(todos)
}

pub async fn get_todo(
    State(service): State<Arc<TodoService>>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, axum::http::StatusCode> {
    match service.get_by_id(id).await {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(axum::http::StatusCode::NOT_FOUND),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
