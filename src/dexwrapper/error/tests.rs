#[allow(unused_imports)]
use super::super::manga::Manga;
#[allow(unused_imports)]
use super::utils::DexWrappedObject;
#[allow(unused_imports)]
use std::fs;

#[test]
#[allow(unused_must_use)]
#[should_panic]
pub fn invalid_json() {
    Manga::from_string("{}");
}

#[test]
pub fn error_json() {
    let r = Manga::from_string(&fs::read_to_string("example_responses\\error\\1.txt").unwrap());
    match r {
        Ok(_) => panic!("Returned Ok on error json."),
        Err(e) => {
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
