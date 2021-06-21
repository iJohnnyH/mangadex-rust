use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataObject {
    pub id: String,
    pub r#type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub bio: String,
    pub created: String,
    pub updated: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MangaTag {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub group: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MangaAttributes {
    pub title: HashMap<String, String>,
    pub altTitles: Vec<HashMap<String, String>>,
    pub description: HashMap<String, String>,
    // pub author: Author,
    pub links: HashMap<String, String>,
    pub originalLanguage: String,   
    pub lastVolume: Option<String>,
    pub lastChapter: Option<String>,
    pub publicationDemographic: String,
    pub status: String,
    pub year: Option<u32>,
    pub contentRating: String,
    // // pub tags: MangaTag,
    pub createdAt: String,
    pub updatedAt: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manga {
    #[serde(flatten)]
    pub data: DataObject,
    pub attributes: MangaAttributes
}