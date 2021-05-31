use super::super::utils;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct ChapterResponse {
    pub result: utils::ResponseResult,
    pub data: ChapterRaw,
    pub relationships: Vec<utils::Relationship>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct ChapterRaw {
    pub id: String,
    pub attributes: ChapterAtribs,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct ChapterAtribs {
    pub title: String,
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub translated_language: String,
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
}
