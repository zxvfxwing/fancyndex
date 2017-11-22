use std::path::{Path, PathBuf};
use rocket::State;
use rocket::response::NamedFile;

use filesystem;

#[get("/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&filesystem::get_parent_cdir()).join(file)).ok()
}
