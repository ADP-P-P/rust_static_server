mod file_decr;

#[macro_use]
extern crate rocket;
extern crate json;

use crate::file_decr::FileDecr;
use rocket::fs::FileServer;
use rocket::serde::json::{Json};
use std::fs::read_dir;

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/", routes![list_files])
        .mount("/home", FileServer::from("client/"))
}

#[get("/path?<path>")]
fn list_files(path: &str) -> Json<Vec<FileDecr>> {
    let mut v: Vec<FileDecr> = Vec::new();
    print!("{}",path);
    let dirs = read_dir(path).unwrap();
    for i in dirs {
        v.push(FileDecr::extra_meta(i.unwrap()));
    }
    Json(v)
}
