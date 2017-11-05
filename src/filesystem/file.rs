use std::path::PathBuf;
use std::process;

pub struct File {
    path: PathBuf,
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
        }
    }

    pub fn name(&self) -> String {
        super::get_filename(&self.path)
    }

    pub fn size(&self) -> u64 {
        super::get_size(&self.path)
    }

    pub fn datetime(&self) -> String {
        super::get_datetime(&self.path)
    }

    pub fn timestamp(&self) -> i64 {
        super::get_timestamp(&self.path)
    }
}

/* Faire disparaître ce module, le remplacer par un vecteur de PathBuf tout simplement */
