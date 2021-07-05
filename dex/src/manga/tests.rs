#[allow(unused_imports)]
use super::super::utils::Demographic;

use super::utils::DexWrappedObject;
use super::Manga;
#[allow(unused_imports)]
use std::borrow::Borrow;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn load_test_responses() -> Vec<Manga> {
    let path_to_example_responses = Path::new(".").join("example_responses").join("manga");
    [
        fs::read_to_string(path_to_example_responses.join("opm.txt").to_str().unwrap()).unwrap(),
        fs::read_to_string(
            path_to_example_responses
                .join("slime.txt")
                .to_str()
                .unwrap(),
        )
        .unwrap(),
        fs::read_to_string(path_to_example_responses.join("solo.txt").to_str().unwrap()).unwrap(),
    ]
    .iter()
    .map(|response_text| Manga::from_string(response_text).unwrap())
    .collect()
}
#[test]
pub fn manga_name() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    assert_eq!(opm.title.get("en").unwrap(), "One Punch-Man");
    assert_eq!(
        slime.title.get("en").unwrap(),
        "Tensei Shitara Slime Datta Ken"
    );
    assert_eq!(solo.title.get("en").unwrap(), "Solo Leveling");
}

#[test]
pub fn manga_id() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    assert_eq!(opm.id, "d8a959f7-648e-4c8d-8f23-f1f3f8e129f3");
    assert_eq!(slime.id, "e78a489b-6632-4d61-b00b-5206f5b8b22b");
    assert_eq!(solo.id, "32d76d19-8a05-4db0-9fc2-e0b0648fe9d0");
}

#[test]
pub fn manga_demographic() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    match opm.publication_demographic.borrow() {
        Some(Demographic::Seinen) => {}
        _ => panic!("Wrong Demographic of OPM"),
    }

    match slime.publication_demographic.borrow() {
        Some(Demographic::Shounen) => {}
        _ => panic!("Wrong Demographic of slime"),
    }

    match solo.publication_demographic.borrow() {
        Some(Demographic::Shounen) => {}
        _ => panic!("Wrong Demographic of solo"),
    }
}

#[test]
pub fn manga_status() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    match opm.status {
        super::super::utils::Status::Ongoing => {}
        _ => panic!("Wrong Status"),
    }

    match slime.status {
        super::super::utils::Status::Ongoing => {}
        _ => panic!("Wrong Status"),
    }

    match solo.status {
        super::super::utils::Status::Ongoing => {}
        _ => panic!("Wrong Status"),
    }
}

#[test]
pub fn manga_rating() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    match opm.content_rating {
        super::super::utils::ContentRating::Safe => {}
        _ => panic!("Wrong ContentRating"),
    }

    match slime.content_rating {
        super::super::utils::ContentRating::Safe => {}
        _ => panic!("Wrong ContentRating"),
    }

    match solo.content_rating {
        super::super::utils::ContentRating::Safe => {}
        _ => panic!("Wrong ContentRating"),
    }
}

#[test]
pub fn manga_author() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    assert_eq!(opm.author_id, "16b98239-6452-4859-b6df-fdb1c7f12b52");
    assert_eq!(slime.author_id, "dbf8af05-7173-49f3-bf60-f4ea3f586486");
    assert_eq!(solo.author_id, "820b13ef-dc7d-42b1-999a-65393b8b4040");
}

#[test]
pub fn manga_artist() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    assert_eq!(opm.artist_id, "47cd4e57-3fc4-4d76-97e4-b3933a5b05ef");
    assert_eq!(slime.artist_id, "560748c6-fbe7-49f5-8258-7b3292942101");
    assert_eq!(solo.artist_id, "86f43f7f-7f32-4ecb-8dd9-7cd2ae16932b");
}

#[test]
pub fn manga_cover() {
    let test_responses = load_test_responses();

    let opm = test_responses.get(0).unwrap();
    let slime = test_responses.get(1).unwrap();
    let solo = test_responses.get(2).unwrap();

    assert_eq!(opm.cover_id, "f1bf8066-d8fc-4fc8-aea3-5b382ba4ee2f");
    assert_eq!(slime.cover_id, "0ecefb70-e710-4f5a-94c0-d3a01db0a5a5");
    assert_eq!(solo.cover_id, "b6c7ce9c-e671-4f26-90b0-e592188e9cd6");
}
