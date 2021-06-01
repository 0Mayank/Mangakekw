pub mod parser;
mod tests;

use super::utils;
use chrono::{DateTime, FixedOffset};
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Chapter {
    id: String,
    title: String,
    volume: Option<String>,
    chapter: Option<String>,
    translated_language: String,
    hash: String,
    data: Vec<String>,
    data_saver: Vec<String>,
    manga_id: String,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    published_at: DateTime<FixedOffset>,
}

impl utils::DexWrappedObject for Chapter {
    type Response = ChapterResponse;

    #[allow(dead_code)]
    fn from_response(response: Self::Response) -> Self {
        let mut manga_id = String::new();
        for relation in response.relationships {
            match relation.r#type {
                utils::RelationshipType::Manga => manga_id = relation.id,
                _ => (),
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
