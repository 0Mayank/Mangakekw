use super::super::utils;
use serde::Deserialize;

pub type ChapterResponse = utils::DexResponse<ChapterAtribs>;

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChapterAtribs {
    pub title: String,
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub translated_language: String,
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
}
