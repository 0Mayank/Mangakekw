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

    #[allow(non_snake_case)]
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
pub enum ErrorType {
    ResponseError(error::ErrorList),
    InvalidJSON,
}

pub trait DexWrappedObject {
    type Response;

    fn from_response(response: Self::Response) -> Self;

    fn serialize(&self, pretty: bool) -> String
    where
        Self: Sized + serde::Serialize,
    {
        if pretty {
            serde_json::to_string_pretty(self).unwrap()
        } else {
            serde_json::to_string(self).unwrap()
        }
    }

    /// s
    ///sd
    ///
    fn from_string<'a>(string: &'a str) -> Result<Self, ErrorType>
    where
        Self: Sized,
        Self::Response: serde::Deserialize<'a>,
    {
        let response: Result<Self::Response, serde_json::Error> = serde_json::from_str(string);

        match response {
            Ok(r) => Ok(Self::from_response(r)),
            Err(_) => {
                let error_response: Result<error::parser::ErrorListResponse, serde_json::Error> =
                    serde_json::from_str(string);
                match error_response {
                    Ok(r) => Err(ErrorType::ResponseError(error::ErrorList::from_response(r))),
                    Err(_) => Err(ErrorType::InvalidJSON),
                }
            }
        }
    }
}
