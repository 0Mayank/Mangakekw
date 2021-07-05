use super::CoverList;
use super::super::utils::{DexError, DexWrappedObject};
use std::collections::HashMap;
use crate::{
    get_data,
    parse_url,
    DynParam
};

impl CoverList {
    /// Searh cover by passing query parameters as query params. Query: "https://api.mangadex.org/cover".
    ///
    /// # Arguments
    ///
    /// * `query_params` - A HashMap<&str, &str> holding query parameters. The paramaeters are as follows:
    ///     * `limit` - integer {1..100}. default value = 10.
    ///     * `offset` - integer >= 0.
    ///     * `manga` - string <manga id>
    ///     * `ids` - array of strings <cover id>
    ///     * `uploaders` - array of strings <user id>
    ///     * `order` - object
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use dex::wrapper::utils::DexWrappedObject;
    /// use dex::request::cover;
    ///
    /// let mut query_params = HashMap::new();
    ///
    /// query_params.insert("limit", "2");
    /// query_params.insert("offset", "3");
    ///
    /// let covers = cover::search(query_params).unwrap();
    ///
    /// println!("{}", covers.serialize(true));
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
    pub async fn search(query_params: HashMap<&str, DynParam<'_>>) -> Result<CoverList, DexError> {
        let uri = parse_url("https://api.mangadex.org/cover", query_params);
        CoverList::from_string(&get_data(&uri).await.unwrap())
    }
}
