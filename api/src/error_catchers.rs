use rocket::Request;

#[catch(404)]
pub fn does_not_exist<'a>(req: &Request) -> String {
    let uri: String = format!("{} is not a valid path.", req.uri().path());
    let desc = req.to_string(); // change to something more meaningful
    
    json!({
        "error" : {
            "short" : uri,
            "long" : desc
        }
    }).to_string()
}