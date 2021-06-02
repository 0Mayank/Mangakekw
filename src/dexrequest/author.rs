use super::utils::{get_data, parse_url};
use crate::dexwrapper::{creator::CreatorTemplate, creator_list::CreatorList,utils::{DexWrappedObject, DexError}};
use std::collections::HashMap;

pub fn search(query_params: HashMap<&str, &str>) -> Result<CreatorList, DexError> {
    let uri = parse_url("https://api.mangadex.org/author", query_params);
    CreatorList::from_string(
        &get_data(&uri)
        .unwrap())
}

pub fn get(id: &str) -> Result<CreatorTemplate, DexError> {
    let uri = format!("https://api.mangadex.org/author/{}", id);
    CreatorTemplate::from_string(
        &get_data(&uri)
        .unwrap())
}