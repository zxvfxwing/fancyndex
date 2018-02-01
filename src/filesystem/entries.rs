use walkdir::DirEntry;
use walker::Walker;
use std::path::PathBuf;
use rayon::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub file: bool,
    pub elements: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Entries {
    pub size: u64,
    pub elements: u64,
    pub directories: Vec<Entry>,
    pub files: Vec<Entry>,
}

impl Entry {
    pub fn new(entry: &DirEntry) -> Entry {

        let mut is_file = true;
        if !entry.file_type().is_file() {
            is_file = false;
        }

        Entry {
            path: entry.path().to_path_buf(),
            name: {
                match super::get_file_name(entry) {
                    Ok(name) => name,
                    Err(_) => "".to_string(),
                }                
            },
            size: {
                if is_file { super::get_file_size(entry) }
                else { 0u64 }
            },
            file: is_file,
            elements: 1,
        }
    }

    pub fn is_file(&self) -> bool {
        self.file
    }
}

impl Entries {
    pub fn new() -> Entries {
        Entries{
            size: 0u64,
            elements: 0u64,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    /// Add a new Entry
    /// Vector push depends of Entry type (file or not)
    pub fn push(&mut self, e: Entry) {
        if e.is_file() {
            self.size += e.size;
            self.files.push( e );
        }
        else {
            self.directories.push( e );
        }
        self.elements += 1;
    }

    /// Total elements
    pub fn telts(&self) -> u64 {
        let delts:u64 = self.directories.par_iter()
                                        .map(|dir| dir.elements)
                                        .sum();
        
        delts + self.files.len() as u64
    }

    /// Total size (bytes)
    pub fn tsize(&self) -> u64 {
        let dsize:u64 = self.directories.par_iter()
                                        .map(|dir| dir.size)
                                        .sum();

        let fsize:u64 = self.files.par_iter()
                                  .map(|file| file.size)
                                  .sum();
          
        dsize + fsize
    }

    /// Process `deep_run` for each directory
    pub fn process_deep_run(&mut self, hidden: bool, symlink: bool) {
        self.directories.par_iter_mut()
                        .for_each(|dir|{
                            let walker = Walker::new(&dir.path, hidden, symlink);
                            let (dsize, delts) = walker.deep_run();
                            dir.size = dsize;
                            dir.elements += delts; /* Directory count as one element itself. Initialized to 1 in constructor. */
                        });

        /* Update total size & total elements */
        self.size = self.tsize();
        self.elements = self.telts();
    }
}