use crate::core::appstate::AppState;
use crate::core::signup::Signup;

use mongodb::bson::doc;
use actix_web::{web, Responder, HttpResponse};

pub async fn signup(body: web::Json<Signup>, data: web::Data<AppState>) -> impl Responder {

        let accounts = &data.db.database("nestktmdb").collection("accounts");

        let insert_result =accounts.insert_one(
            doc!{
                "phone": body.phone.clone()
            }
            ).await.unwrap();
       HttpResponse::Ok().body(format!("Your phone is {}", body.phone)) 
}

