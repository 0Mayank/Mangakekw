use std::collections::HashMap;

use super::utils::{get_data, parse_url};
use crate::wrapper::{chapter_list::ChapterList, chapter::Chapter, utils::{DexWrappedObject, DexError}};

/// Searh chapters by passing query parameters as query params. Query: "https://api.mangadex.org/chapter".
///
/// # Arguments
///
/// * `query_params` - A HashMap<&str, &str> holding query parameters. The paramaeters are as follows:
///     * `limit` - integer {1..100}. default value = 10.
///     * `offset` - integer >= 0.
///     * `title` - string
///     * `ids` - Array of strings. Search by ids of the chapters
///     * `groups` - Array of strings.
///     * `uploader` - string <id>
///     * `manga` - string <id>
///     * `volume` - string
///     * `chapter` - string
///     * `translatedLanguage` - Array of strings
///     * `createdAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `updatedAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `publishAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `order` - object
///
/// # Example
/// 
/// ```
/// use std::collections::HashMap;
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::chapter;
///
/// let mut query_params = HashMap::new();
///
/// query_params.insert("limit", "2");
/// query_params.insert("offset", "3");
///
/// let chapters = chapter::search(query_params).unwrap();
/// 
/// println!("{}", chapters.serialize(true));
/// ```
///
/// # Panics
/// * native TLS backend cannot be initialized
/// * supplied Url cannot be parsed
/// * there was an error while sending request
/// * redirect loop was detected
/// * redirect limit was exhausted
/// * response cannot be parsed to string
/// 
/// # Errors
/// returns enum DexError
///
/// * api returns error json response
/// * serde parsing error
pub async fn search(query_params: HashMap<&str, &str>) -> Result<ChapterList, DexError> {
    let uri = parse_url("https://api.mangadex.org/chapter", query_params);
    ChapterList::from_string(
        &get_data(&uri)
        .await
        .unwrap())
}

/// Get chapter from chapter's id. Query: "https://api.mangadex.org/chapter/{id}".
///
/// # Arguments
///
/// * `id` - Id of chapter
///
/// # Example
///
/// ```
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::chapter;
///
/// let chapter = chapter::get("f5ec5e4f-2c95-48ca-b3f9-8e9ed6227928").unwrap();
/// println!("{}",chapter.serialize(true));
/// ```
///
/// # Panics
/// * native TLS backend cannot be initialized
/// * supplied Url cannot be parsed
/// * there was an error while sending request
/// * redirect loop was detected
/// * redirect limit was exhausted
/// * response cannot be parsed to string
/// 
/// # Errors
/// returns enum DexError
///
/// * api returns error json response
/// * serde parsing error
pub async fn get(id: &str) -> Result<Chapter, DexError> {
    let uri = format!("https://api.mangadex.org/chapter/{}", id);
    Chapter::from_string(
        &get_data(&uri)
        .await
        .unwrap())
}

/// Retrieve links for images of chapter pages.
///
/// # Arguments
///
/// * `id` - id of chapter
/// * `quality_mode` - quality of the images
///     * `=` "data". Original quality
///     * `=` "data-saver". Lower quality
///     * default value data if any other string is passed
///
/// # Example
///
/// ```
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::chapter;
///
/// let pages = chapter::retrieve("f5ec5e4f-2c95-48ca-b3f9-8e9ed6227928", "data").unwrap();
/// println!("{:#?}", pages);
/// ```
///
/// # Panics
/// * native TLS backend cannot be initialized
/// * supplied Url cannot be parsed
/// * there was an error while sending request
/// * redirect loop was detected
/// * redirect limit was exhausted
/// * response cannot be parsed to string
/// 
/// # Errors
/// returns enum DexError
///
/// * api returns error json response
/// * serde parsing error
pub async fn retrieve(id: &str, quality_mode: &str) -> Result<Vec<String>, DexError> {
    let chapter: Chapter = match get(&id).await {
        Ok(c) => c,
        Err(e) => return Err(e)
    };

    let base_url = &base_url(&id).await["baseUrl"];

    let uri = format!("{}/{}/{}", base_url, quality_mode, chapter.hash);

    let pages = if quality_mode == "data" {
        chapter.data
    } else if quality_mode == "data-saver" {
        chapter.data_saver
    } else {chapter.data};

    let mut page_urls = Vec::new();

    for page in pages{
        page_urls.push(format!("{}/{}", uri, page))
    }

    Ok(page_urls)
}

async fn base_url(chapter_id: &str) -> HashMap<String, String> {
    let uri = format!("https://api.mangadex.org/at-home/server/{chapter_id}", chapter_id = chapter_id);
    let deserialized: HashMap<String, String> = serde_json::from_str(&get_data(&uri).await.unwrap()).unwrap();
    
    deserialized
}