pub mod parser;
pub mod tests;

use super::utils;
use chrono::{DateTime, FixedOffset};
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Cover {
    id: String,
    volume: Option<String>,
    file_name: String,
    description: Option<String>,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    manga_id: String,
}

impl utils::DexWrappedObject for Cover {
    type Response = CoverResponse;
    fn from_response(response: Self::Response) -> Self {
        let mut manga_id = String::new();

        for relation in response.relationships {
            match relation.r#type {
                utils::RelationshipType::Manga => manga_id = relation.id,
                _ => (),
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
