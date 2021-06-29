pub mod meta {    
    use dex::wrapper::error::ErrorList as DexErrorList;
    use rocket::response::{Responder, Response};
    use rocket::{http, request::Request, response};
    use std::io::Cursor;

    use dex::wrapper::utils::DexError;
    use dex::wrapper::utils::DexWrappedObject;
    pub struct ApiResponse {
        pub body: String,
        pub status: http::Status,
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

    impl ApiResponse {
        pub fn resolve<T: DexWrappedObject>(result: Result<T, DexError>) -> ApiResponse {
            match result {
                Ok(resp) => ApiResponse {
                    body: <T as DexWrappedObject>::serialize(&resp, false).unwrap(), // cannot return error for this
                    status: http::Status::Ok,
                },
                Err(e) => Self::handle_error(e),
            }
        }

        pub fn resolve_url(result: Result<String, DexError>) -> ApiResponse {
            match result {
                Ok(resp) => ApiResponse {
                    body: json!({ "url": resp }).to_string(),
                    status: http::Status::Ok,
                },
                Err(e) => Self::handle_error(e),
            }
        }

        pub fn handle_error(error: DexError) -> ApiResponse {
            match error {
                DexError::InvalidRequest(error_list) => ApiResponse {
                    body: <DexErrorList as DexWrappedObject>::serialize(&error_list, false)
                        .unwrap(),
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
            }
        }
    }

    #[derive(FromForm)]
    pub struct Order<'a> {
        pub createdat: Option<&'a str>,
        pub updatedat: Option<&'a str>
    }
}

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
    };
}
