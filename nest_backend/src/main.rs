use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState{
    version: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let version = &data.version;
    HttpResponse::Ok().body(format!("Nest_Backend {version}"))
}

async fn app() -> impl Responder {
    HttpResponse::Ok().body("App main")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at 127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new( AppState
                    {version: String::from("v0.0")}
                ))
            .service(index)
            .service(
                web::scope("/app")
                .route("/index", web::get().to(app))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
