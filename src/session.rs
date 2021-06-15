use reqwest::Client;
use serde_json::Value;

pub type ClientResp<T> = Result<T, Error>

pub struct Session{
    pub refresh_token: Option<String>
    pub client: reqwest::Client 
}

impl Session {
    async fn get(&self, url: &str, headers: Option, body: &Value) -> ClientResp<T>
}