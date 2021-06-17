use super::error;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "snake_case"))]
pub enum RelationshipType {
    Manga,
    Chapter,
    CoverArt,
    Author,
    Artist,
    ScanlationGroup,
    User,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum TagGroup {
    Theme,
    Content,
    Genre,
    Format,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum Demographic {
    Shounen,
    Seinen,
    Shoujou,
    Josei,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum Status {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum ResponseResult {
    Ok,
    Error,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub struct Relationship {
    pub id: String,
    pub r#type: RelationshipType,
}
#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DexResponse<T> {
    pub result: ResponseResult,
    pub data: DexRaw<T>,
    pub relationships: Vec<Relationship>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DexListResponse<T> {
    pub results: Vec<T>,
    pub total: usize,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DexRaw<T> {
    pub id: String,
    pub attributes: T,
}

#[derive(Debug)]
pub enum DexError {
    InvalidRequest(error::ErrorList),
    InvalidJSON,
}

/// Base trait implemented by every struct returned by from_string
/// # Object Safety
/// DexWrappedObject is NOT object safe i.e the following code is not valid
/// ```compile_fail
/// fn serialize_all(to_serialize: &Vec<&dyn DexWrappedObject<Parser = SomeParser>>) -> Vec<String> {
///     to_serialize.map(|obj| obj.serialize()).collect()   
/// }
/// ```
pub trait DexWrappedObject
where
    Self: Sized,
{
    /// The type used to parse the JSON into a struct
    /// # Trait Bounds
    /// Must implement serd::Deserialize for it to actualy deserialize something
    type Parser;

    /// Flattens the given `Parser` into a more convinient struct
    fn from_response(response: Self::Parser) -> Self;

    /// Converts a struct into a JSON string
    ///# Arguments
    /// `pretty` - If `true`, the string returned is formatted with indentations for human readablity
    ///# Errors
    /// Serialization can fail if `Self`'s implementation of Serialize decides to fail, or if `Self` contains a map with non-string keys.
    fn serialize(&self, pretty: bool) -> Result<String, serde_json::Error>
    where
        Self: serde::Serialize,
    {
        if pretty {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
    }

    /// Deserializes the JSON into a struct with give Parser
    ///# Errors
    /// This conversion can fail if the structure of the input does not match the structure expected by `Parser`,
    /// for example if `Parser` is a struct type but the input contains something other than a JSON map.
    /// It can also fail if the structure is correct but `Parser`'s implementation of Deserialize decides that something is wrong with the data,
    /// for example required struct fields are missing from the JSON map or some number is too big to fit in the expected primitive type.
    /// In all the above cases, `DexError::InvalidJSON` is returned.
    /// If the JSON itself represented an error returnrd by Mangadex, in valid JSON
    /// the function returns `DexError::InvalidRequest(error::Errorlist)`  
    fn from_string<'a>(string: &'a str) -> Result<Self, DexError>
    where
        Self::Parser: serde::Deserialize<'a>,
    {
        let response: Result<Self::Parser, serde_json::Error> = serde_json::from_str(string);

        match response {
            Ok(r) => Ok(Self::from_response(r)),
            Err(_) => {
                let error_response: Result<error::parser::ErrorListResponse, serde_json::Error> =
                    serde_json::from_str(string);
                match error_response {
                    Ok(r) => Err(DexError::InvalidRequest(error::ErrorList::from_response(r))),
                    Err(_) => Err(DexError::InvalidJSON),
                }
            }
        }
    }
}
