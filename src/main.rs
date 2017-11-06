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
use walkdir::{DirEntry, WalkDir};

/* Modules */
mod filesystem;
mod api;
mod utils;

/* Modules Use */
use filesystem::Entry;

#[get("/")]
fn qqc() -> Json<Entry> {
    Json(api::get_dir_full(&filesystem::get_parent_cdir()))
}


#[get("/")]
fn api() -> Json< Vec<Entry> > {
    let entries = api::get_dir_info(&filesystem::get_parent_cdir());
    Json(entries)
}

#[get("/<path..>")]
fn api_path(path: PathBuf) -> Json< Vec<Entry> > {
    let path = filesystem::get_parent_cdir().join(path);
    let entries = api::get_dir_info(&path);
    Json(entries)
}

#[get("/")]
fn home() -> Template {
    let entries = api::get_dir_info(&filesystem::get_parent_cdir());
    Template::render("index", entries)
}

#[get("/<path..>")]
fn path(path: PathBuf) -> Template {
    let path = filesystem::get_parent_cdir().join(path);
    let entries = api::get_dir_info(&path);
    Template::render("index", entries)
}

fn main() {
    rocket::ignite()
        //.manage(cfg)
        .mount("_api", routes![qqc])
        .mount("_fancyndex/dir/", routes![api, api_path])
        .mount("/home", routes![home, path])
        .attach(Template::fairing())
        .launch();
}
