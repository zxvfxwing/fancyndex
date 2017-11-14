#![feature(plugin, decl_macro)]
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

use filesystem::directory::Directory;
use filesystem::walkdir::WalkDir;
use conf::Config;





use rocket::http::uri::{URI, Segments, SegmentError};
use rocket::request::FromSegments;
use std::fmt::Debug;

struct UnsafePath {
    path: PathBuf
}

impl UnsafePath {
    pub fn new() -> UnsafePath {
        UnsafePath {
            path: PathBuf::new()
        }
    }

    pub fn push_to_path(&mut self, suffix: &str) {
        self.path.push(suffix);
    }

    pub fn pop_path(&mut self) {
        self.path.pop();
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}

impl<'a> FromSegments<'a> for UnsafePath {
    type Error = SegmentError;

    fn from_segments(segments: Segments<'a>) -> Result<UnsafePath, SegmentError> {
        let mut unsafe_p = UnsafePath::new();

        //let mut buf = PathBuf::new();
        for segment in segments {
            let decoded = URI::percent_decode(segment.as_bytes())
                .map_err(|e| SegmentError::Utf8(e))?;

            if decoded == ".." {

                unsafe_p.pop_path();
                //buf.pop();
            } /*else if decoded.starts_with('.') {
                return Err(SegmentError::BadStart('.'))
            } */else if decoded.starts_with('*') {
                return Err(SegmentError::BadStart('*'))
            } else if decoded.ends_with(':') {
                return Err(SegmentError::BadEnd(':'))
            } else if decoded.ends_with('>') {
                return Err(SegmentError::BadEnd('>'))
            } else if decoded.ends_with('<') {
                return Err(SegmentError::BadEnd('<'))
            } else if decoded.contains('/') {
                return Err(SegmentError::BadChar('/'))
            } else if cfg!(windows) && decoded.contains('\\') {
                return Err(SegmentError::BadChar('\\'))
            } else {
                unsafe_p.push_to_path(&*decoded)
                //buf.push(&*decoded)
            }
        }

        Ok(unsafe_p)
        //Ok(buf)
    }
}




#[get("/api")]
fn home(cfg: State<Config>) -> Json<Directory> {
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .binary_unit(cfg.unit_options.binary_unit);

    Json(walker.run())
}

#[get("/api/<path..>")]
fn path(path: UnsafePath, cfg: State<Config>) -> Json<Directory> {
    let path = filesystem::get_parent_cdir().join(path.get_path());
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .binary_unit(cfg.unit_options.binary_unit);

    Json(walker.run())
}

#[get("/hello/<name..>")]
fn hello(name: PathBuf) -> String {
    format!("Hello, {}!", name.display())
}

#[get("/home")]
fn homee(cfg: State<Config>) -> Template {
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .binary_unit(cfg.unit_options.binary_unit)
        .go_deep(false);

    Template::render("index", walker.run())
}

#[get("/home/<path..>")]
fn pathe(path: PathBuf, cfg: State<Config>) -> Template {
    let path = filesystem::get_parent_cdir().join(path);
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .binary_unit(cfg.unit_options.binary_unit)
        .go_deep(false);

    Template::render("index", walker.run())
}

fn main() {
    /* Read config file when starting server */
    let cfg = conf::init_cfg_file("Fancyndex.toml");

    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![home, path, homee, pathe, hello])
        .attach(Template::fairing())
        .launch();
}
