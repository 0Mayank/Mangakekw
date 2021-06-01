use std::collections::HashMap;

use super::utils::{get_data, parse_url};
use crate::dexwrapper::{chapter_list::ChapterList, chapter::Chapter, utils::DexWrappedObject, error::ErrorList};

pub fn search_chapter(query_params: HashMap<&str, &str>) -> Result<ChapterList, ErrorList> {
    let uri = parse_url("https://api.mangadex.org/chapter", query_params);
    ChapterList::from_string(
        &get_data(&uri)
        .unwrap())
}

pub fn get_chapter(id: &str) -> Result<Chapter, ErrorList> {
    let uri = format!("https://api.mangadex.org/chapter/{id}", id = id);
    Chapter::from_string(
        &get_data(&uri)
        .unwrap())
}

pub fn get_chapter_pages() -> () {}