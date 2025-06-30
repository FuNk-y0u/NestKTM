use crate::core::appstate::AppState;
use crate::core::signup::Signup;

use mongodb::bson::doc;
use actix_web::{web, Responder, HttpResponse};

use std::collections::HashMap;

use reqwest::Client;

pub async fn signup(body: web::Json<Signup>, data: web::Data<AppState>) -> impl Responder {
        let client = Client::new();

        let mut form = HashMap::new();
        form.insert("To", format!("+977{0}", body.phone));
        form.insert("Channel", "sms".to_string());

        let result = client.post("https://verify.twilio.com/v2/Services/VAd49684c4d0557955e6e46671fe589bc7/Verifications")
        .basic_auth(&data.sms_ssid,Some(&data.sms_auth))
        .form(&form)
        .send()
        .await;

       // let accounts = &data.db.database("nestktmdb").collection("accounts");

        //let insert_result =accounts.insert_one(
        //    doc!{
        //        "phone": body.phone.clone()
        //    }
        //    ).await.unwrap();
       HttpResponse::Ok().body(format!("Verification code has been sent to {}", body.phone)) 
}

