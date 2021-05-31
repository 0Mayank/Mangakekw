pub mod parser;
mod tests;

use super::utils;
use chrono::{DateTime, FixedOffset};
use parser::*;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Genre {
    id: String,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Theme {
    id: String,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Content {
    id: String,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Format {
    id: String,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Manga {
    pub id: String,
    pub title: Title,
    pub alt_titles: Vec<Title>,
    pub description: String,
    pub original_language: String,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<utils::Demographic>,
    pub status: utils::Status,
    pub year: Option<usize>,
    pub content_rating: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub genres: Vec<Genre>,
    pub themes: Vec<Theme>,
    pub content: Vec<Content>,
    pub format: Vec<Format>,
    pub author_id: String,
    pub artist_id: String,
}

#[allow(dead_code)]
impl Manga {
    pub fn from_response(response: MangaResponse) -> Self {
        let raw = response.data;
        let relations = response.relationships;

        let mut genres = Vec::new();
        let mut themes = Vec::new();
        let mut format = Vec::new();
        let mut content = Vec::new();

        let mut author_id: String = String::new();
        let mut artist_id: String = String::new();

        for relation in relations {
            match relation.r#type {
                utils::RelationshipType::Author => author_id = relation.id.clone(),
                utils::RelationshipType::Artist => artist_id = relation.id.clone(),
                _ => (),
            }
        }

        for tag in raw.attributes.tags {
            match tag.attributes.group {
                utils::TagGroup::Theme => themes.push(Theme {
                    id: raw.id.clone(),
                    name: tag.attributes.name.en.clone(),
                }),
                utils::TagGroup::Content => content.push(Content {
                    id: raw.id.clone(),
                    name: tag.attributes.name.en.clone(),
                }),
                utils::TagGroup::Genre => genres.push(Genre {
                    id: raw.id.clone(),
                    name: tag.attributes.name.en.clone(),
                }),
                utils::TagGroup::Format => format.push(Format {
                    id: raw.id.clone(),
                    name: tag.attributes.name.en.clone(),
                }),
            }
        }
        Manga {
            id: raw.id,
            title: raw.attributes.title,
            alt_titles: raw.attributes.alt_titles,
            description: raw.attributes.description.get("en").unwrap().clone(),
            original_language: raw.attributes.original_language,
            last_volume: raw.attributes.last_volume,
            last_chapter: raw.attributes.last_chapter,
            publication_demographic: raw.attributes.publication_demographic,
            status: raw.attributes.status,
            year: raw.attributes.year,
            content_rating: raw.attributes.content_rating,
            created_at: DateTime::parse_from_rfc3339(&raw.attributes.created_at).unwrap(),
            updated_at: DateTime::parse_from_rfc3339(&raw.attributes.updated_at).unwrap(),
            genres,
            themes,
            format,
            content,
            author_id,
            artist_id,
        }
    }

    pub fn serialize(&self, pretty: bool) -> String {
        if pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }

    #[allow(dead_code)]
    pub fn from_string(string: String) -> Result<Self, serde_json::Error> {
        let response: MangaResponse = serde_json::from_str(&string)?;
        Ok(Self::from_response(response))
    }
}
