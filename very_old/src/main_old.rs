#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate toml;

//extern crate crossbeam;
extern crate chrono;
extern crate walkdir;

use std::path::{Path, PathBuf};

//use rocket::State;

use rocket_contrib::Template;
use rocket_contrib::Json;

use rocket::response::Redirect;
use rocket::response::NamedFile;

use rocket::http::Header;

use walkdir::{DirEntry, WalkDir};
use std::cmp;
use std::ffi::OsString;

//use std::io;
//use std::io::prelude::*;
//use std::fs::File;
//use std::io::BufReader;
//use std::process;

//use std::collections::HashMap;

mod filesystem;
use filesystem::directory;

mod api;

#[derive(Serialize)]
struct TemplateContext {
    vecf: Vec<Context>,
}

#[derive(Serialize)]
struct Context {
    name: String,
    number: i32
}

/* API Fancyndex */
#[get("/www/<file..>")]
fn www(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/dl")]
fn download_home() -> Redirect {
    Redirect::to("/home")
}

#[get("/dl/file/<file..>")]
fn download_file(file: PathBuf) -> Result<Option<NamedFile>, Redirect> {
    let mut path = filesystem::get_parent_current_dir();
    path.push(file);

    match path.exists() {
        true => {
            match path.is_file() {
                true => {
                    Ok(NamedFile::open(path).ok())
                },
                false => {
                    Err(Redirect::to("/home"))
                }
            }
        },
        false => {
            Err(Redirect::to("/home"))
        }
    }
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/home")
}

fn is_hidden(entry: &DirEntry, exe: bool) -> bool {
    if !exe { return false; }

    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)

}

// 679603584
// 579763947

/* Normal route */
#[get("/home")]
fn home() -> Template {
    let path = filesystem::get_parent_current_dir();

    let mut size = 0u64;
    let mut nb_elements = 0u64;

    let ok = true;

    let walker = WalkDir::new(&path)
                //.sort_by(|a,b| a.file_name().cmp(b.file_name()))
                .follow_links(true)
                .into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e, true))
    {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                size += entry.path().metadata().unwrap().len();
            }
            nb_elements+=1;
        }
    }

    println!("{} - {}", size, nb_elements);
    Template::render("index", api::full(&path))
}

#[get("/home/<user_path..>")]
fn user_path(user_path: PathBuf) -> Result<Template, Redirect> {

    let mut path = filesystem::get_parent_current_dir();
    path.push(&user_path);

    match path.exists() {
        true => {
            match path.is_dir() {
                true => {
                    Ok(Template::render("index", api::full(&path)))
                },
                false => {
                    let route = format!("/dl/{}", user_path.display());
                    Err(Redirect::to(&route[..]))
                }
            }
        },
        false => {
            Err(Redirect::to("/home"))
        }
    }
}

/* API Fancyndex */

#[get("/dir")]
fn home_dir() -> Json<api::DirJSON> {
    let path = filesystem::get_parent_current_dir();
    api::dir_light_info(&path)
}

#[get("/dir/<wanted_dir..>")]
fn wanted_dir(wanted_dir: PathBuf) -> Result<Json<api::DirJSON>, Redirect> {
    let mut path = filesystem::get_parent_current_dir();
    path.push(&wanted_dir);

    /* Redirect if path given doesn't exists or redirect to download if it's a file */
    match path.exists() {
        true => {
            match path.is_dir() {
                false => {
                    Err(Redirect::to("/_fancyndex/dir/"))
                },
                true => Ok(api::dir_light_info(&path))
            }
        },
        false => Err(Redirect::to("/_fancyndex/dir"))
    }
}


#[get("/path")]
fn home_path() -> Json<api::FullJSON> {
    let path = filesystem::get_parent_current_dir();
    api::list_full_dir(&path)
}

#[get("/path/<wanted_path..>")]
fn wanted_path(wanted_path: PathBuf) -> Result<Json<api::FullJSON>, Redirect> {
    let mut path = filesystem::get_parent_current_dir();
    path.push(&wanted_path);

    /* Redirect if path given doesn't exists or redirect to download if it's a file */
    match path.exists() {
        true => {
            match path.is_dir() {
                false => {
                    let route = format!("/dl/file/{}", wanted_path.display());
                    Err(Redirect::to(&route[..]))
                },
                true => Ok(api::list_full_dir(&path))
            }
        },
        false => Err(Redirect::to("/_fancyndex/path"))
    }
}

/*
#[derive(Deserialize)]
struct Config {
    home: String,
}

fn init_cfg_file(filename: &str) -> Config {
    match read_file(filename) {
        Ok(s) => toml::from_str(&s[..]).unwrap(),
        Err(e) => {
            println!("Error: {}", e.to_string());
            process::exit(1)
        },
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let cfg_file = File::open(filename)?;
    let mut buf_reader = BufReader::new(cfg_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
*/

fn main() {
    //let cfg = init_cfg_file("/home/spoken/Git/fancyndex/src/config.toml");

    rocket::ignite()
        //.manage(cfg)
        .mount("/", routes![index, home, user_path, download_home, download_file])
        .mount("/_fancyndex/", routes![home_path, wanted_path, www, home_dir, wanted_dir])
        .attach(Template::fairing())
        .launch();
}
