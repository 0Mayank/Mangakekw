use super::ChapterList;
use super::super::utils::{DexError, DexWrappedObject};
use std::collections::HashMap;
use crate::{
    get_data,
    parse_url,
    DynParam
};

impl ChapterList {
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
    pub async fn search(query_params: HashMap<&str, DynParam<'_>>) -> Result<ChapterList, DexError> {
        let uri = parse_url("https://api.mangadex.org/chapter", query_params);
        ChapterList::from_string(&get_data(&uri).await.unwrap())
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
    pub async fn feed(id: &str, query_params: HashMap<&str, DynParam<'_>>) -> Result<ChapterList, DexError> {
        let uri = parse_url(
            &format!("https://api.mangadex.org/manga/{}/feed", id),
            query_params,
        );
        ChapterList::from_string(&get_data(&uri).await.unwrap())
    }
}