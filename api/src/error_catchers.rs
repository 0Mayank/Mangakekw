use rocket::Request;

#[catch(404)]
pub fn does_not_exist<'a>(req: &Request) -> String {
    let origin = req.uri();
    let path = origin.path().to_string();
    let query = match origin.query() {
        Some(query) => query.to_string(),
        None => String::from("")
    };
    let uri: String = format!("{} is not a valid path.", path);
    
    json!({
        "error" : {
            "short" : uri,
            "long" : {
                "path": path,
                "query": query,
                "desc": "Path doesn't exist"
            }
        }
    }).to_string()
}