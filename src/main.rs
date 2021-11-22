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

#[get("/path/<param>")]
fn list_files(param: &str) -> Json<Vec<FileDecr>> {
    let mut v: Vec<FileDecr> = Vec::new();
    print!("{}",param);
    let dirs = read_dir(param).unwrap();
    for i in dirs {
        v.push(FileDecr::extra_meta(i.unwrap()));
    }
    Json(v)
}
