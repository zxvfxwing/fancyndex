#![crate_name = "fancyndex"]

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate walkdir;
extern crate toml;

use walkdir::{DirEntry, WalkDir};
use std::cmp;
use std::ffi::OsString;

use std::path::PathBuf;


use rocket_contrib::Json;
//use rocket::response::content::Json;

mod io;
mod config;
mod filesystem;

use config::Config;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

/// TEST
#[derive(Deserialize)]
pub struct InnerTest {
    what: String,
}

#[derive(Deserialize)]
pub struct Test {
    flag: bool,
    itest: InnerTest,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/*
* curl -H "Content-Type: application/json" -X POST -d '{"flag":true, "itest":{ "what": "truc"}}' http://localhost:8000/test
*/
#[post("/test", format = "application/json", data = "<test>")]
fn test(test: Json<Test>) -> &'static str {
    println!("{}", test.flag);
    println!("{}", &test.itest.what);
    "Test"
}


fn main() {
    let cfg = Config::new("Fancyndex.toml").check();

    rocket::ignite()
        .mount("/", routes![index, test])
        .launch();
    /*
    println!("{:}", cfg.root.path);
    println!("{:}", cfg.walk_opt.hidden);
    println!("{:}", cfg.walk_opt.symlink);

    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut files: Vec<PathBuf> = Vec::new();

    let walker = WalkDir::new(&cfg.root.path)
                            .min_depth(1)
                            .max_depth(1)
                            .follow_links(cfg.walk_opt.symlink)
                            // .sort_by(|a,b| a.metadata().unwrap().len().cmp( &b.metadata().unwrap().len() ))
                            // .sort_by(|a,b| a.file_name().cmp(b.file_name()))
                            .into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e) | cfg.walk_opt.hidden) {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    println!("{} -- {}", entry.path().display(), metadata.len());
                    dirs.push( entry.path().to_path_buf() )
                }
                else {
                     files.push( entry.path().to_path_buf() )
                }
            }
        }
    }

    for p in dirs.iter() {
        println!("{:?}", p.to_str())
    }

    for f in files.iter() {
        println!("{:?}", f.to_str())
    }
    */
}
