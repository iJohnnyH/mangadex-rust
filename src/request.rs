use reqwest::Client;

pub struct Session{
    pub refresh_token: Option<String>
    pub client: reqwest::Client 
}

impl Session {
    
}