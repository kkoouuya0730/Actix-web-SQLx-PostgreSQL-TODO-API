use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

struct AppState {
    pool: PgPool,
}

async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&data.pool)
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("DB OK: {}", row.0))
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .route("/health", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
