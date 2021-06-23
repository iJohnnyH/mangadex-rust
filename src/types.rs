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
    pub id: String,
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
    pub year: Option<String>,
    pub contentRating: String,
    // // pub tags: MangaTag,
    pub createdAt: String,
    pub updatedAt: String
}

fn _to_str_map(map: &serde_json::Map<String, Value>) -> HashMap<String, String> {
    let mut strMap = HashMap::new();
    for (key, val) in map.iter() {
        strMap.insert(key.to_string(), val.as_str().unwrap().to_string());
    }
    strMap
}

impl From<Value> for Manga {
    fn from(i: Value) -> Manga {
        let attr = &i["data"]["attributes"];
        let mut altTitles = Vec::new();
        for map in attr["altTitles"].as_array().unwrap().iter() {
            altTitles.push(_to_str_map(map.as_object().unwrap()));
        }
        Manga {
            id: i["data"]["id"].to_string(),
            title: _to_str_map(attr["title"].as_object().unwrap()),
            altTitles: altTitles,
            description: _to_str_map(attr["description"].as_object().unwrap()),
            links: _to_str_map(attr["links"].as_object().unwrap()),
            originalLanguage: attr["originalLanguage"].to_string(),
            lastVolume: Some(attr["lastVolume"].to_string()),
            lastChapter: Some(attr["lastChapter"].to_string()),
            publicationDemographic: attr["publicationDemographic"].to_string(),
            status: attr["status"].to_string(),
            year: Some(attr["year"].to_string()),
            contentRating: attr["contentRating"].to_string(),
            createdAt: attr["createdAt"].to_string(),
            updatedAt: attr["updatedAt"].to_string()
        }
    }
}
