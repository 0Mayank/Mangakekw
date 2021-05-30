use super::super::utils;
use serde::Deserialize;
use std::{collections::HashMap, usize};

pub type Title = HashMap<String, String>;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct MangaAtribs {
    pub title: Title,
    pub alt_titles: Vec<Title>,
    pub description: HashMap<String, String>,
    pub original_language: String,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<utils::Demographic>,
    pub status: utils::Status,
    pub year: Option<usize>,
    pub content_rating: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<Tag>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct MangaRaw {
    pub id: String,
    pub attributes: MangaAtribs,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct MangaResponse {
    result: utils::ResponseResult,
    pub data: MangaRaw,
    pub relationships: Vec<MangaRelation>,
}
#[derive(Deserialize)]
#[allow(dead_code)]
#[allow(non_snake_case)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct MangaRelation {
    pub id: String,
    pub Type: utils::RelationshipType,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Tag {
    pub id: String,
    pub attributes: TagAtrib,
}
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct TagAtrib {
    pub name: TagName,
    pub group: utils::TagGroup,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct TagName {
    pub en: String,
}
