use std::path::PathBuf;
use walkdir::DirEntry;
use std::ffi::OsStr;

use utils::toggle_prefix;

pub mod scanner;
pub mod unsafepath;

#[derive(Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub bytes: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub path: PathBuf,
    pub bytes: u64,
    pub elements: u64,
    pub directories: Vec<Directory>,
    pub files: Vec<File>
}

impl File {
    pub fn new(p: &PathBuf) -> Self {
        File {
            name: pbuf_fname(p),
            path: p.to_owned(),
            bytes: {
                match p.metadata() {
                    Ok(metadata) => metadata.len(),
                    Err(_) => 0u64,
                }
            },
        }
    }
}

impl Directory {
    pub fn new(p: &PathBuf) -> Self {
        Directory {
            name: pbuf_fname(p),
            path: p.to_owned(),
            bytes: 0u64,
            elements: 0u64,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn push_entry(&mut self, e: &DirEntry) {
        if e.file_type().is_dir() {
            let dir = Directory::new(&e.path().to_path_buf());
            self.bytes += dir.bytes;
            self.directories.push(dir);
        }
        else {
            let file = File::new(&e.path().to_path_buf());
            self.bytes += file.bytes;
            self.files.push(file);
        }
        self.elements += 1;
    }

    pub fn fix_url(mut self, o_prefix: &PathBuf, n_prefix: &PathBuf) -> Self {
        self.path = toggle_prefix(&self.path, o_prefix, n_prefix);
        if self.path.to_str().unwrap() == n_prefix.to_str().unwrap() {
            self.name = String::from("Root");
        }
        
        for d in &mut self.directories {
            d.path = toggle_prefix(&d.path, o_prefix, n_prefix);
        }

        for f in &mut self.files {
            f.path = toggle_prefix(&f.path, o_prefix, n_prefix);
        } 

        self
    }
}

pub fn pbuf_is_hidden(p: &PathBuf) -> bool {
    p.file_name()
    .unwrap_or(OsStr::new(""))
    .to_str()
    .map(|s| s.starts_with("."))
    .unwrap_or(false)
}

pub fn pbuf_is_symlink(p: &PathBuf) -> bool {
    match p.symlink_metadata() {
        Ok(md) => md.file_type().is_symlink(),
        Err(_) => false,
    }
}

pub fn pbuf_fname(p: &PathBuf) -> String {
    p.file_name().unwrap_or(OsStr::new(""))
    .to_str().unwrap()
    .to_string()
}