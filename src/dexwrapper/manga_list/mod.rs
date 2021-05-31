mod parser;
mod tests;

use super::manga;
use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
struct MangaList {
    data: Vec<manga::Manga>,
    count: usize,
}

impl MangaList {
    #[allow(dead_code)]
    pub fn from_response(response: MangaListResponse) -> Self {
        MangaList {
            data: response
                .results
                .into_iter()
                .map(|manga_response| manga::Manga::from_response(manga_response))
                .collect(),
            count: response.total as usize,
        }
    }

    #[allow(dead_code)]
    pub fn deserialize(&self, pretty: bool) -> String {
        if pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }
}
