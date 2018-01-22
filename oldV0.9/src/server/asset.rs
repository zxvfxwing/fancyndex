/*
*
* Asset module
*
* > "/asset/<path..>"
*
* Cached static file output
* Static file output
*
*/

use std::path::{Path, PathBuf};
use rocket::State;
use rocket_file_cache::{Cache, CachedFile};
//use rocket::response::NamedFile;

/*
USE THIS FUNCTION IN PRODUCTION, CACHE ASSET FILES
*/

#[get("/<file..>")]
fn file(file: PathBuf, cache: State<Cache>) -> Option<CachedFile> {
    CachedFile::open(Path::new("web").join(file), cache.inner())
}

/*
#[get("/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("web").join(file)).ok()
}
*/
