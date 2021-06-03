use std::collections::HashMap;

use super::utils::{get_data, parse_url};
use crate::wrapper::{cover_list::CoverList, cover::Cover, utils::{DexWrappedObject, DexError}};

pub fn search(query_params: HashMap<&str, &str>) -> Result<CoverList, DexError> {
    let uri = parse_url("https://api.mangadex.org/cover", query_params);
    CoverList::from_string(
        &get_data(&uri)
        .unwrap())
}
pub fn get(id: &str) -> Result<Cover, DexError> {
    let uri = format!("https://api.mangadex.org/cover/{}", id);
    Cover::from_string(
        &get_data(&uri)
        .unwrap())
}

pub fn retrieve(id: &str, quality: u16) -> Result<String, DexError>{
    let cover = match get(&id) {
        Ok(c) => c,
        Err(e) => return Err(e)
    };

    let mut url = format!("https://uploads.mangadex.org/covers/{}/{}", cover.manga_id, cover.file_name);

    if quality == 512 {
        url.push_str(".512.jpg")
    } else if quality == 256 {
        url.push_str(".256.jpg")
    }

    Ok(url)
}