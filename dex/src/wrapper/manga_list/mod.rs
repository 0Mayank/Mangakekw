mod parser;

use super::manga;
use super::utils;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MangaList {
    pub data: Vec<manga::Manga>,
    pub count: usize,
}

impl utils::DexWrappedObject for MangaList {
    type Parser = MangaListResponse;

    fn from_response(response: Self::Parser) -> Self {
        MangaList {
            data: response
                .results
                .into_iter()
                .map(manga::Manga::from_response)
                .collect(),
            count: response.total as usize,
        }
    }
}
