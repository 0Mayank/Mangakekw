#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use dex::wrapper::error::ErrorList as DexErrorList;
use rocket::response::{Responder, Response};
use rocket::{http, request::Request, response};
use std::io::Cursor;

use dex::request::manga;
use dex::wrapper::utils::DexError;
use dex::wrapper::utils::DexWrappedObject;

struct ApiResponse {
    body: String,
    status: http::Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .header(http::ContentType::JSON)
            .raw_header("server", "Mangakekw using Rocket") //default is "Rocket"
            // .raw_header("x-frame-options", "DENY") // refer to <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options>
            .sized_body(self.body.len(), Cursor::new(self.body))
            .ok()
    }
}

fn resolve<T: DexWrappedObject>(result: Result<T, DexError>) -> ApiResponse {
    match result {
        Ok(resp) => ApiResponse {
            body: <T as DexWrappedObject>::serialize(&resp, false).unwrap(), // cannot return error for this
            status: http::Status::Ok,
        },
        Err(e) => match e {
            DexError::InvalidRequest(error_list) => ApiResponse {
                // body: json!(error_list.results).to_string(),
                body: <DexErrorList as DexWrappedObject>::serialize(&error_list, false).unwrap(),
                status: http::Status::NotFound,
            },
            DexError::InvalidSchema => ApiResponse {
                body: json!({
                "error": {
                    "short" : "Internal server error",
                    "long": "Schema of response received by server was not recognized"
                    }
                })
                .to_string(),
                status: http::Status::InternalServerError,
            },
        },
    }
}

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
