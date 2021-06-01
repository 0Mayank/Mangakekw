pub mod parser;
pub mod tests;

use super::utils;
use chrono::{DateTime, FixedOffset};
use parser::*;
use serde::Serialize;
use serde_json;

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

impl Cover {
    pub fn from_response(response: CoverResponse) -> Self {
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

    #[allow(dead_code)]
    pub fn serialize(&self, pretty: bool) -> String {
        if pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }

    pub fn from_string(string: String) -> Result<Self, serde_json::Error> {
        let response: CoverResponse = serde_json::from_str(&string)?;
        Ok(Self::from_response(response))
    }
}
