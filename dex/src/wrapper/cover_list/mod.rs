mod parser;

use super::cover::Cover;
use super::utils;
use parser::*;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Serialize)]
pub struct CoverList(utils::DexListResponse<Cover>);

impl utils::DexWrappedObject for CoverList {
    type Parser = ChapterListResponse;

    fn from_response(response: Self::Parser) -> Self {
        CoverList(utils::DexListResponse {
            results: response
                .results
                .into_iter()
                .map(Cover::from_response)
                .collect(),
            total: response.total as usize,
        })
    }
}

impl Deref for CoverList {
    type Target = utils::DexListResponse<Cover>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CoverList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
