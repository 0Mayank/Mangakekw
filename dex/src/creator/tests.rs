use super::utils::DexWrappedObject;
use super::CreatorTemplate;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
fn load_test_responses() -> Vec<CreatorTemplate> {
    let path_to_test_respones = Path::new(".").join("example_responses").join("creator");
    eprintln!("{}", path_to_test_respones.to_str().unwrap());
    [
        fs::read_to_string(path_to_test_respones.join("murata.txt").to_str().unwrap()).unwrap(),
        fs::read_to_string(path_to_test_respones.join("fuse.txt").to_str().unwrap()).unwrap(),
        fs::read_to_string(path_to_test_respones.join("chugong.txt").to_str().unwrap()).unwrap(),
    ]
    .iter()
    .map(|response_text| CreatorTemplate::from_string(response_text).unwrap())
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
