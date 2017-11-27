use std::path::{Path, PathBuf};
use rocket::State;
use rocket::response::NamedFile;

use rocket::response::{content, Stream};

use filesystem::{self, get_parent_cdir};
use filesystem::unsafepath::UnsafePBuf;

use std::io::{self, repeat, Repeat, Read, Take};
use std::fs::File;

#[get("/<pfile..>")]
fn big_file(pfile: UnsafePBuf) -> io::Result<Stream<File>>  {
    let absolute_path = Path::new(&filesystem::get_parent_cdir()).join(pfile.path());
    File::open( filesystem::get_path_string(&absolute_path) ).map(|file| Stream::from(file))
    /*NamedFile::open(Path::new(&filesystem::get_parent_cdir()).join(file.path())).ok()*/
}
