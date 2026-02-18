use crate::AppState;
use actix_web::{HttpResponse, Responder, web};

pub async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let result = data.repo.find_all().await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
