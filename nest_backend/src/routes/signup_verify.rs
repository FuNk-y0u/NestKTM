/*
pub async fn signup_verify(body: web::Json<Signup>, data: web::Data<AppState>) -> impl Responder {
    let client = Client::new();

    let mut form = HashMap::new();
    form.insert("To",format!("+977{0}", body.phone));
    form.insert("Code",body.code);

    let result = client.post("https://verify.twilio.com/v2/Services/VAd49684c4d0557955e6e46671fe589bc7/VerificationCheck")
        .basic_auth(&data.sms_ssid, Some(&data.sms_auth))
        .form(&form)
        .send()
        .await;


}
*/
