// use serde::Deserialize;
// use serde_json::map::Map;
// use serde_json::{json, Value};

use crate::types::Manga;
use crate::session::{Session, ClientResp};
use crate::{BASE_URI};

pub struct MDex {
    pub session: Session
}

impl MDex {
    pub async fn mangas<>(&self) -> ClientResp<Vec<Manga>> {
        let url = format!("{}{}", BASE_URI, "/manga");
        let result = self.session.get(&url).await?;
        let mangas: Vec<Manga> = serde_json::from_str(&result).unwrap();
        println!("Mangas: {:?}", mangas);
        Ok(mangas)
    }
}
