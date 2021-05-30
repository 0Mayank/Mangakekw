use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "snake_case"))]
pub enum RelationshipType {
    Manga,
    Chapter,
    CoverArt,
    Author,
    Artist,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum TagGroup {
    Theme,
    Content,
    Genre,
    Format,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
pub enum Demographic {
    Shounen,
    Seinen,
    Shoujou,
    Josei,
}
#[derive(Deserialize, Serialize)]
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
