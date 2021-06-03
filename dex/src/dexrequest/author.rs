use super::utils::{get_data, parse_url};
use crate::dexwrapper::{creator::CreatorTemplate, creator_list::CreatorList,utils::{DexWrappedObject, DexError}};
use std::collections::HashMap;

/// Searh authors by passing query parameters as query params. Query: "https://api.mangadex.org/author".
///
/// # Arguments
///
/// * `query_params` - A HashMap holding query parameters. The paramaeters are as follows:
///     * `limit` - integer {1..100}. default value = 10.
///     * `offset` - integer >= 0.
///     * `ids` - Array of strings. Search by ids of the authors
///     * `name` - string. search by name of the author
///     * `order` - object
///
/// # Example
/// 
/// ```
/// use std::collections::HashMap;
/// use dex::dexwrapper::utils::DexWrappedObject;
/// use dex::dexrequest::author;
///
/// let mut query_params = HashMap::new();
///
/// query_params.insert("limit", "2");
/// query_params.insert("offset", "3");
///
/// let authors = author::search(query_params).unwrap();
/// 
/// println!("{}",authors.serialize(true));
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
/// returns enum enum DexError
///
/// * Api returns error response json
/// * serde parsing error
pub fn search(query_params: HashMap<&str, &str>) -> Result<CreatorList, DexError> {
    let uri = parse_url("https://api.mangadex.org/author", query_params);
    CreatorList::from_string(
        &get_data(&uri)
        .unwrap())
}

/// Get author from author's id. Query: "https://api.mangadex.org/author/{id}".
///
/// # Arguments
///
/// * `id` - Id of author
pub fn get(id: &str) -> Result<CreatorTemplate, DexError> {
    let uri = format!("https://api.mangadex.org/author/{}", id);
    CreatorTemplate::from_string(
        &get_data(&uri)
        .unwrap())
}