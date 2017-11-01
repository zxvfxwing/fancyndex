use std::path::PathBuf;
use std::process;

use filesystem::file::File;

pub struct Directory {
    path: PathBuf,
    size: u64,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {

    fn run(&mut self) {
        if let Ok(entries) = self.path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {

                        if metadata.is_dir() {
                            let new_dir = Directory::new(&entry.path());
                            self.size += new_dir.size();
                            self.directories.push(new_dir);
                        }

                        else if metadata.is_file() {
                            let new_file = File::new(&entry.path());
                            self.size += new_file.size();
                            self.files.push(new_file);
                        }

                        /* If entry isn't a directory & file -> symbolic link, so read it */
                        else {
                            if let Ok(link) = entry.path().read_link() {
                                if let Ok(metadata) = link.metadata() {
                                    if metadata.is_dir() {
                                        let new_dir = Directory::new(&entry.path());
                                        self.size += new_dir.size();
                                        self.directories.push(new_dir);
                                    }
                                    else {
                                        let new_file = File::new(&entry.path());
                                        self.size += new_file.size();
                                        self.files.push(new_file);
                                    }
                                }
                            }
                        }

                    }
                }
            }
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
            size: 0u64,
            directories: Vec::new(),
            files: Vec::new(),
        };

        new_dir.run();
        new_dir
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
        self.size
    }

    pub fn datetime(&self) -> String {
        super::get_datetime(&self.path)
    }

    pub fn timestamp(&self) -> i64 {
        super::get_timestamp(&self.path)
    }
}
