use super::Chapter;
use super::super::utils::{DexError, DexWrappedObject};
use std::collections::HashMap;
use crate::{
    get_data,
}; 

impl Chapter {
    /// Get chapter from chapter's id. Query: "https://api.mangadex.org/chapter/{id}".
    ///
    /// # Arguments
    ///
    /// * `id` - Id of chapter
    ///
    /// # Example
    ///
    /// ```
    /// extern crate tokio;
    /// use dex::DexWrappedObject;
    /// use dex::Chapter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let chapter = Chapter::get("f5ec5e4f-2c95-48ca-b3f9-8e9ed6227928").await.unwrap();
    ///     println!("{:?}", chapter.serialize(true));
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
    pub async fn get(id: &str) -> Result<Chapter, DexError> {
        let uri = format!("https://api.mangadex.org/chapter/{}", id);
        Chapter::from_string(&get_data(&uri).await.unwrap())
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
    /// extern crate tokio;
    /// use dex::DexWrappedObject;
    /// use dex::Chapter;
    /// use dex::ParamSet;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let params = ParamSet::WithoutHF {
    ///          id: "f5ec5e4f-2c95-48ca-b3f9-8e9ed6227928",
    ///          quality: "data"      
    ///         };
    ///
    ///     let chapter = Chapter::retrieve(params).await.unwrap();
    ///     println!("{:?}", chapter);
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
    pub async fn retrieve(params: CqueryParams<'_>) -> Result<Vec<String>, DexError> {
        
        let (id, quality, hash, pages) = parse_params(params).await?;

        let base_url = &base_url(id).await["baseUrl"];
        let uri = format!("{}/{}/{}", base_url, quality, hash);

        let mut page_urls = Vec::new();

        for page in pages {
            page_urls.push(format!("{}/{}", uri, page))
        }

        Ok(page_urls)
    }
}
async fn parse_params(param_type: CqueryParams<'_>) -> Result<(&str, &str, String, Vec<String>), DexError>{
    let hash: String;
    let pages;
    let id;
    let quality;

    match param_type {
        CqueryParams::WithoutHF{id: i, quality: q} => {
            id = i;
            quality = q;

            let chapter: Chapter = match Chapter::get(id).await {
                Ok(c) => c,
                Err(e) => return Err(e),
            };

            pages = if quality == "data" {
                chapter.data
            } else if quality == "data-saver" {
                chapter.data_saver
            } else {
                chapter.data
            };

            hash = chapter.hash;
        },
        CqueryParams::WithHF{id: i, quality: q, hash: h, file_names} => {
            id = i;
            quality = q;
            hash = h;
            pages = file_names;
        }
    }

    Ok((id, quality, hash, pages))
}

async fn base_url(chapter_id: &str) -> HashMap<String, String> {
    let uri = format!(
        "https://api.mangadex.org/at-home/server/{chapter_id}",
        chapter_id = chapter_id
    );
    let deserialized: HashMap<String, String> =
        serde_json::from_str(&get_data(&uri).await.unwrap()).unwrap();

    deserialized
}

pub enum CqueryParams<'a>{
    WithoutHF{
        id: &'a str,
        quality: &'a str
    },
    WithHF{
        id: &'a str,
        quality: &'a str,
        hash: String,
        file_names: Vec<String>
    }
}
