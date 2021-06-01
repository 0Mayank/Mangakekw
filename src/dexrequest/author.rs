use super::utils::{get_data};
use crate::dexwrapper::{creator::CreatorTemplate, error::ErrorList, utils::DexWrappedObject};
use std::collections::HashMap;

pub fn search_author(_query_params: HashMap<&str, &str>) -> () {}

pub fn get_author(id: &str) -> Result<CreatorTemplate, ErrorList> {
    let uri = format!("https://api.mangadex.org/author/{id}", id = id);
    CreatorTemplate::from_string(
        &get_data(&uri)
        .unwrap())
}