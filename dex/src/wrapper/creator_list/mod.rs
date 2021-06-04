pub mod parser;

use super::creator;
use super::utils;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreatorList {
    pub data: Vec<creator::CreatorTemplate>,
    pub count: usize,
}

impl utils::DexWrappedObject for CreatorList {
    type Parser = CreatorListResponse;

    fn from_response(response: Self::Parser) -> Self {
        CreatorList {
            data: response
                .results
                .into_iter()
                .map(|creator_response| creator::CreatorTemplate::from_response(creator_response))
                .collect(),
            count: response.total as usize,
        }
    }
}