use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ErrorAtribs {
    pub id: String,
    pub status: u32,
    pub title: String,
    pub detail: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ErrorListResponse {
    pub result: String,
    pub errors: Vec<ErrorAtribs>,
}
