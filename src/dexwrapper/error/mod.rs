pub mod parser;
use super::utils;
use parser::*;

pub type Error = ErrorAtribs;

impl utils::DexWrappedObject for Error {
    type Response = ErrorAtribs;
    fn from_response(response: Self::Response) -> Self {
        response
    }
}

pub type ErrorList = utils::DexListResponse<Error>;

impl utils::DexWrappedObject for ErrorList {
    type Response = ErrorListResponse;
    #[allow(dead_code)]
    fn from_response(response: Self::Response) -> Self {
        ErrorList {
            total: response.errors.len(),
            results: response.errors,
        }
    }
}
