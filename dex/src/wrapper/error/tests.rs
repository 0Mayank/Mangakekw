#[allow(unused_imports)]
use super::super::manga::Manga;
#[allow(unused_imports)]
use super::utils::DexError;
#[allow(unused_imports)]
use super::utils::DexWrappedObject;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::path::Path;

#[test]
#[allow(unused_must_use)]
pub fn invalid_json() {
    if let Err(e) = Manga::from_string("{}") {
        match e {
            DexError::InvalidSchema => (),
            DexError::InvalidRequest(_) => {
                panic!("Wrong type of error returned, expected InvalidSchema got InvalidRequest(_)")
            }
        }
    }
}

#[test]
pub fn error_json() {
    let path_to_example_response = Path::new(".")
        .join("example_responses")
        .join("error")
        .join("1.txt");
    let r = Manga::from_string(
        &fs::read_to_string(path_to_example_response.to_str().unwrap()).unwrap(),
    );
    match r {
        Ok(_) => panic!("Returned Ok on error json."),
        Err(e) => {
            if let DexError::InvalidRequest(e) = e {
                for error in e.results.iter() {
                    assert_eq!(error.status, 404);
                    assert_eq!(error.id, "524eb5f9-8408-5783-b4fc-5b176572e923");
                    assert_eq!(error.title, "not_found_http_exception");
                    assert_eq!(
                        error.detail,
                        "No route found for \"GET /author/dbf8af05-71as73-49f3-bf60-f4ea3f586486\""
                    );
                }
            }
        }
    }
}
