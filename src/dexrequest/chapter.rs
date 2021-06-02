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
    let uri = format!("https://api.mangadex.org/chapter/{id}", id = id);
    Chapter::from_string(
        &get_data(&uri)
        .unwrap())
}

pub fn get_pages() -> () {}

