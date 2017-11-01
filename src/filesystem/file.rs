use std::path::PathBuf;
use std::process;

pub struct File {
    path: PathBuf,
    size: u64,
}

impl File {

    pub fn new(p: &PathBuf) -> File {

        if p.is_dir() {
            println!("{}", p.display());
            println!("This PathBuf is a directory. You cannot make an instance of struct File with it !");
            process::exit(1);
        }

        File {
            path: p.to_path_buf(),
            size: super::get_size(p),
        }
    }

    pub fn name(&self) -> String {
        super::get_filename(&self.path)
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn datetime(&self) -> String {
        super::get_datetime(&self.path)
    }

    pub fn timestamp(&self) -> i64 {
        super::get_timestamp(&self.path)
    }
}
