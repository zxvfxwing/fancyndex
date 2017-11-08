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

/*

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

*/

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn main() {
    /*rocket::ignite()
        //.manage(cfg)
        //.mount("_api", routes![qqc])
        //.mount("_fancyndex/dir/", routes![api, api_path])
        //.mount("/home", routes![home, path])
        .attach(Template::fairing())
        .launch();
    */

    //let mut dir = filesystem::directory::Directory::new_one(&filesystem::get_parent_cdir());
    //dir.deep_run();

    let path = filesystem::get_parent_cdir();

    let do_hidden = true;
    let do_symlink = true;

    let mut size = 0u64;
    let mut elts = 0u64;
    let walker = WalkDir::new(path)
                        .follow_links(do_symlink)
                        .into_iter();

    /*
    *
    * The predicate is applied to all entries.
    * If the predicate is true, iteration carries on as normal.
    * If the predicate is false, the entry is ignored and if it is a directory,
    * it is not descended into.
    *
    * !is_hidden | do_hidden
    *    true    |    true    => true
    *   false    |    true    => true
    *   false    |   false    => false (here filter does his job)
    *    true    |   false    => true
    *
    */

    for entry in walker.filter_entry(|e| !is_hidden(e) | do_hidden ) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                size += entry.metadata().unwrap().len();
            }
            elts+=1;
            //println!("{}", entry.path().display());
        }
    }

    println!("{} - {}", size, elts);
}
