use super::utils::{get_data, parse_url};
use std::collections::HashMap;

use crate::dexwrapper::{manga::Manga, manga_list::MangaList, error::ErrorList, chapter_list::ChapterList, utils::DexWrappedObject};

pub fn search_manga(query_params: HashMap<&str, &str>) -> Result<MangaList, ErrorList> {
    let uri = parse_url("https://api.mangadex.org/manga", query_params);
    MangaList::from_string(
        &get_data(&uri)
        .expect(""))
}

pub fn get_manga(id: &str) ->  Result<Manga, ErrorList> {
    let uri = format!("https://api.mangadex.org/manga/{}", id);
    Manga::from_string(
        &get_data(&uri)
        .expect(""))
}

pub fn get_manga_random() -> Result<Manga, ErrorList> {
    let uri = "https://api.mangadex.org/manga/random";
    Manga::from_string(
        &get_data(uri)
        .expect(""))
}

pub fn feed_manga(id: &str, query_params: HashMap<&str, &str>) -> Result<ChapterList, ErrorList> {
    let uri = parse_url(
        &format!("https://api.mangadex.org/manga/{}/feed", id),
        query_params
    );
    ChapterList::from_string(
        &get_data(&uri)
        .expect(""))
}
