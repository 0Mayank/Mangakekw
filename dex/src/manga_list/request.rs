use super::MangaList;
use super::super::utils::{DexError, DexWrappedObject};
use std::collections::HashMap;
use crate::{
    get_data,
    parse_url,
    DynParam
};

impl MangaList {
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
    /// extern crate tokio;
    /// use std::collections::HashMap;
    /// use dex::DexWrappedObject;
    /// use dex::MangaList;
    /// use dex::ParamType;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut query_params = HashMap::new();
    ///     
    ///     query_params.insert("limit", ParamType::String(Some("2")));
    ///     query_params.insert("offset", ParamType::String(Some("3")));
    ///     query_params.insert("status", ParamType::String(None)); // gets ignored
    /// 
    ///     let mangas = MangaList::search(query_params).await.unwrap();
    /// 
    ///     println!("{:?}", mangas.serialize(true)); 
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
    pub async fn search(query_params: HashMap<&str, DynParam<'_>>) -> Result<MangaList, DexError> {
        let uri = parse_url("https://api.mangadex.org/manga", query_params);
        MangaList::from_string(&get_data(&uri).await.unwrap())
    }
}