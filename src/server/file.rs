use std::path::{Path, PathBuf};
use rocket::State;
use rocket::response::NamedFile;

use filesystem::{self, get_parent_cdir};
use filesystem::unsafepath::UnsafePBuf;

#[get("/<file..>")]
fn static_file(file: UnsafePBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&filesystem::get_parent_cdir()).join(file.path())).ok()
}
