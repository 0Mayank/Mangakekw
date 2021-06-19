#[macro_use]
extern crate rocket;

use api::meta::ApiResponse;
use dex::request::author;
use dex::request::cover;
use dex::request::manga;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![index, get_manga, get_author, get_cover])
}
