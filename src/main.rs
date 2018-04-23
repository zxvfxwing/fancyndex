#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
//use rocket_contrib::Json;

#[macro_use] 
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate walkdir;

//use std::path::PathBuf;

pub mod api;
pub mod utils;

pub mod filesystem;
/*use filesystem::{
    scanner::Scanner,
    Directory
};*/

use api::home;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/home", routes![home::index])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}