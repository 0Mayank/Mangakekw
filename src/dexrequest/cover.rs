use std::collections::HashMap;

use super::utils::{get_data, parse_url};
use crate::dexwrapper::{cover_list::CoverList, cover::Cover, utils::{DexWrappedObject, DexError}};

pub fn search(query_params: HashMap<&str, &str>) -> Result<CoverList, DexError> {
    let uri = parse_url("https://api.mangadex.org/cover", query_params);
    CoverList::from_string(
        &get_data(&uri)
        .unwrap())
}
pub fn get(id: &str) -> Result<Cover, DexError> {
    let uri = format!("https://api.mangadex.org/cover/{id}", id = id);
    Cover::from_string(
        &get_data(&uri)
        .unwrap())
}