#[allow(unused_imports)]
use chrono::{Datelike, Timelike};

use super::utils::DexWrappedObject;
use super::Cover;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn load_test_responses() -> Cover {
    let path_to_test_responses = Path::new(".")
        .join("example_responses")
        .join("cover")
        .join("1.txt");
    let s = fs::read_to_string(path_to_test_responses.to_str().unwrap()).unwrap();

    Cover::from_string(&s).unwrap()
}
#[test]
pub fn cover_id() {
    let test_response = load_test_responses();

    assert_eq!(test_response.id, "c613ff43-aff9-4fd0-9477-9efaae2b8e01");
}

#[test]
pub fn cover_desc() {
    let test_response = load_test_responses();

    assert_eq!(test_response.description.unwrap(), "");
}

#[test]
pub fn cover_volume() {
    let test_response = load_test_responses();

    assert_eq!(test_response.volume.unwrap(), "4");
}

#[test]
pub fn cover_file_name() {
    let test_response = load_test_responses();

    assert_eq!(
        test_response.file_name,
        "81adb500-792b-47ae-a1ae-d0170aca5eba.jpg"
    );
}
