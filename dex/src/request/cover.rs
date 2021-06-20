use std::collections::HashMap;

use super::utils::{get_data, parse_url};
use crate::wrapper::{
    cover::Cover,
    cover_list::CoverList,
    utils::{DexError, DexWrappedObject},
};

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
pub async fn search(query_params: HashMap<&str, &str>) -> Result<CoverList, DexError> {
    let uri = parse_url("https://api.mangadex.org/cover", query_params);
    CoverList::from_string(&get_data(&uri).await.unwrap())
}

/// Get cover from cover's id. Query: "https://api.mangadex.org/cover/{id}".
///
/// # Arguments
///
/// * `id` - Id of cover
///
/// # Example
///
/// ```
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::cover;
///
/// let cover = cover::get("e3d51092-6377-4f5a-8691-d9cec1adf640").unwrap();
/// println!("{}", cover.serialize(true));
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
pub async fn get(id: &str) -> Result<Cover, DexError> {
    let uri = format!("https://api.mangadex.org/cover/{}", id);
    Cover::from_string(&get_data(&uri).await.unwrap())
}

/// Retrieve links for the image of cover
///
/// # Arguments
///
/// * `id` - id of cover
/// * `quality_mode` - quality of the image
///     * `=` 0. Original quality
///     * `=` 512. 512 pixels
///     * `=` 256. 256 pixels
///     * default value 0 if any other number is passed
///
/// # Example
///
/// ```
/// use dex::wrapper::utils::DexWrappedObject;
/// use dex::request::cover;
///
/// let cover_image = cover::retrieve("e3d51092-6377-4f5a-8691-d9cec1adf640", 512).unwrap();
/// println!("{}", cover_image);
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
pub async fn retrieve(id: &str, quality: u16) -> Result<String, DexError> {
    let cover = match get(id).await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let mut url = format!(
        "https://uploads.mangadex.org/covers/{}/{}",
        cover.manga_id, cover.file_name
    );

    if quality == 512 {
        url.push_str(".512.jpg")
    } else if quality == 256 {
        url.push_str(".256.jpg")
    }

    Ok(url)
}
