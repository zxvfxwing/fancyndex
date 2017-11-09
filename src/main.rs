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
mod utils;

use filesystem::directory::Directory;

#[get("/")]
fn home() {
    let path = filesystem::get_parent_cdir();

    if let Ok(entries) = path.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {

                let mut size = 0u64;
                let mut elts = 0u64;

                let walker = WalkDir::new(entry.path()).follow_links(true).into_iter();

                for entry in walker {
                    if let Ok(entry) = entry {
                        // println!("{}", entry.path().display());
                        if entry.file_type().is_file() {
                            size += entry.metadata().unwrap().len();
                        }
                        elts+=1;
                    }
                }

                println!("{} -- {}, {}", entry.path().display(), size, elts);
            }
        }
    }
}

#[get("/<path..>")]
fn path(path: PathBuf) {



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
