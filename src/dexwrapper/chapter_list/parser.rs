use super::super::chapter::parser;

use serde::Deserialize;
#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct ChapterListResponse {
    pub results: Vec<parser::ChapterResponse>,
    pub total: i32,
}
