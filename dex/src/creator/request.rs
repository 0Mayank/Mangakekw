use super::CreatorTemplate;
use super::super::utils::{DexError, DexWrappedObject};
use crate::{
    get_data
};

impl CreatorTemplate {
    /// Get author from author's id. Query: "https://api.mangadex.org/author/{id}".
    ///
    /// # Arguments
    ///
    /// * `id` - Id of author
    ///
    /// # Example
    ///
    /// ```
    /// extern crate tokio;
    /// use dex::DexWrappedObject;
    /// use dex::Author;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let author = Author::get("15fe4d54-ae08-4177-af94-868bb7db1bcf").await.unwrap();
    ///     println!("{:?}", author.serialize(true));
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
    pub async fn get(id: &str) -> Result<Self, DexError> {
        let uri = format!("https://api.mangadex.org/author/{}", id);
        CreatorTemplate::from_string(&get_data(&uri).await.unwrap())
    }
}