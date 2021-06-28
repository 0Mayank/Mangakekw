#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

use dex::request::author;
use dex::request::cover;
use dex::request::manga;
use dex::request::utils::DynParam::{
    String as S,
    Array as A,
    Integer as I
};
use utils::meta::{
    ApiResponse,
    Order
};

use std::collections::HashMap;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get/manga/<id>")]
async fn get_manga(id: &str) -> ApiResponse {
    let result = manga::get(id).await;

    ApiResponse::resolve(result)
}

#[get("/get/author/<id>")]
async fn get_author(id: &str) -> ApiResponse {
    let result = author::get(id).await;

    ApiResponse::resolve(result)
}

#[get("/get/cover/<id>")]
async fn get_cover(id: &str) -> ApiResponse {
    let result = cover::retrieve(id, 512).await;

    ApiResponse::resolve_url(result)
}

#[get("/search/manga?<limit>&<offset>&<title>&<ids>&<authors>&<year>&<includedtags>&<includedtagsmode>&<excludedtags>&<excludedtagsmode>&<status>&<originallanguage>&<publicationdemographic>&<contentrating>&<createdatsince>&<updatedatsince>&<_order>")]
async fn search_manga(
    limit: Option<&str>, 
    offset: Option<&str>, 
    title: Option<&str>, 
    ids: Option<Vec<String>>, // ids should be Option<Vec<&str>>
    authors: Option<Vec<String>>,
    year: Option<i32>,
    includedtags: Option<Vec<String>>,
    includedtagsmode: Option<&str>,
    excludedtags: Option<Vec<String>>,
    excludedtagsmode: Option<&str>,
    status: Option<Vec<String>>,
    originallanguage: Option<Vec<String>>,
    publicationdemographic: Option<Vec<String>>,
    contentrating: Option<Vec<String>>,
    createdatsince: Option<&str>,
    updatedatsince: Option<&str>,
    _order: Order
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
    );
    
    let result = manga::search(query_params).await;

    ApiResponse::resolve(result)
}

#[get("/search/author/<_id>")]
async fn search_author(_id: &str) -> ApiResponse {
    todo!()
}

#[get("/search/chapter/<_id>")]
async fn search_chapter(_id: &str) -> ApiResponse {
    todo!()
}

#[get("/search/cover/<_id>")]
async fn search_cover(_id: &str) -> ApiResponse {
    todo!()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api/",
        routes![
            index,
            get_manga,
            get_author,
            get_cover,
            search_author,
            search_manga,
            search_chapter,
            search_cover
        ],
    )
}
