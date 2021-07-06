use super::Cover;
use super::super::utils::{DexError, DexWrappedObject};
use crate::{
    get_data
};

impl Cover {
    /// Get cover from cover's id. Query: "https://api.mangadex.org/cover/{id}".
    ///
    /// # Arguments
    ///
    /// * `id` - Id of cover
    ///
    /// # Example
    ///
    /// ```
    /// extern crate tokio;
    /// use dex::DexWrappedObject;
    /// use dex::Cover;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cover = Cover::get("94baea07-7492-4fa7-a5c2-a19beaee4da6").await.unwrap();
    ///     println!("{:?}", cover.serialize(true));
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
    /// extern crate tokio;
    /// use dex::DexWrappedObject;
    /// use dex::Cover;
    /// use dex::ParamSet;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let cover_image = Cover::retrieve("94baea07-7492-4fa7-a5c2-a19beaee4da6", 512).await.unwrap();
    ///     println!("{}", cover_image);
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
    pub async fn retrieve(id: &str, quality: u16) -> Result<String, DexError> {
        let cover = match Self::get(id).await {
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
}