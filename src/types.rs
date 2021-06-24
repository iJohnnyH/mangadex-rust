use serde::{Serialize, Deserialize,};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataObject {
    pub id: String,
    pub r#type: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    #[serde(default = "default_str")]
    pub id: String,
    #[serde(default = "default_str")]
    pub name: String,
    #[serde(default = "default_str")]
    pub image_url: String,
    #[serde(default = "default_str")]
    pub bio: String,
    #[serde(default = "default_str")]
    pub created_at: String,
    #[serde(default = "default_str")]
    pub updated_at: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MangaTag {
    pub id: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    #[serde(flatten)]
    pub id: String,
    pub title: HashMap<String, String>,
    pub alt_titles: Vec<HashMap<String, String>>,
    pub description: HashMap<String, String>,
    pub links: HashMap<String, String>,
    pub original_language: String,
    pub last_volume: String,
    pub last_chapter: String,
    pub publication_demographic: String,
    pub status: String,
    pub year: String,
    pub content_rating: String,
    pub tags: Vec<MangaTag>,
    pub created_at: String,
    pub updated_at: String,
    pub author: Author
}

fn _to_str_map(map: &serde_json::Map<String, Value>) -> HashMap<String, String> {
    let mut str_map = HashMap::new();
    for (key, val) in map.iter() {
        str_map.insert(key.to_string(), val.as_str().unwrap().to_string());
    }
    str_map
}

fn default_str() -> String {
    "".to_string()
}

impl From<Value> for Manga {
    fn from(i: Value) -> Manga {
        let attr = &i["data"]["attributes"];
        let mut alt = Vec::new();
        for map in attr["altTitles"].as_array().unwrap().iter() {
            alt.push(_to_str_map(map.as_object().unwrap()));
        }
        let mut tags = Vec::new();
        for map in attr["tags"].as_array().unwrap().iter() {
            let id = map["id"].as_str().unwrap().to_string();
            let name = map["attributes"]["name"]["en"].as_str().unwrap().to_string();
            tags.push(MangaTag {
                id: id,
                name: name
            })
        }

        let mut author: Author = Author::default();
        for relat in i["relationships"].as_array().unwrap().iter() {
            if relat["type"].as_str().unwrap_or("").to_string() == "author" {
                author = Author {
                    id: relat["id"].as_str().unwrap_or("").to_string(),
                    name: default_str(),
                    image_url: default_str(),
                    bio: default_str(),
                    created_at: default_str(),
                    updated_at: default_str()
                };
            }
        }

        Manga {
            id: i["data"]["id"].as_str().unwrap_or("").to_string(),
            title: _to_str_map(attr["title"].as_object().unwrap()),
            alt_titles: alt,
            description: _to_str_map(attr["description"].as_object().unwrap()),
            links: _to_str_map(attr["links"].as_object().unwrap()),
            original_language: attr["originalLanguage"].as_str().unwrap_or("").to_string(),
            last_volume: attr["lastVolume"].as_str().unwrap_or("").to_string(),
            last_chapter: attr["lastChapter"].as_str().unwrap_or("").to_string(),
            publication_demographic: attr["publicationDemographic"].as_str().unwrap_or("").to_string(),
            status: attr["status"].as_str().unwrap_or("").to_string(),
            year: attr["year"].as_str().unwrap_or("").to_string(),
            content_rating: attr["contentRating"].as_str().unwrap_or("").to_string(),
            tags: tags,
            created_at: attr["createdAt"].as_str().unwrap_or("").to_string(),
            updated_at: attr["updatedAt"].as_str().unwrap_or("").to_string(),
            author: author
        }
    }
}
