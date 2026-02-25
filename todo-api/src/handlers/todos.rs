// Handler層(HTTP)
// リクエスト受付
// レスポンス返却
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, extract::Path};
use std::sync::Arc;

use crate::domain::todo::{CreateTodoRequest, Todo, UpdateTodoRequest};
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
) -> Result<Json<Todo>, StatusCode> {
    match service.get_by_id(id).await {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_todo(
    State(service): State<Arc<TodoService>>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let todo = service
        .create(payload.title)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn update_todo_completed(
    State(service): State<Arc<TodoService>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let result = service
        .update_completed(id, payload.completed)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match result {
        Some(todo) => Ok((StatusCode::OK, Json(todo))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_todo(
    Path(id): Path<i32>,
    State(service): State<Arc<TodoService>>,
) -> Result<StatusCode, StatusCode> {
    let deleted = service
        .delete(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
