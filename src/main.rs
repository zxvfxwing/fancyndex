#![crate_name = "fancyndex"]

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate rayon;
extern crate chrono;
//extern crate walkdir;

/* Crates use */
use rocket_contrib::Template;

/* Fancyndex mod */
mod io;
mod api;
mod config;
//mod walker;
mod filesystem;

use config::Config;
use api::home;
use api::error;

//use filesystem::walkdir::{WalkDir, WalkDirBuilder};
//use std::fs::{self, DirEntry};

fn main() {
    let cfg = Config::new("Fancyndex.toml").check();
    
    /*
    let walkdir = WalkDirBuilder::new(cfg.root.path)
                                    .do_hidden(cfg.walk_opt.hidden)
                                    .do_symlink(cfg.walk_opt.symlink)
                                    .use_entries_opt(cfg.entries_opt)
                                    .build();

    let entries = walkdir.scan();

    if let Ok(entries) = entries {
        for e in entries.dirs() {
            println!("{:?} - {}", e.name(), e.size());
        }

        for e in entries.files() {
            println!("{:?} - {}", e.name(), e.size());
        }
    }
    */

    /*
    if let Ok(entries) = cfg.root.path.read_dir() {
        for possible_entry in entries {
            if let Ok(entry) = possible_entry {



                println!("{:?}", entry.file_name());

            }
        }
    }
    */

    
    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![api::redirect_home])
        .mount("/home", routes![home::index, home::path])
        .mount("/error", routes![error::config_fail])
        .attach(Template::fairing())
        .launch();
    
    
}
