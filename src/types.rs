use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub image_url: Something,
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
pub struct Manga {
    pub id: u32
    pub title: String,
    pub alt_title: Vec<String>,
    pub desc: String,
    pub is_locked: bool,
    pub links: String,
    pub original_lang: String,   
    pub language: String,
    pub last_vol: String,
    pub last_chap: String,
    pub pub_dem: String,
    pub status: String,
    pub year: u32,
    pub rating: String,
    pub tags: MangaTag,
    pub version: u32,
    pub created: String,
    pub updated: String
}