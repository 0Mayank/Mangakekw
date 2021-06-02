#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod dexrequest;
mod dexwrapper;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}