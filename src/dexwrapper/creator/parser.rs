#[allow(dead_code)]
use super::super::utils;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct CreatorResponse {
    pub result: utils::ResponseResult,
    pub data: CreatorRaw,
    pub relationships: Vec<utils::Relationship>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct CreatorRaw {
    pub id: String,
    pub attributes: CreatorAtribs,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct CreatorAtribs {
    pub name: String,
    pub image_url: Option<String>,
}
