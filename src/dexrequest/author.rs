use super::utils::{get_data};
use crate::dexwrapper::{creator::CreatorTemplate, utils::{DexWrappedObject, DexError}};
use std::collections::HashMap;

pub fn search(_query_params: HashMap<&str, &str>) -> () {}

pub fn get(id: &str) -> Result<CreatorTemplate, DexError> {
    let uri = format!("https://api.mangadex.org/author/{id}", id = id);
    CreatorTemplate::from_string(
        &get_data(&uri)
        .unwrap())
}