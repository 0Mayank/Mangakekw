pub mod parser;
pub mod request;
mod tests;

use super::utils;
use chrono::{DateTime, FixedOffset};
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub translated_language: String,
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
    pub manga_id: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub published_at: DateTime<FixedOffset>,
}

impl utils::DexWrappedObject for Chapter {
    type Parser = ChapterResponse;

    #[allow(dead_code)]
    fn from_response(response: Self::Parser) -> Self {
        let mut manga_id = String::new();
        for relation in response.relationships {
            if let utils::RelationshipType::Manga = relation.r#type {
                manga_id = relation.id
            }
        }
        Chapter {
            id: response.data.id,
            title: response.data.attributes.title,
            volume: response.data.attributes.volume,
            chapter: response.data.attributes.chapter,
            translated_language: response.data.attributes.translated_language,
            hash: response.data.attributes.hash,
            data: response.data.attributes.data,
            data_saver: response.data.attributes.data_saver,
            created_at: DateTime::parse_from_rfc3339(&response.data.attributes.created_at).unwrap(),
            updated_at: DateTime::parse_from_rfc3339(&response.data.attributes.updated_at).unwrap(),
            published_at: DateTime::parse_from_rfc3339(&response.data.attributes.published_at)
                .unwrap(),
            manga_id,
        }
    }
}