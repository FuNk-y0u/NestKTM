use mongodb::Client;

pub struct AppState{
    pub version: String,
    pub db: Client,
    pub sms_ssid: String,
    pub sms_auth: String

}

