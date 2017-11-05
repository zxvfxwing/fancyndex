#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate toml;

extern crate rocket;
extern crate rocket_contrib;

//extern crate crossbeam;
extern crate chrono;

use std::path::{Path, PathBuf};

//use rocket::State;

use rocket_contrib::Template;
use rocket_contrib::Json;

use rocket::response::Redirect;
use rocket::response::NamedFile;

use rocket::http::Header;

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

/* Normal route */
#[get("/home")]
fn home() -> Template {
    let path = filesystem::get_parent_current_dir();

    println!("{}", path.display());

    //let dir = directory::Directory::new(&path);

    let mut v: Vec<Context> = Vec::new();

    let vone = Context {
        name: String::from("This is an example"),
        number: 42
    };

    v.push(vone);

    let essaie = TemplateContext {
        vecf: v
    };

    Template::render("index", essaie)
}

#[get("/home/<user_path..>")]
fn user_path(user_path: PathBuf) -> Result<Template, Redirect> {

    let mut path = filesystem::get_parent_current_dir();
    path.push(&user_path);

    match path.exists() {
        true => {
            match path.is_dir() {
                true => {
                    let mut v: Vec<Context> = Vec::new();

                    let vone = Context {
                        name: String::from("This is an example"),
                        number: 42
                    };

                    v.push(vone);

                    let essaie = TemplateContext {
                        vecf: v
                    };

                    Ok(Template::render("index", essaie))
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

    Header::new("Cache-Control", "max-age=650000");

    rocket::ignite()
        //.manage(cfg)
        .mount("/", routes![index, home, user_path, download_home, download_file])
        .mount("/_fancyndex/", routes![home_path, wanted_path, www, home_dir, wanted_dir])
        .attach(Template::fairing())
        .launch();
}
