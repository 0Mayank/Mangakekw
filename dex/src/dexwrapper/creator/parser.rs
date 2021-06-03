#[allow(dead_code)]
use super::super::utils;
use serde::Deserialize;

pub type CreatorResponse = utils::DexResponse<CreatorAtribs>;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreatorAtribs {
    pub name: String,
    pub image_url: Option<String>,
}
