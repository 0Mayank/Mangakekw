use super::utils::{get_data, parse_url};
use std::collections::HashMap;

use crate::wrapper::{
    chapter_list::ChapterList,
    manga::Manga,
    manga_list::MangaList,
    utils::{DexError, DexWrappedObject},
};

/// Searh manga by passing query parameters as query params. Query: "https://api.mangadex.org/manga".
///
/// # Arguments
///
/// * `query_params` - A HashMap<&str, &str> holding query parameters. The paramaeters are as follows:
///     * `limit` - integer {1..100}. default value = 10.
///     * `offset` - integer >= 0.
///     * `title` - string
///     * `ids` - Array of strings <manga id>. Limited to 100 per request
///     * `authors` - Array of strings. <author id>
///     * `authors` - Array of strings. <artist id>
///     * `year` - integer. Year of release
///     * `includedTags` - Array of string <id>
///     * `includedTagsMode` - string
///         * `=` "AND"
///         * `=` "OR"
///         * default "AND"
///     * `excludedTags` - Array of string <id>
///     * `excludedTagsMode` - string
///         * `=` "AND"
///         * `=` "OR"
///         * default "AND"
///     * `status` - Array of strings
///         * `=` "ongoing"
///         * `=` "completed"
///         * `=` "hiatus"
///         * `=` "cancelled"
///     * `originalLanguage` - Array of strings
///     * `publicationDemographic` - Array of strings
///     * `contentRating` - Array of strings
///         * `=` "none"
///         * `=` "safe"
///         * `=` "suggestive"
///         * `=` "erotica"
///         * `=` "pornographic"
///         * default ["none","safe","suggestive","erotica"]
///     * `createdAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `updatedAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `order` - object
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::manga;
///
/// let mut query_params = HashMap::new();
///
/// query_params.insert("limit", "2");
/// query_params.insert("offset", "3");
///
/// let mangas = manga::search(query_params).unwrap();
///
/// println!("{}", mangas.serialize(true));
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
pub async fn search(query_params: HashMap<&str, &str>) -> Result<MangaList, DexError> {
    let uri = parse_url("https://api.mangadex.org/manga", query_params);
    MangaList::from_string(&get_data(&uri).await.unwrap())
}

/// Get manga from manga's id. Query: "https://api.mangadex.org/manga/{id}".
///
/// # Arguments
///
/// * `id` - Id of manga
///
/// # Example
///
/// ```
/// extern crate tokio;
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::manga;
///
/// #[tokio::main]
/// async fn main() {
///     let manga = manga::get("eb2d1a45-d4e7-4e32-a171-b5b029c5b0cb").await.unwrap();
///     println!("{:?}", manga.serialize(true));
/// }
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
pub async fn get(id: &str) -> Result<Manga, DexError> {
    let uri = format!("https://api.mangadex.org/manga/{}", id);
    Manga::from_string(&get_data(&uri).await.unwrap())
}

/// Get chapter from chapter's id. Query: "https://api.mangadex.org/manga/random".
///
/// # Example
///
/// ```
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::manga;
///
/// let manga = manga::random().unwrap();
/// println!("{}", manga.serialize(true));
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
pub async fn random() -> Result<Manga, DexError> {
    let uri = "https://api.mangadex.org/manga/random";
    Manga::from_string(&get_data(uri).await.unwrap())
}

/// Feed manga chapters by passing manga id and query parameters as query params. Query: "https://api.mangadex.org/manga".
///
/// # Arguments
///
/// * `id` - Id of manga
/// * `query_params` - A HashMap<&str, &str> holding query parameters. The paramaeters are as follows:
///     * `limit` - integer {1..100}. default value = 10.
///     * `offset` - integer >= 0.
///     * `translatedLanguage` - array of strings
///     * `createdAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `updatedAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `publishedAtSince` - DateTime string with following format: YYYY-MM-DDTHH:MM:SS
///     * `order` - object
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::manga;
///
/// let mut query_params = HashMap::new();
///
/// query_params.insert("limit", "2");
/// query_params.insert("offset", "3");
///
/// let chapters = manga::feed("eb2d1a45-d4e7-4e32-a171-b5b029c5b0cb", query_params).unwrap();
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
pub async fn feed(id: &str, query_params: HashMap<&str, &str>) -> Result<ChapterList, DexError> {
    let uri = parse_url(
        &format!("https://api.mangadex.org/manga/{}/feed", id),
        query_params,
    );
    ChapterList::from_string(&get_data(&uri).await.unwrap())
}
