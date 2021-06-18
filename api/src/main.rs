#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use rocket::response::{Responder, Response};
use rocket::{http, request::Request, response};
use serde::Serialize;
use std::io::Cursor;

use dex::wrapper::utils::DexWrappedObject;
use dex::request::manga;
use dex::wrapper::utils::DexError;

struct ApiResponse {
    body: String,
    status: http::Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .header(http::ContentType::JSON)
            .sized_body(self.body.len(), Cursor::new(self.body))
            .ok()
    }
}

fn resolve<T: DexWrappedObject + Serialize>(result: Result<T, DexError>) -> ApiResponse {
    match result {
        Ok(resp) => ApiResponse {
            body: <T as DexWrappedObject>::serialize(&resp ,false).unwrap(), // cannot return error for this
            status: http::Status::Ok,
        },
        Err(e) => match e {
            DexError::InvalidRequest(error_list) => {
                ApiResponse {
                    body: json!(error_list.results).to_string(),
                    status: http::Status::NotFound,
                }
            },
            DexError::InvalidSchema => {
                ApiResponse {
                    body: json!({
                    "error": {
                        "short" : "Internal server error",
                        "long": "Schema of response received by server was not recognized"
                        }
                    }).to_string(),
                    status: http::Status::InternalServerError,
                }
            },
        },
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get_manga/<id>")]
async fn get_manga(id: &str) -> ApiResponse {
    let result = manga::get(id).await;

    resolve(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![index, get_manga]) //? why base /api??
}
