#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

/* Crates */
#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate toml;

/* -- Use -- */
/* STD Lib */
use std::path::PathBuf;
use std::ffi::OsString;

/* Rocket Web Framework Use */
use rocket_contrib::Template;
use rocket_contrib::Json;
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::State;
use rocket::http::RawStr;

/* Walkdir */
/*use walkdir::{DirEntry, WalkDir}; */

/* Modules */
mod filesystem;
mod utils;
mod conf;
mod server;

use filesystem::unsafepath::UnsafePBuf;
use filesystem::directory::Directory;
use filesystem::walkdir::WalkDir;
use conf::Config;

use server::api;

#[get("/hello/<name..>")]
fn hello(name: PathBuf) -> String {
    format!("Hello, {}!", name.display())
}

#[get("/home")]
fn home(cfg: State<Config>) -> Template {
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .go_deep(false);

    Template::render("index", walker.run())
}

#[get("/home/<path..>")]
fn path(path: PathBuf, cfg: State<Config>) -> Template {
    let path = filesystem::get_parent_cdir().join(path);
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .go_deep(false);

    Template::render("index", walker.run())
}

fn main() {
    /* Read config file when starting server */
    let cfg = conf::init_cfg_file("Fancyndex.toml");

    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![home, path, hello])
        .mount("/api/", routes![api::default_home_path, api::home_path, api::default_path, api::path])
        .attach(Template::fairing())
        .launch();
}
