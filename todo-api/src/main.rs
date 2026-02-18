mod handlers;
mod models;
mod repository;

use crate::repository::todo_repository::{PgTodoRepository, TodoRepository};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

struct AppState {
    repo: Arc<dyn TodoRepository>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");
    println!("Connected to DB");

    let repo = PgTodoRepository::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                repo: Arc::new(repo.clone()),
            }))
            .route("/todos", web::get().to(handlers::todos::get_todos))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
