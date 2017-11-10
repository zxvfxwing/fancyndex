#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

/* Crates */
#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate walkdir;

/* -- Use -- */
/* STD Lib */
use std::path::{Path, PathBuf};
use std::cmp;
use std::ffi::OsString;

/* Rocket Web Framework Use */
use rocket_contrib::Template;
use rocket_contrib::Json;
use rocket::response::Redirect;
use rocket::response::NamedFile;

/* Walkdir */
/*use walkdir::{DirEntry, WalkDir}; */

/* Modules */
mod filesystem;
mod utils;

use filesystem::directory::Directory;
use filesystem::walkdir::WalkDir;

#[get("/")]
fn home() -> Json<Directory> {
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_symlink(true)
        .binary_unit(true)
        .do_hidden(true);

    Json(walker.run())
}

#[get("/<path..>")]
fn path(path: PathBuf) -> Json<Directory> {
    let path = filesystem::get_parent_cdir().join(path);
    let walker = WalkDir::init(&path)
        .do_symlink(true)
        .do_hidden(true);

    Json(walker.run())
}

fn main() {
    rocket::ignite()
        //.manage(cfg)
        //.mount("_api", routes![qqc])
        //.mount("_fancyndex/dir/", routes![api, api_path])
        //.mount("/home", routes![home, path])
        .mount("/", routes![home, path])
        .attach(Template::fairing())
        .launch();
}
