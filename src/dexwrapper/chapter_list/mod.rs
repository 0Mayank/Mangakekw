mod parser;

use super::chapter;
use super::utils;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ChapterList {
    data: Vec<chapter::Chapter>,
    count: usize,
}

impl utils::DexWrappedObject for ChapterList {
    type Response = ChapterListResponse;
    #[allow(dead_code)]
    fn from_response(response: Self::Response) -> Self {
        ChapterList {
            data: response
                .results
                .into_iter()
                .map(|chapter_response| chapter::Chapter::from_response(chapter_response))
                .collect(),
            count: response.total as usize,
        }
    }
}
