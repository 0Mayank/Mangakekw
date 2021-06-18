mod parser;

use super::cover;
use super::utils;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CoverList {
    pub data: Vec<cover::Cover>,
    pub count: usize,
}

impl utils::DexWrappedObject for CoverList {
    type Parser = ChapterListResponse;

    fn from_response(response: Self::Parser) -> Self {
        CoverList {
            data: response
                .results
                .into_iter()
                .map(cover::Cover::from_response)
                .collect(),
            count: response.total as usize,
        }
    }
}
