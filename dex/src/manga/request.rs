use super::Manga;
use super::super::utils::{DexError, DexWrappedObject};
use crate::{
    get_data,
};

impl Manga {
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
    /// use dex::DexWrappedObject;
    /// use dex::Manga;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let manga = Manga::get("eb2d1a45-d4e7-4e32-a171-b5b029c5b0cb").await.unwrap();
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
    /// extern crate tokio;
    /// use dex::DexWrappedObject;
    /// use dex::Manga;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let manga = Manga::random().await.unwrap();
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
    pub async fn random() -> Result<Manga, DexError> {
        let uri = "https://api.mangadex.org/manga/random";
        Manga::from_string(&get_data(uri).await.unwrap())
    }
}