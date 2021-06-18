pub mod parser;
pub mod tests;

use super::utils;
use chrono::{DateTime, FixedOffset};
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Cover {
    pub id: String,
    pub volume: Option<String>,
    pub file_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub manga_id: String,
}

impl utils::DexWrappedObject for Cover {
    type Parser = CoverResponse;
    fn from_response(response: Self::Parser) -> Self {
        let mut manga_id = String::new();

        for relation in response.relationships {
            if let utils::RelationshipType::Manga = relation.r#type {
                manga_id = relation.id
            }
        }

        Cover {
            id: response.data.id,
            volume: response.data.attributes.volume,
            file_name: response.data.attributes.file_name,
            description: response.data.attributes.description,
            created_at: DateTime::parse_from_rfc3339(&response.data.attributes.created_at).unwrap(),
            updated_at: DateTime::parse_from_rfc3339(&response.data.attributes.updated_at).unwrap(),
            manga_id,
        }
    }
}
