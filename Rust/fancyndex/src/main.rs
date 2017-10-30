#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
#![warn(unused_imports)]
#![warn(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate rocket;
extern crate rocket_contrib;

extern crate chrono;

use std::path::{Path, PathBuf};
use rocket::State;
use rocket_contrib::Template;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::process;

use std::collections::HashMap;

use chrono::prelude::*;


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

/*
#[derive(Serialize)]
struct Folder {
    dirs: Vec<Dir>,
    files: Vec<File>
}
*/

#[get("/")]
fn home(cfg: State<Config>) -> Template {

    let path = Path::new(&cfg.home[..]);

    if path.exists() && path.is_dir() {
        println!("path found!");
    }

    println!("cfg_home: {}, path: {}", cfg.home, path.display());

    let mut context = HashMap::new();
    context.insert("home", &cfg.home);

    let mut v: Vec<Context> = Vec::new();

    let vone = Context {
        name: String::from("This is an example"),
        number: 42
    };

    v.push(vone);

    for entry in path.read_dir().expect("read_dir call failed") {

        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(time) = metadata.modified() {
                    let datetime: DateTime<Local> = time.into();
                    println!("{}", datetime.format("%Y-%m-%d %T"));
                }

                println!("{} => {} | {}", entry.path().display(), metadata.is_dir(), metadata.len());
            }
        }

    }

    let essaie = TemplateContext {
        vecf: v
    };

    Template::render("index", essaie)
}

#[get("/<user_path..>")]
fn user_path(user_path: PathBuf, cfg: State<Config>) -> String {

    let dir = directory::Directory::new();
    println!("{}", dir.name());

    let the_file = dir.get_file(0);
    println!("{}", the_file.name());

    println!("{}", user_path.display());


    let path = Path::new(&cfg.home[..]).join(&user_path);

    if path.exists() && path.is_dir() {
        println!("path found!");
    }

    for entry in path.read_dir().expect("read_dir call failed") {

        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(time) = metadata.modified() {
                    let datetime: DateTime<Local> = time.into();
                    println!("{}", datetime.format("%Y-%m-%d %T"));
                }

                println!("{} => {} | {}", entry.path().display(), metadata.is_dir(), metadata.len());
            }
        }

    }

    format!("cfg_home: {}, user_path: {}, path: {}", cfg.home, user_path.display(), path.display())
}

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

fn main() {
    let cfg = init_cfg_file("/home/spoken/Git/fancyndex/Rust/fancyndex/src/config.toml");

    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![home, user_path])
        .attach(Template::fairing())
        .launch();
}
