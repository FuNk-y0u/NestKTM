use actix_web::{web, Responder, HttpResponse};

use crate::core::appstate::AppState;

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let version = &data.version;

    HttpResponse::Ok().body(
        format!("Nest_Backend {version}")
    )
}
