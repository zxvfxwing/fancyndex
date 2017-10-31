use std::path::PathBuf;
use std::process;

pub struct Directory {
    path: PathBuf,
    directories: Vec<Directory>,
    files: Vec<PathBuf>,
}

impl Directory {

    fn run(&mut self) {
        if let Ok(entries) = self.path.read_dir() {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            self.directories.push(Directory::new(&entry.path()));
                        }
                        else {
                            self.files.push(entry.path());
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

        if p.is_file() { process::exit(1); }

        let mut new_dir = Directory {
            path: p.to_path_buf(),
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

    pub fn files(&self) -> &Vec<PathBuf> {
        &self.files
    }

    pub fn size(&self) -> u64 {
        let mut size = 0u64;

        for dir in &self.directories {
            size += dir.size();
        }

        for file in &self.files {
            size += super::get_size(file);
        }

        size
    }

    pub fn datetime(&self) -> String {
        super::get_datetime(&self.path)
    }
}
