use super::super::utils;
use serde::Deserialize;

pub type CoverResponse = utils::DexResponse<CoverAtribs>;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CoverAtribs {
    pub volume: Option<String>,
    pub file_name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
