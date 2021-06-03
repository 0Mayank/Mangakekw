pub mod parser;
mod tests;
use super::utils;
use parser::*;

pub type Error = ErrorAtribs;

impl utils::DexWrappedObject for Error {
    type Parser = ErrorAtribs;
    fn from_response(response: Self::Parser) -> Self {
        response
    }
}

pub type ErrorList = utils::DexListResponse<Error>;

impl utils::DexWrappedObject for ErrorList {
    type Parser = ErrorListResponse;
    #[allow(dead_code)]
    fn from_response(response: Self::Parser) -> Self {
        ErrorList {
            total: response.errors.len(),
            results: response.errors,
        }
    }
}
