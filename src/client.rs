// use serde::Deserialize;
// use serde_json::map::Map;
use serde_json::{Value};

use crate::types::*;
use crate::session::{Session, ClientResp};
use crate::{BASE_URI};

pub struct MDex {
    pub session: Session
}

impl MDex {
    pub fn new() -> Self {
        Self {
            session: Session::new()
        }
    }

    pub async fn mangas<>(&self) -> ClientResp<Vec<Manga>> {
        let url = format!("{}{}", BASE_URI, "/manga");
        let result = self.session.get(&url).await?;
        let data: Value = result.json::<serde_json::Value>().await?.to_owned();
        println!("{}", data);
        let mut mangas: Vec<Manga> = Vec::new();
        for m in data["results"].as_array().unwrap().iter() {
            mangas.push(Manga::from(m.to_owned()));
        }
        Ok(mangas)
    }

    pub async fn manga<>(&self, id: String) ->ClientResp<Manga> {
        let url = format!("{}{}{}", BASE_URI, "/manga/", id);
        let result = self.session.get(&url).await?;
        let data: Value = result.json::<serde_json::Value>().await?.to_owned();
        let manga: Manga = Manga::from(data);
        Ok(manga)
    }
}
