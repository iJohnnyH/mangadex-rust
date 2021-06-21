// use serde::Deserialize;
// use serde_json::map::Map;
// use serde_json::{json, Value};

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

    // pub async fn mangas<>(&self) -> ClientResp<Vec<Manga>> {
    //     let url = format!("{}{}", BASE_URI, "/manga");
    //     let result = self.session.get(&url).await?;
    //     println!("{}", result);
    //     let mangas: Vec<Manga> = serde_json::from_str(&result).unwrap();
    //     println!("Mangas: {:?}", mangas);
    //     Ok(mangas)
    // }

    pub async fn manga<>(&self, id: String) ->ClientResp<Manga> {
        let url = format!("{}{}{}", BASE_URI, "/manga/", id);
        let result = self.session.get(&url).await?;
        let data: String = result.json::<serde_json::Value>().await?["data"].to_string();
        let m: Manga = serde_json::from_str(&data).unwrap();
        Ok(m)
    }
}
