mod parser;

use parser::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
struct Chapter {
    id: String,
    title: String,
    volume: Option<String>,
    chapter: Option<String>,
    translated_language: String,
    hash: String,
    data: String,
}

impl Chapter {
    #[allow(dead_code)]
    pub fn from_response(response: ChapterResponse) -> Self {
        Chapter {
            id: response.data.id,
            title: response.data.attributes.title,
            volume: response.data.attributes.volume,
            chapter: response.data.attributes.chapter,
            translated_language: response.data.attributes.translated_language,
            hash: response.data.attributes.hash,
            data: response.data.attributes.data,
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
}
