use super::super::manga::parser::MangaResponse;

use serde::Deserialize;
#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct MangaListResponse {
    pub results: Vec<MangaResponse>,
    pub total: i32,
}
