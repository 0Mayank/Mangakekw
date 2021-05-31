mod parser;

use super::chapter;
use parser::*;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
struct ChapterList {
    data: Vec<chapter::Chapter>,
    count: usize,
}

impl ChapterList {
    #[allow(dead_code)]
    pub fn from_response(response: ChapterListResponse) -> Self {
        ChapterList {
            data: response
                .results
                .into_iter()
                .map(|chapter_response| chapter::Chapter::from_response(chapter_response))
                .collect(),
            count: response.total as usize,
        }
    }

    #[allow(dead_code)]
    pub fn serialize(&self, pretty: bool) -> String {
        if pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }

    #[allow(dead_code)]
    pub fn from_string(string: String) -> Result<Self, serde_json::Error> {
        let response: ChapterListResponse = serde_json::from_str(&string)?;
        Ok(Self::from_response(response))
    }
}
