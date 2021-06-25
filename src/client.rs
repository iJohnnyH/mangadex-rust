use serde_json::Value;

use crate::session::{ClientResp, Session};
use crate::types::*;
use crate::BASE_URI;
use log::info;

use std::collections::HashMap;

pub struct MDex {
    pub session: Session,
}

impl MDex {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
        }
    }

    pub async fn mangas(&self, queries: Option<HashMap<String, String>>) -> ClientResp<Vec<Manga>> {
        info!("Fetching mangas");
        let url = format!("{}{}", BASE_URI, "/manga");
        let result = self.session.get(&url, queries).await?;
        let data: Value = result.json::<serde_json::Value>().await?;

        let mut mangas: Vec<Manga> = Vec::new();
        for m in data["results"].as_array().unwrap().iter() {
            mangas.push(Manga::from(m.to_owned()));
        }
        info!("Finished parsing mangas");
        Ok(mangas)
    }

    pub async fn manga(&self, id: String) -> ClientResp<Manga> {
        info!("Fetching manga {}", id);
        let url = format!("{}{}{}", BASE_URI, "/manga/", id);
        let result = self.session.get(&url, None).await?;
        let data: Value = result.json::<serde_json::Value>().await?;
        let manga: Manga = Manga::from(data);
        info!("Finished fetching manga {}", id);
        Ok(manga)
    }
}
