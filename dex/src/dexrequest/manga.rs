use super::utils::{get_data, parse_url};
use std::collections::HashMap;

use crate::dexwrapper::{manga::Manga, manga_list::MangaList, chapter_list::ChapterList, utils::{DexWrappedObject, DexError}};

pub fn search(query_params: HashMap<&str, &str>) -> Result<MangaList, DexError> {
    let uri = parse_url("https://api.mangadex.org/manga", query_params);
    MangaList::from_string(
        &get_data(&uri)
        .expect(""))
}

pub fn get(id: &str) ->  Result<Manga, DexError> {
    let uri = format!("https://api.mangadex.org/manga/{}", id);
    Manga::from_string(
        &get_data(&uri)
        .expect(""))
}

pub fn get_random() -> Result<Manga, DexError> {
    let uri = "https://api.mangadex.org/manga/random";
    Manga::from_string(
        &get_data(uri)
        .expect(""))
}

pub fn feed(id: &str, query_params: HashMap<&str, &str>) -> Result<ChapterList, DexError> {
    let uri = parse_url(
        &format!("https://api.mangadex.org/manga/{}/feed", id),
        query_params
    );
    ChapterList::from_string(
        &get_data(&uri)
        .expect(""))
}
