#[allow(unused_imports)]
use chrono::{Datelike, Timelike};

use super::utils::DexWrappedObject;
use super::Chapter;
use std::fs;

#[allow(dead_code)]
fn load_test_responses() -> Chapter {
    let s = fs::read_to_string("example_responses\\chapter\\1.txt").unwrap();

    Chapter::from_string(&s).unwrap()
}
#[test]
pub fn chapter_title() {
    let test_response = load_test_responses();

    assert_eq!(test_response.title, "Oneshot");
}

#[test]
pub fn chapter_hash() {
    let test_response = load_test_responses();

    assert_eq!(test_response.hash, "e199c7d73af7a58e8a4d0263f03db660");
}

#[test]
pub fn chapter_volume() {
    let test_response = load_test_responses();

    assert_eq!(test_response.volume, None);
}

#[test]
pub fn chapter_created_at() {
    let test_response = load_test_responses();

    assert_eq!(test_response.created_at.minute(), 22);
    assert_eq!(test_response.created_at.second(), 14);
    assert_eq!(test_response.created_at.hour(), 13);

    assert_eq!(test_response.created_at.date().month().to_string(), "1");
    assert_eq!(test_response.created_at.year(), 2018);
}

#[test]
pub fn chapter_published_at() {
    let test_response = load_test_responses();

    assert_eq!(test_response.published_at.minute(), 22);
    assert_eq!(test_response.published_at.second(), 14);
    assert_eq!(test_response.published_at.hour(), 13);

    assert_eq!(test_response.published_at.date().month().to_string(), "1");
    assert_eq!(test_response.published_at.year(), 2018);
}

#[test]
pub fn chapter_updated_at() {
    let test_response = load_test_responses();

    assert_eq!(test_response.updated_at.minute(), 22);
    assert_eq!(test_response.updated_at.second(), 14);
    assert_eq!(test_response.updated_at.hour(), 13);

    assert_eq!(test_response.updated_at.date().month().to_string(), "1");
    assert_eq!(test_response.updated_at.year(), 2018);
}
