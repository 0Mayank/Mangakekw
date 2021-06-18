mod parser;

use super::chapter;
use super::utils;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ChapterList {
    pub data: Vec<chapter::Chapter>,
    pub count: usize,
}

impl utils::DexWrappedObject for ChapterList {
    type Parser = ChapterListResponse;
    #[allow(dead_code)]
    fn from_response(response: Self::Parser) -> Self {
        ChapterList {
            data: response
                .results
                .into_iter()
                .map(chapter::Chapter::from_response)
                .collect(),
            count: response.total as usize,
        }
    }
}
