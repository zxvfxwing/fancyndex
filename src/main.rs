#![crate_name = "fancyndex"]

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate rayon;
extern crate walkdir;

/* Crates use */
use rocket_contrib::Template;

/* Fancyndex mod */
mod io;
mod api;
mod config;
mod walker;
mod filesystem;

use config::Config;
use api::home;

fn main() {
    let cfg = Config::new("Fancyndex.toml").check();
    
    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![api::redirect_home])
        .mount("/home", routes![home::index, home::path])
        .attach(Template::fairing())
        .launch();
}
