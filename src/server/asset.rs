/*
*
* Asset module
*
* > "/asset/<path..>"
*
* Static file ouput
*
*/

use std::path::{Path, PathBuf};
use rocket::State;
use rocket_file_cache::{Cache, CachedFile};

#[get("/<file..>")]
fn file(file: PathBuf, cache: State<Cache>) -> Option<CachedFile> {
    CachedFile::open(Path::new("web").join(file), cache.inner())
}
