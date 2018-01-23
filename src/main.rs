#![crate_name = "fancyndex"]

#[macro_use]
extern crate serde_derive;
extern crate walkdir;
extern crate toml;

use walkdir::WalkDir;

mod io;
mod config;

use config::Config;

fn main() {
    let cfg = Config::new("Fancyndex.toml").check();
    println!("{:}", cfg.folder.path);
    println!("{:}", cfg.walk_opt.hidden);
    println!("{:}", cfg.walk_opt.symlink);

    

    /*for entry in WalkDir::new(cfg.folder.path) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
    */
}
