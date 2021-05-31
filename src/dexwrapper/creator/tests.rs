use super::CreatorTemplate;
use std::fs;

#[allow(dead_code)]
fn load_test_responses() -> Vec<CreatorTemplate> {
    [
        fs::read_to_string("example_responses\\creator\\murata.txt").unwrap(),
        fs::read_to_string("example_responses\\creator\\fuse.txt").unwrap(),
        fs::read_to_string("example_responses\\creator\\chugong.txt").unwrap(),
    ]
    .iter()
    .map(|response_text| CreatorTemplate::from_string(response_text.to_string()).unwrap())
    .collect()
}
#[test]
pub fn creator_name() {
    let test_responses = load_test_responses();

    let murata = test_responses.get(0).unwrap();
    let fuse = test_responses.get(1).unwrap();
    let chugong = test_responses.get(2).unwrap();

    assert_eq!(murata.name, "Murata Yuusuke");
    assert_eq!(chugong.name, "Chugong 추공");
    assert_eq!(fuse.name, "Fuse");
}

#[test]
pub fn creator_id() {
    let test_responses = load_test_responses();

    let murata = test_responses.get(0).unwrap();
    let fuse = test_responses.get(1).unwrap();
    let chugong = test_responses.get(2).unwrap();

    assert_eq!(murata.id, "47cd4e57-3fc4-4d76-97e4-b3933a5b05ef");
    assert_eq!(chugong.id, "820b13ef-dc7d-42b1-999a-65393b8b4040");
    assert_eq!(fuse.id, "dbf8af05-7173-49f3-bf60-f4ea3f586486");
}
