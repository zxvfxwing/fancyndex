#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate toml;

extern crate rocket;
extern crate rocket_contrib;

extern crate chrono;

use std::path::{Path, PathBuf};
//use rocket::State;
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::response::NamedFile;

//use std::io;
//use std::io::prelude::*;
//use std::fs::File;
//use std::io::BufReader;
//use std::process;

//use std::collections::HashMap;

mod filesystem;
use filesystem::directory;

#[derive(Serialize)]
struct TemplateContext {
    vecf: Vec<Context>
}

#[derive(Serialize)]
struct Context {
    name: String,
    number: i32
}

#[get("/www/<file..>")]
fn www(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/")]
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

#[get("/path")]
fn home_path() -> String {
    let path = filesystem::get_parent_current_dir();

    if !path.exists() {
        /* Redirection */
    }

    let dir = directory::Directory::new(&path);

    format!("home_path: {}", path.display())
}

#[get("/path/<wanted_path..>")]
fn wanted_path(wanted_path: PathBuf) -> String {
    let mut path = filesystem::get_parent_current_dir();
    path.push(&wanted_path);

    if !path.exists() {
        /* Redirection */
    }

    /* Redirection */
    if path.is_file() {
        println!("{}", path.display());
    }

    let dir = directory::Directory::new(&path);
    /*
        println!("{}", dir.name());
        println!("{}", dir.size());
        println!("{}", dir.datetime());
        println!("{}", dir.nb_total_files());
    */

    for x in dir.directories() {
        //println!("{} - {} - {} - {}", x.name(), x.size(), x.datetime(), x.timestamp());
    }

    for y in dir.files() {
        //println!("{} - {} - {} - {}", y.name(), y.size(), y.datetime(), y.timestamp());
    }

    format!("wanted_path: {}, path: {}", wanted_path.display(), path.display())
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
        .mount("/", routes![home])
        .mount("/fancyndex/", routes![home_path, wanted_path, www])
        .attach(Template::fairing())
        .launch();
}
