
use std::{error::Error, str};

#[derive(Debug)]
pub struct MangadexReq {
    query: String,
}

impl MangadexReq {
    pub fn new(query: &str) -> MangadexReq {
        MangadexReq {
            query: String::from(query)
        }
    }

    pub fn get(self) -> Result<String, Box<dyn Error>> {
        match  get_data(self.query) {
            Ok(s) => Ok(s),
            Err(e) => return Err(e)
        }
    }
}

fn get_data(query: String) -> Result<String, Box<dyn std::error::Error>> {
    let json_text = reqwest::blocking::get(format!("https://api.mangadex.org/{}", query))?.text()?;

    Ok(json_text)
}