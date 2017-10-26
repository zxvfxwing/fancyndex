#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use rocket::State;
use rocket_contrib::Template;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::process;

use std::collections::HashMap;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    vecf: Vec<TemplateContext>
}

#[get("/")]
fn home(cfg: State<Config>) -> Template {
    let path = Path::new(&cfg.home[..]);

    if path.exists() && path.is_dir() {
        println!("path found!");
    }

    println!("cfg_home: {}, path: {}", cfg.home, path.display());

    let mut context = HashMap::new();
    context.insert("home", &cfg.home);

    let v: Vec<TemplateContext> = Vec::new();

    let essaie = TemplateContext {
        name: String::from("essaie"),
        vecf: v,
    };

    let mut tpml = Template::render("index", context);
    tpml = Template::render("index", essaie);
    tpml
    //Template::render("index", format!("{home}", home = cfg.home))
}

#[get("/<user_path..>")]
fn user_path(user_path: PathBuf, cfg: State<Config>) -> String {

    let path = Path::new(&cfg.home[..]).join(&user_path);

    if path.exists() && path.is_dir() {
        println!("path found!");
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
