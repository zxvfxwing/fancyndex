use std::path::{Path, PathBuf};

use filesystem::file::File;

pub struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {

    /*fn new(&str) {

    }*/

    /*
    *   Construction d'une nouvelle instance de la structure `Directory`
    *   All fields are private here, getter needed.
    */
    pub fn new() -> Directory {

        let f = File { name: "mouahahah".to_string() };

        Directory {
            name: "try".to_string(),
            directories: Vec::new(),
            files: {
                let mut vecf = Vec::new();
                vecf.push(f);
                vecf
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_dir(&self, index: usize) -> &Directory {
        &self.directories[index]
    }

    pub fn get_file(&self, index: usize) -> &File {
        &self.files[index]
    }

    pub fn dirs(&self) -> &Vec<Directory> {
        &self.directories
    }

    pub fn files(&self) -> &Vec<File> {
        &self.files
    }
}
