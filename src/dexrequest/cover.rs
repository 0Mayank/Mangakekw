use std::collections::HashMap;

use super::utils::{get_data, parse_url};
use crate::dexwrapper::{cover_list::CoverList, cover::Cover, error::ErrorList, utils::DexWrappedObject};

pub fn search_cover(query_params: HashMap<&str, &str>) -> Result<CoverList, ErrorList> {
    let uri = parse_url("https://api.mangadex.org/cover", query_params);
    CoverList::from_string(
        &get_data(&uri)
        .unwrap())
}
pub fn get_cover(id: &str) -> Result<Cover, ErrorList> {
    let uri = format!("https://api.mangadex.org/cover/{id}", id = id);
    Cover::from_string(
        &get_data(&uri)
        .unwrap())
}