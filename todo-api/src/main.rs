use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, routing::get};

use dotenvy::dotenv;
use tokio::net::TcpListener;

use std::env;

mod db;
mod domain;
mod handlers;
mod repository;
mod service;

use handlers::todos::get_todo;
use handlers::todos::list_todo;
use repository::todo_repository::TodoRepository;
use repository::todo_repository_impl::TodoRepositoryImpl;
use service::todo_service::TodoService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env読み込み
    dotenv().ok();
    // 環境変数読み込み
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // DB接続
    let pool = db::new_pool(&database_url).await;
    println!("Connected to DB");
    // Repository生成
    let repo: Arc<dyn TodoRepository> = Arc::new(TodoRepositoryImpl::new(pool));
    // Service生成
    let service = Arc::new(TodoService::new(repo));
    // Router構築
    let app = Router::new()
        .route("/todos", get(list_todo))
        .route("/todos/:id", get(get_todo))
        .with_state(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
