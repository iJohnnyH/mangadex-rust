use log::error;
use reqwest::{Client, Error, Response, StatusCode};
use thiserror::Error;

use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Reqwest Error")]
    Error(#[from] reqwest::Error),

    #[error("Error parsing JSON")]
    ParseJson(#[from] serde_json::Error),

    #[error("Status code {0}")]
    MangaDexError(StatusCode, Response, reqwest::Error),
}

impl ClientError {
    pub async fn from_resp(resp: reqwest::Response, err: Error) -> Self {
        let status = resp.status();
        match status {
            status => Self::MangaDexError(status, resp, err),
        }
    }
}

pub type ClientResp<T> = Result<T, ClientError>;

pub struct Session {
    pub refresh_token: Option<String>,
    pub client: reqwest::Client,
}

impl Session {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            refresh_token: None,
        }
    }

    pub async fn get(
        &self,
        url: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<Response, ClientError> {
        let mut req = self.client.get(url);
        match params {
            Some(queries) => {
                for (k, v) in queries.iter() {
                    req = req.query(&[(k, v)]);
                }
            }
            _ => (),
        }
        // If refresh token is set, add it to the headers

        let resp = req.send().await?;
        if resp.status().is_success() {
            Ok(resp)
        } else {
            let err = resp.error_for_status_ref().err().unwrap();
            Err(ClientError::from_resp(resp, err).await)
        }
    }
}
