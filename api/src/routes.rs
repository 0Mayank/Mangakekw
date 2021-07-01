use dex::request::author;
use dex::request::chapter::{self, CqueryParams};
use dex::request::cover;
use dex::request::manga;
use dex::request::utils::DynParam::{
    String as S,
    Array as A,
    Integer as I
};

use super::utils::meta::{
    ApiResponse,
    Order
};

use std::collections::HashMap;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get/manga/<id>")]
pub async fn get_manga(id: &str) -> ApiResponse {
    let result = manga::get(id).await;

    ApiResponse::resolve(result)
}

#[get("/get/author/<id>")]
pub async fn get_author(id: &str) -> ApiResponse {
    let result = author::get(id).await;

    ApiResponse::resolve(result)
}

#[get("/get/chapter/<id>")]
pub async fn get_chapter(id: &str) -> ApiResponse {
    let result = chapter::get(id).await;

    ApiResponse::resolve(result)
}

#[get("/get/cover/<id>")]
pub async fn get_cover(id: &str) -> ApiResponse {
    let result = cover::get(id).await;

    ApiResponse::resolve(result)
}

#[get("/search/manga?<limit>&<offset>&<title>&<ids>&<authors>&<year>&<includedtags>&<includedtagsmode>&<excludedtags>&<excludedtagsmode>&<status>&<originallanguage>&<publicationdemographic>&<contentrating>&<createdatsince>&<updatedatsince>&<order>")]
pub async fn search_manga(
    limit: Option<&str>, 
    offset: Option<&str>, 
    title: Option<&str>, 
    ids: Vec<String>, // ids should be Option<Vec<&str>>
    authors: Vec<String>,
    year: Option<i32>,
    includedtags: Vec<String>,
    includedtagsmode: Option<&str>,
    excludedtags: Vec<String>,
    excludedtagsmode: Option<&str>,
    status: Vec<String>,
    originallanguage: Vec<String>,
    publicationdemographic: Vec<String>,
    contentrating: Vec<String>,
    createdatsince: Option<&str>,
    updatedatsince: Option<&str>,
    order: Order<'_>
) -> ApiResponse 
{
    let query_params: HashMap<_,_> = collection!(
        "limit" => S(limit),
        "offset" => S(offset),
        "title" => S(title),
        "ids" => A(ids),
        "authors" => A(authors),
        "year" => I(year),
        "includedTags" => A(includedtags),
        "includedTagsMode" => S(includedtagsmode),
        "excludedTags" => A(excludedtags),
        "excludedTagsMode" => S(excludedtagsmode),
        "status" => A(status),
        "originalLanguage" => A(originallanguage),
        "publicationDemographic" => A(publicationdemographic),
        "contentRating" => A(contentrating),
        "createdAtSince" => S(createdatsince),
        "updatedAtSince" => S(updatedatsince),
        "order[createdAt]" => S(order.createdat),
        "order[updatedAt]" => S(order.updatedat)
    );
    
    let result = manga::search(query_params).await;

    ApiResponse::resolve(result)
}

#[get("/search/author/<_id>")]
pub async fn search_author(_id: &str) -> ApiResponse {
    todo!()
}

#[get("/search/chapter/<_id>")]
pub async fn search_chapter(_id: &str) -> ApiResponse {
    todo!()
}

#[get("/search/cover/<_id>")]
pub async fn search_cover(_id: &str) -> ApiResponse {
    todo!()
}

#[get("/chapter/retrieve/<id>?<quality>&<hash>&<files>")]
pub async fn chapter_retrieve(id: &str, quality: Option<&str>, hash: Option<String>, files: Vec<String>) -> ApiResponse {
    let params;
    println!("{:?}", files);
    
    if let Some(hash) = hash{ // TODO blonteractor does the refractor
        if files.len() != 0 {
            params = CqueryParams::WithHF{
                id,
                quality: quality.unwrap_or_else(|| "data"),
                hash,
                file_names: files
            }
        } else {
            params = CqueryParams::WithoutHF{
                id,
                quality: quality.unwrap_or_else(|| "data"),
            }
        }
    } else {
        params = CqueryParams::WithoutHF{
            id,
            quality: quality.unwrap_or_else(|| "data"),
        }
    }
    
    let result = chapter::retrieve(params).await;
    
    ApiResponse::resolve_vec(result)
}

#[get("/cover/retrieve/<id>?<quality>")]
pub async fn cover_retrieve(id: &str, quality: Option<u16>) -> ApiResponse {
    let result = cover::retrieve(id, quality.unwrap_or_else(|| 512)).await;

    ApiResponse::resolve_url(result)
}