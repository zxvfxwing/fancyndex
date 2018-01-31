#![crate_name = "fancyndex"]

#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate walkdir;
extern crate toml;

use std::path::{Path,PathBuf};
use rocket_contrib::Json;

mod io;
mod config;
mod filesystem;
mod walker;

use config::Config;
use walker::Walker;

/// TEST
#[derive(Deserialize)]
pub struct InnerTest {
    what: String,
}

#[derive(Deserialize)]
pub struct Test {
    flag: bool,
    itest: InnerTest,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/*
* curl -H "Content-Type: application/json" -X POST -d '{"flag":true, "itest":{ "what": "truc"}}' http://localhost:8000/test
*/
#[post("/test", format = "application/json", data = "<test>")]
fn test(test: Json<Test>) -> &'static str {
    println!("{}", test.flag);
    println!("{}", &test.itest.what);
    "Test"
}


fn main() {
    let cfg = Config::new("Fancyndex.toml").check();

    println!("{:}", cfg.root.path);
    println!("{:}", cfg.walk_opt.hidden);
    println!("{:}", cfg.walk_opt.symlink);

    let p = Path::new(&cfg.root.path);

    let walker = Walker::new(&p, cfg.walk_opt.hidden, cfg.walk_opt.symlink);
    let rw = walker.run();

    println!("?? {}", rw.len());

    let mut total_el = 0u64;
    let mut total_s = 0u64;

    for e in rw.iter(){
        println!("{}", p.display());

        let w = Walker::new(&e, false, false).deep_run();
        println!("{} --> Size: {}, Nb elements: {}", e.display(), w.0, w.1);

        total_s += w.0;
        total_el += w.1;

    }
    
    let r = walker.deep_run();

    println!("{}", p.display());
    println!("Size: {} {}, Nb elements: {} {}", r.0, total_s, r.1, total_el);
    

    //walker.deep_run();

    /*
    rocket::ignite()
        .mount("/", routes![index, test])
        .launch();
    */
}
