#![crate_name = "fancyndex"]

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate walkdir;
extern crate toml;
extern crate rayon;

use std::path::Path;
use rocket_contrib::Json;
use rocket::State;

mod io;
mod config;
mod filesystem;
mod walker;

use config::Config;
use walker::Walker;
use filesystem::entries::*;

/*
* curl -H "Content-Type: application/json" -X POST -d '{"size":0,"elements":0,"directories":[{"path":"/home/spoken/Git/dotconfig","name":"dotconfig","size":0,"file":false,"elements":1},{"path":"/home/spoken/Git/M1","name":"M1","size":0,"file":false,"elements":1},{"path":"/home/spoken/Git/fancyndex","name":"fancyndex","size":0,"file":false,"elements":1}],"files":[]}' http://localhost:8000/test
*/

#[get("/")]
fn index() -> Json<Entries> {
    let p = filesystem::cdir();
    let walker = Walker::new(&p, false, false);
    Json(walker.run())
}

#[post("/test", format = "application/json", data = "<entries>")]
fn test(mut entries: Json<Entries>, cfg: State<Config>) -> Json<Entries> {
    entries.process_deep_run(cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    entries
}

fn main() {
    let cfg = Config::new("Fancyndex.toml").check();

    println!("{:}", cfg.root.path);
    println!("{:}", cfg.walk_opt.hidden);
    println!("{:}", cfg.walk_opt.symlink);

    /*
    let p = Path::new(&cfg.root.path);

    let walker = Walker::new(&p, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    let mut entries = walker.run();

    entries.process_deep_run(cfg.walk_opt.hidden, cfg.walk_opt.symlink);

    println!("{} {}", entries.tsize(), entries.telts() );
    */
    
    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![index, test])
        .launch();
}
