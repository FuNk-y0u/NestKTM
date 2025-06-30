use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};

use std::env;

mod core;
use core::appstate::AppState;

mod routes;
use routes::index::index;
use routes::signup::signup;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("Server starting at 127.0.0.1:8000");
    
    // CONFIG STUFF
    let client_uri = env::var("MONGODB_URI").expect("[ENV] Mongodb uri not set in env");
    let ip = env::var("ADDRESS").expect("[ENV] Server address not set in env");
    let port = env::var("PORT").expect("[ENV] Server port not set in env");
    let twillo_ssid = env::var("TWILLO_SSID").expect("[ENV] twillo ssid not set in env");
    let twillo_auth = env::var("TWILLO_AUTH").expect("[ENV] twillo auth not set in env");

    // Mongodb client creation
    let options = ClientOptions::parse(&client_uri).await.expect("[MDB] Unable to parse client uri");
    let client: Client = Client::with_options(options).expect("[MDB] Unable to create client");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new( AppState
                    {
                        version: String::from("v0.0"),
                        db: client.clone(),
                        sms_ssid: twillo_ssid.clone(),
                        sms_auth: twillo_auth.clone()
                    }
                ))
            .route("/", web::get().to(index))
            .route("/signup", web::post().to(signup))

    })
    .bind((ip, port.parse().expect("[ENV] Unable to parse port number")))?
    .run()
    .await
}
