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

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DexListResponse<T> {
    pub results: Vec<T>,
    pub total: i32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DexRaw<T> {
    pub id: String,
    pub attributes: T,
}
