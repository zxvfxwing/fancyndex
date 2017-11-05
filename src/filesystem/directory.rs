use std::path::PathBuf;
use std::process;
//use crossbeam;

use filesystem::file::File;

pub struct Directory {
    path: PathBuf,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {

    fn add_entry(&mut self, entry: &PathBuf) {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_dir()  { self.add_dir(entry);     }
            else
            if metadata.is_file() { self.add_file(entry);    }
            else                  { self.add_symlink(entry); }
        }
    }

    fn add_file(&mut self, entry: &PathBuf) {
        self.files.push(File::new(&entry));
    }

    fn add_dir(&mut self, entry: &PathBuf) {
        self.directories.push(Directory::new(&entry));
    }

    fn add_symlink(&mut self, entry: &PathBuf) {
        if let Ok(link) = entry.read_link() {
            if let Ok(metadata) = link.metadata() {
                if metadata.is_dir() { self.add_dir(&entry);  }
                else                 { self.add_file(&entry); }
            }
        }
    }

    fn run(&mut self) {
        if let Ok(entries) = self.path.read_dir() {
            for entry in entries {
                    if let Ok(entry) = entry {
                        let path_buf = &entry.path();
                        let filename = &super::get_filename(path_buf)[..];
                        /* Not counting dotfiles, filename which begin with a dot */
                        if filename.chars().nth(0).unwrap() != '.' {
                            self.add_entry(path_buf);
                        }
                    }
            }
        }
    }

    pub fn soft_new(p: &PathBuf) -> Directory {
        Directory {
            path: p.to_path_buf(),
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    /*
    *   New instance of Directory struct
    *   All fields are private here, getter needed.
    */
    pub fn new(p: &PathBuf) -> Directory {
        if p.is_file() {
            println!("This PathBuf is a file. You cannot make an instance of struct Directory with it !");
            process::exit(1);
        }

        let mut new_dir = Directory {
            path: p.to_path_buf(),
            directories: Vec::new(),
            files: Vec::new(),
        };

        new_dir.run();
        new_dir
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> String {
        super::get_filename(&self.path)
    }

    pub fn directories(&self) -> &Vec<Directory> {
        &self.directories
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn size(&self) -> u64 {
        let mut size = 0u64;

        for dir in &self.directories {
            size += dir.size();
        }

        for file in &self.files {
            size += file.size();
        }

        size
    }

    pub fn nb_dirs(&self) -> u64 {
        self.directories.len() as u64
    }

    pub fn nb_files(&self) -> u64 {
        self.files.len() as u64
    }

    pub fn nb_elements(&self) -> u64 {
        self.nb_dirs() + self.nb_files()
    }

    pub fn nb_total_elements(&self) -> u64 {
        let mut nb_total_elements = 0u64;

        for dir in &self.directories {
            nb_total_elements += dir.nb_total_elements();
        }

        nb_total_elements += self.nb_elements();
        nb_total_elements
    }

    pub fn datetime(&self) -> String {
        super::get_datetime(&self.path)
    }

    pub fn timestamp(&self) -> i64 {
        super::get_timestamp(&self.path)
    }
}
