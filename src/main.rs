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
//use std::path::PathBuf;
//use std::ffi::OsString;

/* Rocket Web Framework Use */
use rocket_contrib::Template;
//use rocket_contrib::Json;
use rocket::response::Redirect;
//use rocket::response::NamedFile;
//use rocket::State;
//use rocket::http::RawStr;

/* Walkdir */
/*use walkdir::{DirEntry, WalkDir}; */

/* Modules */
mod filesystem;
mod utils;
mod conf;
mod server;

//use filesystem::unsafepath::UnsafePBuf;
//use filesystem::directory::Directory;
//use filesystem::walkdir::WalkDir;
//use conf::Config;

use server::{api, asset, home};

#[get("/")]
fn go_home() -> Redirect {
    Redirect::to("/home")
}

fn main() {
    /* Read config file when starting server */
    let cfg = conf::read_cfg_file("Fancyndex.toml");

    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![go_home])
        .mount("/asset/", routes![asset::file])
        .mount("/home/", routes![home::default_home_path, home::home_path, home::default_path, home::path])
        .mount("/api/", routes![api::default_home_path, api::home_path, api::default_path, api::path])
        .attach(Template::fairing())
        .launch();
}
