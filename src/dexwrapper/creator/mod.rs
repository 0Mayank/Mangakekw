pub mod parser;
mod tests;

use super::utils;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct CreatorTemplate {
    id: String,
    name: String,
    image_url: Option<String>,
    biography: String,
    works_id: Vec<String>,
}

impl CreatorTemplate {
    #[allow(dead_code)]
    pub fn from_response(response: CreatorResponse) -> Self {
        let mut works = Vec::new();
        for relation in response.relationships {
            match relation.r#type {
                utils::RelationshipType::Manga => works.push(relation.id),
                _ => (),
            }
        }

        CreatorTemplate {
            id: response.data.id.clone(),
            name: response.data.attributes.name.clone(),
            image_url: response.data.attributes.image_url,
            biography: String::new(),
            works_id: works,
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
}
#[derive(Serialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum Creator {
    Author(CreatorTemplate),
    Artist(CreatorTemplate),
}
