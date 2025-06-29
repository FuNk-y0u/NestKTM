use mongodb::Client;

pub struct AppState{
    pub version: String,
    pub db: Client
}

