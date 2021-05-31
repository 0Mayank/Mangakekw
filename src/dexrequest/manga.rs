use super::utils::{get_data, parse_url};
use std::collections::HashMap;
use crate::dexwrapper::{manga::Manga, manga_list::MangaList};

pub fn search_manga(query_params: HashMap<&str, &str>) -> MangaList {
    let uri = parse_url("https://api.mangadex.org/manga", query_params);
    let manga_list = match get_data(&uri){
        Ok(s) => s,
        Err(_) => panic!(),
    };

    let manga_list = match MangaList::from_string(manga_list) {
        Ok(m) => m,
        Err(_) => panic!(),
    };

    manga_list
}

pub fn get_manga(id: &str) -> () {
    let uri = format!("https://api.mangadex.org/manga/{}", id);
    let _manga = get_data(&uri);
}

pub fn get_manga_random() -> () {
    let uri = "https://api.mangadex.org/manga/random";
    let _manga = get_data(uri);
}

pub fn feed_manga(id: &str, query_params: HashMap<&str, &str>) -> () {
    let uri = parse_url(&format!("https://api.mangadex.org/manga/{}/feed", id), query_params);
    let _chapters = get_data(&uri); 
}