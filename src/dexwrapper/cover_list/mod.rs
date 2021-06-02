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
    type Response = ChapterListResponse;
    #[allow(dead_code)]
    fn from_response(response: Self::Response) -> Self {
        CoverList {
            data: response
                .results
                .into_iter()
                .map(|chapter_response| cover::Cover::from_response(chapter_response))
                .collect(),
            count: response.total as usize,
        }
    }
}
