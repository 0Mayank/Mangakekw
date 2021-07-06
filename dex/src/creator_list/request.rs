use super::CreatorList;
use super::super::utils::{DexError, DexWrappedObject};
use std::collections::HashMap;
use crate::{
    get_data,
    parse_url,
    DynParam
};

impl CreatorList {
    /// Searh authors by passing query parameters as query params. Query: "https://api.mangadex.org/author".
    ///
    /// # Arguments
    ///
    /// * `query_params` - A HashMap<&str, &str> holding query parameters. The paramaeters are as follows:
    ///     * `limit` - integer {1..100}. default value = 10.
    ///     * `offset` - integer >= 0.
    ///     * `ids` - Array of strings. Search by ids of the authors
    ///     * `name` - string. search by name of the author
    ///     * `order` - object
    ///
    /// # Example
    ///
    /// ```
    /// extern crate tokio;
    /// use std::collections::HashMap;
    /// use dex::DexWrappedObject;
    /// use dex::AuthorList;
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
    ///     let authors = AuthorList::search(query_params).await.unwrap();
    /// 
    ///     println!("{:?}", authors.serialize(true)); 
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
    pub async fn search(query_params: HashMap<&str, DynParam<'_>>) -> Result<CreatorList, DexError> {
        let uri = parse_url("https://api.mangadex.org/author", query_params);
        CreatorList::from_string(&get_data(&uri).await.unwrap())
    }
}