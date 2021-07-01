use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;

use std::fs;
use std::path::Path;

#[test]
fn search_manga() {
    let uri = 
    "/api/search/manga?limit=3&includedtags=0bc90acb-ccc1-44ca-a34a-b9f3a73259d0&includedtags=292e862b-2d17-4062-90a2-0356caa4ae27&includedtagsmode=OR&contentrating=suggestive&order[createdat]=desc";
    let path = "search_manga";
    let status = Status::Ok;

    do_test(uri, path, status);
}

#[test]
fn get_manga() {
    let uri = "/api/get/manga/d8d6a0c7-6ef0-4b8f-a71c-3bd89395a71b";
    let path = "get_manga";
    let status = Status::Ok;

    do_test(uri, path, status);
}

#[test]
fn get_author() {
    let uri = "/api/get/author/15fe4d54-ae08-4177-af94-868bb7db1bcf";
    let path = "get_author";
    let status = Status::Ok;

    do_test(uri, path, status);
}

#[test]
fn get_chapter() {
    let uri = "/api/get/chapter/f5ec5e4f-2c95-48ca-b3f9-8e9ed6227928";
    let path = "get_chapter";
    let status = Status::Ok;

    do_test(uri, path, status);
}

#[test]
fn get_cover() {
    let uri = "/api/get/cover/e3d51092-6377-4f5a-8691-d9cec1adf640";
    let path = "get_cover";
    let status = Status::Ok;

    do_test(uri, path, status);
}

#[test]
fn chapter_retrieve() {
    let uri = "/api/chapter/retrieve/d0723f93-ff71-43f8-b9ef-57fb14d00e35";
    let path = "chapter_retrieve";
    let status = Status::Ok;

    do_test(uri, path, status);
}

#[test]
fn cover_retrieve() {
    let uri = "/api/cover/retrieve/e3d51092-6377-4f5a-8691-d9cec1adf640";
    let path = "cover_retrieve";
    let status = Status::Ok;

    do_test(uri, path, status);
}

fn do_test(uri: &str, path: &str, status: Status) {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    let response = client.get(uri).dispatch();

    assert_eq!(response.status(), status);
    assert_eq!(response.into_string().unwrap(), load_test_response(path));
}

fn load_test_response(path: &str) -> String{
    let path = Path::new(".").join("example_responses").join(path);
    
    fs::read_to_string(path).unwrap()
}