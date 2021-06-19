#[macro_use]
extern crate rocket;

use api::meta::{resolve, ApiResponse};
use dex::request::manga;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get-manga/<id>")]
async fn get_manga(id: &str) -> ApiResponse {
    let result = manga::get(id).await;

    resolve(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![index, get_manga]) //? why base /api??
}
