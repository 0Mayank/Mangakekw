#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

mod utils;

use dex::request::author;
use dex::request::cover;
use dex::request::manga;
use utils::meta::ApiResponse;

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

#[get("/search/manga/<_id>")]
async fn search_manga(_id: &str) -> ApiResponse {
    todo!()
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
