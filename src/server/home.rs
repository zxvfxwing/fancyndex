/*
*
* Home module
*
* > "/home"
* > "/home?<SortQueries>"
* > "/home/<upath: UnsafePBuf ..>"
* > "/home/<upath: UnsafePBuf ..>?<queries: SortQueries>"
*
* Handlebars template ouput
*
*/

use rocket_contrib::Template;
use rocket::State;

use filesystem;
use filesystem::unsafepath::UnsafePBuf;
use filesystem::walkdir::WalkDir;

use conf::Config;

use server::{self, SortQueries};

#[get("/")]
fn default_home_path(cfg: State<Config>) -> Template {
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .go_deep(false);

    Template::render("index", walker.run())
}

#[get("/?<queries>")]
fn home_path(queries: SortQueries, cfg: State<Config>) -> Template {
    let queries = server::parse_queries(queries);
    let path = filesystem::get_parent_cdir();
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .sort_by(queries.0, queries.1)
        .go_deep(false);

    Template::render("index", walker.run())
}

#[get("/<upath..>")]
fn default_path(upath: UnsafePBuf, cfg: State<Config>) -> Template {
    let path = filesystem::get_parent_cdir().join(upath.path());
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .go_deep(false);

    Template::render("index", walker.run())
}

#[get("/<upath..>?<queries>")]
fn path(upath: UnsafePBuf, queries: SortQueries, cfg: State<Config>) -> Template {
    let queries = server::parse_queries(queries);
    let path = filesystem::get_parent_cdir().join(upath.path());
    let walker = WalkDir::init(&path)
        .do_hidden(cfg.walk_options.do_hidden)
        .do_symlink(cfg.walk_options.do_symlink)
        .use_binary_unit(cfg.unit_options.binary_unit)
        .sort_by(queries.0, queries.1)
        .go_deep(false);

    Template::render("index", walker.run())
}
