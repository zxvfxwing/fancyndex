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
use rocket::response::NamedFile;

#[get("/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("web").join(file)).ok()
}
