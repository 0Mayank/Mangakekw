
use std::error::Error;

#[derive(Debug)]
pub struct MangadexReq {
    base: String,
}

impl MangadexReq {
    pub fn new(base: &str) -> MangadexReq {
        MangadexReq {
            base: String::from(base)
        }
    }

    pub fn get(&self, query: &str) -> Result<String, Box<dyn Error>> {
        match  get_data(&self.base, query) {
            Ok(s) => Ok(s),
            Err(e) => return Err(e)
        }
    }
}

fn get_data(base: &str, query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json_text = reqwest::blocking::get(format!("{}/{}", base, query))?.text()?;

    Ok(json_text)
}