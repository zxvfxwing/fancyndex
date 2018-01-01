use std::path::{Path, PathBuf};
use std::io;
use rocket::response::NamedFile;

use filesystem::{self, get_parent_cdir};
use filesystem::unsafepath::UnsafePBuf;

#[get("/<pfile..>")]
fn big_file(pfile: UnsafePBuf) -> Option<NamedFile> {
    let absolute_path = Path::new(&filesystem::get_parent_cdir()).join(pfile.path());
    NamedFile::open(absolute_path).ok()
}
