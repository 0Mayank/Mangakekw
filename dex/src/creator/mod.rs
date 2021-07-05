pub mod parser;
pub mod request;
mod tests;

use super::utils;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreatorTemplate {
    pub id: String,
    pub name: String,
    pub image_url: Option<String>,
    pub biography: String,
    pub works_id: Vec<String>,
}

impl utils::DexWrappedObject for CreatorTemplate {
    type Parser = CreatorResponse;

    fn from_response(response: Self::Parser) -> Self {
        let mut works = Vec::new();
        for relation in response.relationships {
            if let utils::RelationshipType::Manga = relation.r#type {
                works.push(relation.id)
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
}

#[derive(Serialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum Creator {
    Author(CreatorTemplate),
    Artist(CreatorTemplate),
}
