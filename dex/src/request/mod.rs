use async_trait::async_trait;

pub mod author;
pub mod chapter;
pub mod cover;
pub mod manga;
pub mod utils;

pub use crate::wrapper::utils::{DexWrappedObject, DexError};
pub use self::utils::DynParam;

use std::collections::HashMap;
use utils::{get_data, parse_url};

use serde::Deserialize;

#[async_trait]
pub trait Requests<T:DexWrappedObject> {
    
    type query;

    async fn get(id: &str) -> Result<T, DexError> {
        let uri = format!("https://api.mangadex.org/author/{}", id);
        <T as DexWrappedObject>::from_string(&get_data(&uri).await.unwrap());
        
        todo!();
    }
    fn search(query_params: HashMap<&str, DynParam<'_>>) -> Result<T, DexError>;
}
