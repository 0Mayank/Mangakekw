#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use rocket::response::{Responder, Response};
use rocket::{http, request::Request, response};
use rocket_dyn_templates::handlebars::JsonValue;

use dex::request::manga;
use dex::wrapper::utils::DexError;

struct ApiResponse {
    body: JsonValue,
    status: http::Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.body.respond_to(&request).unwrap())
            .status(self.status)
            .header(http::ContentType::JSON)
            .ok()
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get_manga/<id>")]
async fn get_manga(id: &str) -> ApiResponse {
    let result = manga::get(id).await;

    match result {
        Ok(manga) => ApiResponse {
            body: json!(manga),
            status: http::Status::Ok,
        },
        Err(e) => match e {
            DexError::InvalidRequest(error_list) => ApiResponse {
                body: json!(error_list.results),
                status: http::Status::BadRequest,
            },
            DexError::InvalidSchema => ApiResponse {
                body: json!({
                "error": {
                    "short" : "Internal server error",
                    "long": "Schema of response received by server was not recognized"
                    }
                }),
                status: http::Status::InternalServerError,
            },
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![index, get_manga])
}
