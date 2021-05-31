use super::Chapter;
use std::fs;

#[allow(dead_code)]
fn load_test_responses() -> Chapter {
    let s = fs::read_to_string("example_responses\\chapter\\1.txt").unwrap();

    Chapter::from_string(s).unwrap()
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
