use super::utils::{get_data, parse_url};
use std::collections::HashMap;

pub fn search_manga(query_params: HashMap<&str, &str>) -> () {
    let uri = parse_url("https://api.mangadex.org/manga", query_params);
    let _manga_list = get_data(&uri);
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