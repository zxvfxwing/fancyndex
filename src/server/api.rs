/*
*
* API Module
*
* > "/api/path"
* > "/api/path?<sort: SortQueries>"
* > "/api/path/<upath: UnsafePBuf ..>"
* > "/api/path/<upath: UnsafePBuf ..>?<queries: SortQueries>
*
* Json output
*
*/

use rocket_contrib::Json;
use rocket::State;

use filesystem;
use filesystem::unsafepath::UnsafePBuf;
use filesystem::directory::Directory;
use filesystem::walkdir::WalkDir;

use conf::Config;

use server::{self, SortQueries};

#[get("/path")]
pub fn default_home_path(cfg: State<Config>) -> Json<Directory> {
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit);

    Json(walker.run())
}

#[get("/path?<queries>")]
pub fn home_path(queries: SortQueries, cfg: State<Config>) -> Json<Directory> {
    let queries = server::parse_queries(queries);
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .sort_by(queries.0, queries.1);

    Json(walker.run())
}

#[get("/path/<upath..>")]
pub fn default_path(upath: UnsafePBuf, cfg: State<Config>) -> Json<Directory> {
    let path = filesystem::get_parent_cdir().join(upath.path());
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit);

    Json(walker.run())
}

#[get("/path/<upath..>?<queries>")]
pub fn path(upath: UnsafePBuf, queries: SortQueries, cfg: State<Config>) -> Json<Directory> {
    let queries = server::parse_queries(queries);
    let path = filesystem::get_parent_cdir().join(upath.path());
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .sort_by(queries.0, queries.1);

    Json(walker.run())
}
