mod parser;
mod tests;

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
    type Response = MangaListResponse;
    #[allow(dead_code)]
    fn from_response(response: Self::Response) -> Self {
        MangaList {
            data: response
                .results
                .into_iter()
                .map(|manga_response| manga::Manga::from_response(manga_response))
                .collect(),
            count: response.total as usize,
        }
    }
}
