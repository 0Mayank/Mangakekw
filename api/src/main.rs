#[cfg(test)] mod tests;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod utils;

mod routes;

use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api/",
        routes![
            index,
            get_manga,
            get_author,
            get_cover,
            get_chapter,
            search_author,
            search_manga,
            search_chapter,
            search_cover,
            chapter_retrieve,
            cover_retrieve
        ],
    )
}
