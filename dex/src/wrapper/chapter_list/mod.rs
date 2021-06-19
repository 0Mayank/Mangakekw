mod parser;

use super::chapter::Chapter;
use super::utils;
use parser::*;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Serialize)]
pub struct ChapterList(utils::DexListResponse<Chapter>);

impl utils::DexWrappedObject for ChapterList {
    type Parser = ChapterListResponse;

    fn from_response(response: Self::Parser) -> Self {
        ChapterList(utils::DexListResponse {
            results: response
                .results
                .into_iter()
                .map(Chapter::from_response)
                .collect(),
            total: response.total as usize,
        })
    }
}

impl Deref for ChapterList {
    type Target = utils::DexListResponse<Chapter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChapterList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
