pub mod parser;
mod tests;
use std::ops::{Deref, DerefMut};

use super::utils;
use parser::*;
use serde::Serialize;

pub type Error = ErrorAtribs;

impl utils::DexWrappedObject for Error {
    type Parser = ErrorAtribs;
    fn from_response(response: Self::Parser) -> Self {
        response
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorList(utils::DexListResponse<Error>);

impl utils::DexWrappedObject for ErrorList {
    type Parser = ErrorListResponse;

    fn from_response(response: Self::Parser) -> Self {
        ErrorList(utils::DexListResponse {
            total: response.errors.len(),
            results: response.errors,
        })
    }
}

impl Deref for ErrorList {
    type Target = utils::DexListResponse<Error>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ErrorList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
