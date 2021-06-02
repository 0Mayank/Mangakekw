use std::collections::HashMap;

use super::utils::{get_data, parse_url};
use crate::dexwrapper::{chapter_list::ChapterList, chapter::Chapter, utils::{DexWrappedObject, DexError}};

pub fn search(query_params: HashMap<&str, &str>) -> Result<ChapterList, DexError> {
    let uri = parse_url("https://api.mangadex.org/chapter", query_params);
    ChapterList::from_string(
        &get_data(&uri)
        .unwrap())
}

pub fn get(id: &str) -> Result<Chapter, DexError> {
    let uri = format!("https://api.mangadex.org/chapter/{}", id);
    Chapter::from_string(
        &get_data(&uri)
        .unwrap())
}

pub fn retrieve(id: &str, quality_mode: &str) -> Result<Vec<String>, DexError> {
    let chapter: Chapter = match get(&id) {
        Ok(c) => c,
        Err(e) => return Err(e)
    };

    let base_url = &base_url(&id)["baseUrl"];

    let uri = format!("{}/{}/{}", base_url, quality_mode, chapter.hash);

    let pages = if quality_mode == "data" {
        chapter.data
    } else if quality_mode == "data-saver" {
        chapter.data_saver
    } else {chapter.data};

    let mut page_urls = Vec::new();

    for page in pages{
        page_urls.push(format!("{}/{}", uri, page))
    }

    Ok(page_urls)
}

fn base_url(chapter_id: &str) -> HashMap<String, String> {
    let uri = format!("https://api.mangadex.org/at-home/server/{chapter_id}", chapter_id = chapter_id);
    let deserialized: HashMap<String, String> = serde_json::from_str(&get_data(&uri).unwrap()).unwrap();
    
    deserialized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retriev_test() {
        let res = retrieve("f5ec5e4f-2c95-48ca-b3f9-8e9ed6227928", "data").unwrap();
        println!("{:#?}", res);
    }
    
    #[test]
    fn retriev_test_low() {
        let res = retrieve("f5ec5e4f-2c95-48ca-b3f9-8e9ed6227928", "data-saver").unwrap();
        println!("{:#?}", res);
    }
}