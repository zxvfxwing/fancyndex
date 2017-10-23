#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
//#![warn(unused_imports)]

extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::response::Redirect;
// use rocket::http::RawStr;

#[get("/")]
fn home() -> Redirect {
    Redirect::to("/home")
}

#[get("/<user_path..>")]
fn user_path(user_path: PathBuf) -> String {

    let path = Path::new("/").join(&user_path);

    if path.exists() {
        println!("!YOUHOU");
    }

    format!("user_path {}, path: {}", user_path.display(), path.display())
}

fn main() {
    rocket::ignite().mount("/", routes![home, user_path]).launch();
}
