pub mod parser;

use super::creator::CreatorTemplate;
use super::utils;
use parser::*;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreatorList(utils::DexListResponse<CreatorTemplate>);

impl utils::DexWrappedObject for CreatorList {
    type Parser = CreatorListResponse;

    fn from_response(response: Self::Parser) -> Self {
        CreatorList(utils::DexListResponse {
            results: response
                .results
                .into_iter()
                .map(CreatorTemplate::from_response)
                .collect(),
            total: response.total as usize,
        })
    }
}

impl Deref for CreatorList {
    type Target = utils::DexListResponse<CreatorTemplate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CreatorList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
